use crossterm::cursor::{Hide, MoveTo};
use crossterm::style::Attribute::{NoUnderline, Underlined};
use crossterm::style::Print;
use crossterm::terminal::{Clear, ClearType};
use crossterm::{ExecutableCommand, QueueableCommand};
use std::io::{Stdout, Write};

/// Walls come in three types:
/// * Horizontal (H)
/// * Vertical (V)
/// * None (N)
///
/// Horizontal and None walls can hold a character to display inside the cell.
#[derive(Copy, Clone, Debug)]
pub enum Wall {
    Horizontal(char),
    Vertical,
    None(char),
}

impl Wall {
    /// Prints wall at current cursor position.
    pub fn print(self, stdout: &mut Stdout) {
        match self {
            Wall::Horizontal(char) => stdout
                .execute(Print(format!("{Underlined}{char}{NoUnderline}")))
                .unwrap(),
            Wall::Vertical => stdout.execute(Print('│')).unwrap(),
            Wall::None(char) => stdout.execute(Print(char)).unwrap(),
        };
    }

    /// Prints wall at current cursor position with the given character. Panics if Wall is Vertical.
    pub fn print_with_char(self, stdout: &mut Stdout, char: char) {
        match self {
            Wall::Horizontal(_) => stdout
                .execute(Print(format!("{Underlined}{char}{NoUnderline}")))
                .unwrap(),
            Wall::None(_) => stdout.execute(Print(char)).unwrap(),
            Wall::Vertical => unreachable!(),
        };
    }
}

/// The Maze struct stores the frame as a matrix of Walls. Example 3x3 matrix and its frame:
/// ```text
/// _ _____ [[H, N, H, H, H, H, H],
/// │____ │  [V, H, H, H, H, N, V],
/// │ _ │ │  [V, N, H, N, V, N, V],
/// │_│__ │  [V, H, V, H, H, N, V]]
/// ```
#[allow(unused)]
pub struct Maze {
    rows: usize,
    columns: usize,
    frame: Vec<Vec<Wall>>,
}

impl Maze {
    /// Returns maze initialized with walls around every cell.
    pub fn new_walled(rows: usize, columns: usize) -> Maze {
        Maze {
            rows,
            columns,
            frame: walled_maze(rows, columns),
        }
    }

    /// Clears the terminal and prints the frame of the maze to the terminal.
    pub fn print(&self, stdout: &mut Stdout) {
        stdout.queue(Hide).unwrap();
        stdout.queue(MoveTo(0, 0)).unwrap();
        stdout.queue(Clear(ClearType::All)).unwrap();

        for row in &self.frame {
            for &wall in row {
                wall.print(stdout);
            }
            stdout.queue(Print('\n')).unwrap();
        }

        // Flush to make sure the frame is drawn.
        stdout.flush().unwrap();
    }

    /// Returns wall from frame coordinates.
    pub fn get_wall(&self, column: usize, row: usize) -> Wall {
        self.frame[row][column]
    }

    /// Sets wall from frame coordinates.
    pub fn set_wall(&mut self, column: usize, row: usize, cell: Wall) {
        self.frame[row][column] = cell;
    }
}

/// Returns frame for a walled maze with openings in the corners.
/// For example a 3x3 matrix and its walled frame:
/// ```text
/// _ _____ [[H, N, H, H, H, H, H],
/// │_│_│_│  [V, H, V, H, V, H, V],
/// │_│_│_│  [V, H, V, H, V, H, V],
/// │_│_│ │  [V, H, V, H, V, N, V]]
/// ```
fn walled_maze(rows: usize, columns: usize) -> Vec<Vec<Wall>> {
    let mut buffer = Vec::new();

    let top_row = vec![Wall::Horizontal(' '); columns * 2 + 1];

    let mut row = vec![Wall::Vertical];
    for _ in 0..columns {
        row.push(Wall::Horizontal(' '));
        row.push(Wall::Vertical);
    }

    // Create frame.
    buffer.push(top_row);
    for _ in 0..rows {
        buffer.push(row.clone());
    }

    // Create openings.
    buffer[0][1] = Wall::None(' ');
    buffer[rows][columns * 2 - 1] = Wall::None(' '); // (rows, columns * 2 - 1) is lower right cell

    buffer
}
