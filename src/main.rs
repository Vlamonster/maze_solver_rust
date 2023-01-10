mod generator;
mod maze;
mod solver;

use clap::Parser;
use crossterm::cursor::Show;
use crossterm::ExecutableCommand;
use std::io::stdout;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Number of rows to draw.
    rows: usize,

    /// Number of columns to draw.
    columns: usize,

    /// Generator used.
    #[arg(short, long, default_value = "depth_first_search", value_parser = ["depth_first_search", "breadth_first_search", "kruskal"])]
    generator: String,

    /// Solver used. If one is selected, then the generator will run with a delay of 0.
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

    if args.solver.is_none() {
        match (args.generator.as_str(), args.delay) {
            ("depth_first_search", 0) => {
                generator::depth_first_search::generate_instant(
                    &mut stdout,
                    args.rows,
                    args.columns,
                );
            }
            ("depth_first_search", delay) => {
                generator::depth_first_search::generate(
                    &mut stdout,
                    args.rows,
                    args.columns,
                    delay,
                );
            }
            ("breadth_first_search", 0) => {
                generator::breadth_first_search::generate_instant(
                    &mut stdout,
                    args.rows,
                    args.columns,
                );
            }
            ("breadth_first_search", delay) => {
                generator::breadth_first_search::generate(
                    &mut stdout,
                    args.rows,
                    args.columns,
                    delay,
                );
            }
            ("kruskal", 0) => {
                generator::kruskal::generate_instant(&mut stdout, args.rows, args.columns);
            }
            ("kruskal", delay) => {
                generator::kruskal::generate(&mut stdout, args.rows, args.columns, delay);
            }
            _ => unreachable!(),
        }
    } else {
        let mut maze = match args.generator.as_str() {
            "depth_first_search" => generator::depth_first_search::generate_instant(
                &mut stdout,
                args.rows,
                args.columns,
            ),
            "breadth_first_search" => generator::breadth_first_search::generate_instant(
                &mut stdout,
                args.rows,
                args.columns,
            ),
            "kruskal" => generator::kruskal::generate_instant(&mut stdout, args.rows, args.columns),
            _ => unreachable!(),
        };

        solver::depth_first_search::solve(&mut stdout, &mut maze, args.delay, args.trace);
    }

    stdout.execute(Show).unwrap();
}
