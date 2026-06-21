use super::{ProjectType, Projects};
use colored::Colorize;
use std::{
    io::{self, Write},
    path::PathBuf,
};
use terminal_size::{Width, terminal_size};

pub fn print_groups(stdout: &mut impl Write, projects: Projects) -> io::Result<()> {
    let mut groups: Vec<(&ProjectType, &Vec<PathBuf>)> = projects.by_type.iter().collect();
    groups.sort_by_key(|(pt, _)| format!("{pt:?}"));

    for (project_type, dirs) in groups {
        writeln!(
            stdout,
            "\n{}",
            format!("{project_type:?}").bold().underline()
        )?;
        let names: Vec<&str> = dirs
            .iter()
            .filter_map(|p| p.file_name()?.to_str())
            .collect();
        print_names(stdout, &names)?;
    }

    if !projects.unknown.is_empty() {
        writeln!(stdout, "\n{}", "Unknown".bold().underline())?;
        let names: Vec<&str> = projects
            .unknown
            .iter()
            .filter_map(|p| p.file_name()?.to_str())
            .collect();
        print_names(stdout, &names)?;
    }

    Ok(())
}

fn print_names(stdout: &mut impl Write, names: &[&str]) -> io::Result<()> {
    let mut sorted = names.to_vec();
    sorted.sort();

    let term_width = terminal_size()
        .map(|(Width(w), _)| w as usize)
        .unwrap_or(80);

    // Try decreasing column counts until everything fits
    let cols = (1..=sorted.len())
        .rev()
        .find(|&cols| {
            let rows = sorted.len().div_ceil(cols);
            let col_widths: Vec<usize> = (0..cols)
                .map(|col| {
                    sorted
                        .iter()
                        .skip(col * rows)
                        .take(rows)
                        .map(|n| n.len())
                        .max()
                        .unwrap_or(0)
                        + 2
                })
                .collect();
            col_widths.iter().sum::<usize>() <= term_width
        })
        .unwrap_or(1);

    let rows = sorted.len().div_ceil(cols);

    let col_widths: Vec<usize> = (0..cols)
        .map(|col| {
            sorted
                .iter()
                .skip(col * rows)
                .take(rows)
                .map(|n| n.len())
                .max()
                .unwrap_or(0)
                + 2
        })
        .collect();

    for row in 0..rows {
        let line = (0..cols)
            .filter_map(|col| {
                let idx = col * rows + row;
                sorted
                    .get(idx)
                    .map(|name| format!("{name:<width$}", width = col_widths[col]))
            })
            .collect::<Vec<_>>()
            .join("");
        writeln!(stdout, "{line}")?;
    }

    Ok(())
}
