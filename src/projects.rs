use clap::ValueEnum;
use ignore::WalkBuilder;
use std::{collections::HashMap, fs, io, path::PathBuf};

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, ValueEnum)]
pub enum ProjectType {
    Julia,
    Rust,
    Python,
    Go,
}

pub struct Projects {
    pub by_type: HashMap<ProjectType, Vec<PathBuf>>,
    pub unknown: Vec<PathBuf>,
}

impl Projects {
    pub fn collect(dir: &PathBuf, filter: Option<ProjectType>) -> io::Result<Self> {
        let mut by_type: HashMap<ProjectType, Vec<PathBuf>> = HashMap::new();
        let mut unknown: Vec<PathBuf> = Vec::new();

        let walk = WalkBuilder::new(dir)
            .max_depth(Some(1))
            // Filter only for subdirectories
            .filter_entry(|e| e.file_type().map(|ft| ft.is_dir()).unwrap_or(false))
            .build();

        for entry in walk
            .into_iter()
            .filter_map(|e| e.ok())
            // Filter out the root directory
            .filter(|e| e.depth() > 0)
        {
            let path = entry.into_path();
            let contents = fs::read_dir(&path)?
                .filter_map(|e| e.ok())
                .map(|e| e.path())
                .collect::<Vec<_>>();

            match classify_project(&contents) {
                Some(pt) => {
                    if filter.is_none_or(|f| f == pt) {
                        by_type.entry(pt).or_default().push(path);
                    }
                }
                None => {
                    if filter.is_none() {
                        unknown.push(path);
                    }
                }
            }
        }

        Ok(Self { by_type, unknown })
    }
}

fn classify_project(contents: &[PathBuf]) -> Option<ProjectType> {
    let filenames: Vec<&str> = contents
        .iter()
        .filter_map(|p| p.file_name()?.to_str())
        .collect();

    if filenames.contains(&"Cargo.toml") {
        Some(ProjectType::Rust)
    } else if filenames.contains(&"Project.toml") {
        Some(ProjectType::Julia)
    } else if filenames.contains(&"go.mod") {
        Some(ProjectType::Go)
    } else if filenames
        .iter()
        .any(|f| matches!(*f, "pyproject.toml" | "requirements.txt"))
    {
        Some(ProjectType::Python)
    } else {
        None
    }
}
