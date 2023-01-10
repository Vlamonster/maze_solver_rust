mod generator;
mod maze;

use crate::generator::{breadth_first_search, depth_first_search, kruskal};
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

    /// Number of milliseconds between animation.
    #[arg(short, long, default_value_t = 25)]
    delay: u64,
}

fn main() {
    let args = Args::parse();
    let mut stdout = stdout();

    match (args.generator.as_str(), args.delay) {
        ("depth_first_search", 0) => {
            depth_first_search::generate_instant(&mut stdout, args.rows, args.columns);
        }
        ("depth_first_search", delay) => {
            depth_first_search::generate(&mut stdout, args.rows, args.columns, delay);
        }
        ("breadth_first_search", 0) => {
            breadth_first_search::generate_instant(&mut stdout, args.rows, args.columns);
        }
        ("breadth_first_search", delay) => {
            breadth_first_search::generate(&mut stdout, args.rows, args.columns, delay);
        }
        ("kruskal", 0) => {
            kruskal::generate_instant(&mut stdout, args.rows, args.columns);
        }
        ("kruskal", delay) => {
            kruskal::generate(&mut stdout, args.rows, args.columns, delay);
        }
        _ => unreachable!(),
    }

    stdout.execute(Show).unwrap();
}
