mod display;
mod projects;

use clap::{ArgAction, Parser, ValueEnum, crate_authors, crate_name, crate_version, value_parser};
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
    #[arg(
        action = ArgAction::Set,
        num_args = 0..=1,
        value_name = "projects directory",
        value_parser = value_parser!(String),
    )]
    dir: Option<String>,

    /// Sort order for projects within each group
    #[arg(long, short, value_enum, default_value_t = SortOrder::Name)]
    sort: SortOrder,
}

#[derive(ValueEnum, Clone, Default)]
enum SortOrder {
    #[default]
    Name,
    Modified,
}

// TODO: option to pull info from github?
// TODO: option to change depth
// TODO: list line by line (-1)
// TODO: add additional checking that (for example) rust projects have a src directory

fn main() -> io::Result<()> {
    let cli = Cli::parse();
    let dir = resolve_dir(cli.dir);

    let projects = Projects::collect(&dir)?;

    let mut stdout = io::stdout();
    print_groups(&mut stdout, projects, &cli.sort)?;
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
