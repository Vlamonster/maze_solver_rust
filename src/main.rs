mod generator;
mod maze;
mod solver;

use crate::maze::Maze;
use anyhow::Result;
use clap::{ArgGroup, Parser};
use crossterm::cursor::Show;
use crossterm::ExecutableCommand;
use std::io::stdout;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(group(ArgGroup::new("maze_used").required(true).args(["generator", "input"])))]
struct Args {
    /// Number of rows to draw.
    #[arg(default_value_t = 16)]
    rows: u16,

    /// Number of columns to draw.
    #[arg(default_value_t = 48)]
    columns: u16,

    /// Generator used.
    #[arg(short, long, value_parser = ["depth_first_search", "breadth_first_search", "kruskal"])]
    generator: Option<String>,

    /// Input path used.
    #[arg(short, long)]
    input: Option<String>,

    /// Solver used. If Some, then the generator will run with a delay of 0.
    #[arg(short, long, value_parser = ["depth_first_search"])]
    solver: Option<String>,

    /// Flag to enable drawing visited cells.
    #[arg(short, long)]
    trace: bool,

    /// Number of milliseconds between animation.
    #[arg(short, long, default_value_t = 25)]
    delay: u64,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let mut stdout = stdout();

    let delay = if args.solver.is_some() { 0 } else { args.delay };

    let mut maze = match (args.input.as_deref(), args.generator.as_deref()) {
        (Some(path), _) => {
            let maze = Maze::from_path(PathBuf::from(path))?;
            maze.print(&mut stdout)?;
            maze
        }
        (_, Some("depth_first_search")) => {
            generator::depth_first_search::generate(&mut stdout, args.rows, args.columns, delay)?
        }
        (_, Some("breadth_first_search")) => {
            generator::breadth_first_search::generate(&mut stdout, args.rows, args.columns, delay)?
        }
        (_, Some("kruskal")) => {
            generator::kruskal::generate(&mut stdout, args.rows, args.columns, delay)?
        }
        _ => unreachable!(),
    };

    #[allow(clippy::single_match)]
    match args.solver.as_deref() {
        Some("depth_first_search") => {
            solver::depth_first_search::solve(&mut stdout, &mut maze, args.delay, args.trace)?;
        }
        _ => {}
    };

    stdout.execute(Show)?;

    Ok(())
}
