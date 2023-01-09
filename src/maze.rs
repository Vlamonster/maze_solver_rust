use crossterm::cursor::{Hide, MoveTo};
use crossterm::style::Attribute::{NoUnderline, Underlined};
use crossterm::style::Print;
use crossterm::terminal::{Clear, ClearType};
use crossterm::{ExecutableCommand, QueueableCommand};
use std::collections::HashSet;
use std::io::{Stdout, Write};

// Walls come in three types:
// Horizontal (H), which is represented by an underlined space.
// Vertical (V), which is represented by a vertical line.
// None (N), which is represented by a space.
#[derive(Copy, Clone, Debug)]
pub enum Wall {
    Horizontal,
    Vertical,
    None,
}

// The Maze struct stores the frame as a matrix of Walls.
// For example the following 3 by 3 maze
// _ _____
// │____ │
// │ _ │ │
// │_│__ │
//
// is represented by
// [[H, N, H, H, H, H, H],
//  [V, H, H, H, H, N, V],
//  [V, N, H, N, V, N, V],
//  [V, H, V, H, H, N, V]]
#[allow(unused)]
pub struct Maze {
    rows: usize,
    columns: usize,
    frame: Vec<Vec<Wall>>,
    edges: HashSet<((usize, usize), (usize, usize))>,
}

impl Maze {
    pub fn new(rows: usize, columns: usize, frame: Vec<Vec<Wall>>) -> Maze {
        Maze {
            rows,
            columns,
            frame,
            edges: HashSet::new(),
        }
    }

    pub fn insert_edge(&mut self, node_1: (usize, usize), node_2: (usize, usize)) {
        self.edges.insert((node_1, node_2));
        self.edges.insert((node_2, node_1));
    }

    pub fn get_cell(&self, column: usize, row: usize) -> Wall {
        self.frame[row][column]
    }

    pub fn set_cell(&mut self, column: usize, row: usize, cell: Wall) {
        self.frame[row][column] = cell;
    }
}

/// Initializes walled maze in the terminal.
pub fn init_maze(stdout: &mut Stdout, rows: usize, columns: usize) -> Maze {
    let mut buffer = Vec::new();

    let top_row = vec![Wall::Horizontal; columns * 2 + 1];

    let mut row = vec![Wall::Vertical];
    for _ in 0..columns {
        row.push(Wall::Horizontal);
        row.push(Wall::Vertical);
    }

    // Create frame.
    buffer.push(top_row);
    for _ in 0..rows {
        buffer.push(row.clone());
    }

    // Create openings.
    buffer[0][1] = Wall::None;
    buffer[rows][columns * 2 - 1] = Wall::None; // (rows, columns * 2 - 1) is lower right cell

    // Setup terminal for drawing the maze.
    stdout.queue(Hide).unwrap();
    stdout.queue(MoveTo(0, 0)).unwrap();
    stdout.queue(Clear(ClearType::All)).unwrap();

    // Draw frame.
    for line in &buffer {
        stdout.queue(Clear(ClearType::UntilNewLine)).unwrap();
        for &cell in line {
            print_cell(stdout, cell, ' ');
        }
        stdout.queue(Print('\n')).unwrap();
    }

    // Flush to make sure the maze is drawn.
    stdout.flush().unwrap();

    Maze::new(rows, columns, buffer)
}

/// Print cell type with given character at current cursor position.
pub fn print_cell(stdout: &mut Stdout, cell: Wall, char: char) {
    match cell {
        Wall::Horizontal => stdout
            .execute(Print(format!("{}{}{}", Underlined, char, NoUnderline)))
            .unwrap(),
        Wall::Vertical => stdout.execute(Print('│')).unwrap(),
        Wall::None => stdout.execute(Print(char)).unwrap(),
    };
}
