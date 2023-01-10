mod generator;
mod maze;
mod solver;

use crate::maze::Maze;
use clap::Parser;
use crossterm::cursor::Show;
use crossterm::ExecutableCommand;
use std::io::stdout;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Number of rows to draw.
    #[arg(default_value_t = 16)]
    rows: usize,

    /// Number of columns to draw.
    #[arg(default_value_t = 48)]
    columns: usize,

    /// Generator used.
    #[arg(short, long, default_value = "depth_first_search", value_parser = ["depth_first_search", "breadth_first_search", "kruskal"])]
    generator: String,

    /// Input path used. If None, then the generator will be used.
    #[arg(short, long)]
    input: Option<String>,

    /// Solver used. If Some, then the generator will run with a delay of 0.
    #[arg(short, long, value_parser = ["depth_first_search"])]
    solver: Option<String>,

    /// Flag to enable drawing visited cells.
    #[arg(short, long, default_value_t = false)]
    trace: bool,

    /// Number of milliseconds between animation.
    #[arg(short, long, default_value_t = 25)]
    delay: u64,
}

fn main() {
    let args = Args::parse();
    let mut stdout = stdout();

    let delay = if args.solver.is_some() { 0 } else { args.delay };
    let mut maze = match (args.input, args.generator.as_str(), delay) {
        (Some(path), _, _) => {
            let maze = Maze::from_path(PathBuf::from(path));
            maze.print(&mut stdout);
            maze
        }
        (_, "depth_first_search", 0) => {
            generator::depth_first_search::generate_instant(&mut stdout, args.rows, args.columns)
        }
        (_, "depth_first_search", _) => {
            generator::depth_first_search::generate(&mut stdout, args.rows, args.columns, delay)
        }
        (_, "breadth_first_search", 0) => {
            generator::breadth_first_search::generate_instant(&mut stdout, args.rows, args.columns)
        }
        (_, "breadth_first_search", _) => {
            generator::breadth_first_search::generate(&mut stdout, args.rows, args.columns, delay)
        }
        (_, "kruskal", 0) => {
            generator::kruskal::generate_instant(&mut stdout, args.rows, args.columns)
        }
        (_, "kruskal", _) => {
            generator::kruskal::generate(&mut stdout, args.rows, args.columns, delay)
        }
        _ => unreachable!(),
    };

    if let Some(solver) = args.solver {
        match solver.as_str() {
            "depth_first_search" => {
                solver::depth_first_search::solve(&mut stdout, &mut maze, args.delay, args.trace)
            }
            _ => unreachable!(),
        }
    }

    stdout.execute(Show).unwrap();
}
