mod generate;
mod maze;

use crate::generate::generate_maze;
use clap::Parser;
use std::io::stdout;
use crossterm::cursor::Show;
use crossterm::ExecutableCommand;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Number of rows to draw.
    rows: usize,

    /// Number of columns to draw.
    columns: usize,

    /// Number of milliseconds between animation.
    #[arg(short, long, default_value_t = 25)]
    delay: u64,
}

fn main() {
    let args = Args::parse();
    let mut stdout = stdout();
    let _maze = generate_maze(&mut stdout, args.rows, args.columns, args.delay);
    stdout.execute(Show).unwrap();
}
