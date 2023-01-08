use crossterm::cursor::{Hide, MoveTo};
use crossterm::style::Print;
use crossterm::terminal::{Clear, ClearType};
use crossterm::QueueableCommand;
use std::collections::HashSet;
use std::io::{Stdout, Write};

#[allow(unused)]
pub struct Maze {
    rows: usize,
    columns: usize,
    edges: HashSet<((usize, usize), (usize, usize))>,
}

impl Maze {
    pub fn new(rows: usize, columns: usize) -> Maze {
        Maze {
            rows,
            columns,
            edges: HashSet::new(),
        }
    }

    pub fn insert_edge(&mut self, node_1: (usize, usize), node_2: (usize, usize)) {
        self.edges.insert((node_1, node_2));
        self.edges.insert((node_2, node_1));
    }
}

/// Initializes walled maze in the terminal.
pub fn init_maze(stdout: &mut Stdout, rows: usize, columns: usize) {
    let mut buffer = String::new();

    // Write top row to the buffer.
    buffer.push('_');
    for _ in 0..columns {
        buffer.push_str("__");
    }
    buffer.push('\n');

    // Write the of the rows to the buffer.
    for _ in 0..rows {
        buffer.push('│');
        for _ in 0..columns {
            buffer.push_str("_│");
        }
        buffer.push('\n');
    }

    // Setup terminal for drawing the maze.
    stdout.queue(Hide).unwrap();
    stdout.queue(MoveTo(0, 0)).unwrap();
    stdout.queue(Clear(ClearType::All)).unwrap();

    // Queue the lines to be drawn to the terminal.
    for line in buffer.lines() {
        stdout.queue(Clear(ClearType::UntilNewLine)).unwrap();
        stdout.queue(Print(line)).unwrap();
        stdout.queue(Print('\n')).unwrap();
    }

    // Flush to make sure the maze is drawn.
    stdout.flush().unwrap();
}
