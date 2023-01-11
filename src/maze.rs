use anyhow::Result;
use crossterm::cursor::{Hide, MoveTo};
use crossterm::style::Attribute::{NoUnderline, Underlined};
use crossterm::style::Print;
use crossterm::terminal::{Clear, ClearType};
use crossterm::{ExecutableCommand, QueueableCommand};
use itertools::Itertools;
use std::fs::File;
use std::io::{Read, Stdout, Write};
use std::path::PathBuf;

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
    pub fn print(self, stdout: &mut Stdout) -> Result<()> {
        match self {
            Wall::Horizontal(char) => {
                stdout.execute(Print(format!("{Underlined}{char}{NoUnderline}")))?
            }
            Wall::Vertical => stdout.execute(Print('│'))?,
            Wall::None(char) => stdout.execute(Print(char))?,
        };
        Ok(())
    }

    /// Prints wall at current cursor position with the given character. Panics if Wall is Vertical.
    pub fn print_with_char(self, stdout: &mut Stdout, char: char) -> Result<()> {
        match self {
            Wall::Horizontal(_) => {
                stdout.execute(Print(format!("{Underlined}{char}{NoUnderline}")))?
            }
            Wall::None(_) => stdout.execute(Print(char))?,
            Wall::Vertical => unreachable!(),
        };
        Ok(())
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

    /// Parses maze from path.
    pub fn from_path(path: PathBuf) -> Result<Maze> {
        parse_maze(path)
    }

    /// Clears the terminal and prints the frame of the maze to the terminal.
    pub fn print(&self, stdout: &mut Stdout) -> Result<()> {
        stdout.queue(Hide)?;
        stdout.queue(MoveTo(0, 0))?;
        stdout.queue(Clear(ClearType::All))?;

        for row in &self.frame {
            for &wall in row {
                wall.print(stdout)?;
            }
            stdout.queue(Print('\n'))?;
        }

        // Flush to make sure the frame is drawn.
        stdout.flush()?;

        Ok(())
    }

    /// Returns wall from frame coordinates.
    pub fn get_wall(&self, column: usize, row: usize) -> Wall {
        self.frame[row][column]
    }

    /// Sets wall from frame coordinates.
    pub fn set_wall(&mut self, column: usize, row: usize, cell: Wall) {
        self.frame[row][column] = cell;
    }

    pub fn get_end(&self) -> (usize, usize) {
        (self.columns - 1, self.rows - 1)
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

/// Parses maze from path. Files should be stored as follows:
/// * Horizontal = '_' (underscore)
/// * Vertical = '|' (pipe)
/// * None = ' ' (space)
fn parse_maze(path: PathBuf) -> Result<Maze> {
    let mut buffer = String::new();
    File::open(path)?.read_to_string(&mut buffer)?;

    let frame = buffer
        .lines()
        .map(|line| {
            line.chars()
                .map(|wall| match wall {
                    '_' => Wall::Horizontal(' '),
                    '|' => Wall::Vertical,
                    ' ' => Wall::None(' '),
                    x => panic!("Bad character in file: {x}!"),
                })
                .collect_vec()
        })
        .collect_vec();

    Ok(Maze {
        rows: frame.len() - 1,
        columns: (frame[0].len() - 1) / 2,
        frame,
    })
}
