mod display;
mod projects;

use clap::{Parser, ValueEnum, crate_authors, crate_name, crate_version};
use display::print_groups;
use projects::{ProjectType, Projects};
use std::{
    env,
    io::{self, Write},
    path::PathBuf,
};

#[derive(Parser)]
#[command(
    name = crate_name!(),
    author = crate_authors!(", "),
    version = crate_version!(),
)]
/// Summary of projects by project type
///
/// Recurse files from a directory, and list them in groups based on project type (e.g., Python, Julia, Rust, &c.).
struct Cli {
    /// The starting directory of your projects directory (optional)
    ///
    /// By default, the starting directory is ~/projects/, and falls back to current directory
    #[arg(value_name = "PROJECTS DIRECTORY")]
    dir: Option<String>,

    /// Sort order for projects within each group
    #[arg(long, short, value_enum, default_value_t = SortOrder::Name)]
    sort: SortOrder,

    /// Filter by project type
    #[arg(short = 't', long = "type", value_enum, value_name = "PROJECT TYPE")]
    filter: Option<ProjectType>,
}

#[derive(ValueEnum, Clone, Copy, Default)]
enum SortOrder {
    #[default]
    Name,
    Modified,
}

// TODO: option to pull info from github?
// TODO: option to change depth
// TODO: list line by line (-1)
// TODO: add additional checking that (for example) rust projects have a src directory
// TODO: allow type filter to take multiple values??
// TODO: it would be great if I could pipe the output to rg or fd so that I could find something within those directories if I could filter by type
// TODO: go through common defaults in order or precedence, like ~/projects, ~/Programming, etc.?

fn main() -> io::Result<()> {
    let cli = Cli::parse();
    let dir = resolve_dir(cli.dir);

    let projects = Projects::collect(&dir, cli.filter)?;

    let mut stdout = io::stdout();
    print_groups(&mut stdout, projects, cli.sort)?;
    stdout.flush()?;

    Ok(())
}

fn resolve_dir(dir: Option<String>) -> PathBuf {
    match dir {
        Some(path) => PathBuf::from(path),
        None => env::home_dir()
            .map(|p| p.join("projects"))
            .or_else(|| env::current_dir().ok())
            .unwrap_or_else(|| PathBuf::from(".")),
    }
}
