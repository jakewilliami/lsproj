mod display;
mod projects;

use clap::{Parser, ValueEnum, crate_authors, crate_name, crate_version};
use display::{DisplayOpts, print_groups};
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

    /// Filter by project type(s)
    #[arg(short = 't', long = "type", value_enum, value_name = "PROJECT TYPE")]
    filter: Vec<ProjectType>,

    /// Display one entry per line
    #[arg(short = '1')]
    one_per_line: bool,
}

#[derive(ValueEnum, Clone, Copy, Default)]
enum SortOrder {
    #[default]
    Name,
    Modified,
}

// TODO: option to pull info from github?
// TODO: option to change depth
// TODO: add additional checking that (for example) rust projects have a src directory
// TODO: it would be great if I could pipe the output to rg or fd so that I could find something within those directories if I could filter by type
// TODO: go through common defaults in order or precedence, like ~/projects, ~/Programming, etc.?
// TODO: sort project grouping order too?
// TODO: warn rather than error in collect if something can't be read?

fn main() -> io::Result<()> {
    let cli = Cli::parse();
    let dir = resolve_dir(cli.dir);

    let projects = Projects::collect(&dir, cli.filter)?;

    let display_opts = DisplayOpts {
        sort: cli.sort,
        one_per_line: cli.one_per_line,
    };

    let mut stdout = io::stdout();
    print_groups(&mut stdout, projects, &display_opts)?;
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
