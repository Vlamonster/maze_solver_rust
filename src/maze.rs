use anyhow::{bail, Result};
use crossterm::cursor::{Hide, MoveTo};
use crossterm::style::Attribute::{NoUnderline, Underlined};
use crossterm::style::Print;
use crossterm::terminal::{Clear, ClearType};
use crossterm::{ExecutableCommand, QueueableCommand};
use itertools::Itertools;
use std::fs::File;
use std::io::{Read, Stdout, Write};
use std::path::PathBuf;
use thiserror::Error;

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
    rows: u16,
    columns: u16,
    frame: Vec<Vec<Wall>>,
}

impl Maze {
    /// Returns maze initialized with walls around every cell.
    pub fn new_walled(rows: u16, columns: u16) -> Maze {
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
    pub fn get_wall(&self, column: u16, row: u16) -> Wall {
        self.frame[row as usize][column as usize]
    }

    /// Sets wall from frame coordinates.
    pub fn set_wall(&mut self, column: u16, row: u16, cell: Wall) {
        self.frame[row as usize][column as usize] = cell;
    }

    pub fn get_end(&self) -> (u16, u16) {
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
fn walled_maze(rows: u16, columns: u16) -> Vec<Vec<Wall>> {
    let mut buffer = Vec::new();

    let top_row = vec![Wall::Horizontal(' '); columns as usize * 2 + 1];

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
    buffer[rows as usize][columns as usize * 2 - 1] = Wall::None(' '); // (rows, columns * 2 - 1) is lower right cell

    buffer
}

#[derive(Error, Debug)]
enum ParsingError {
    #[error("Bad character '{2}' found at {0}:{1}.")]
    BadCharacter(usize, usize, char),
    #[error("There are not enough character rows.")]
    NotEnoughRows,
    #[error("There are not enough character columns.")]
    NotEnoughColumns,
    #[error("There are an even number of character columns.")]
    EvenNumberOfColumns,
    #[error("Varying character row length for row {0}.")]
    VaryingRowLengths(usize),
    #[error("Number of rows exceeds u16::MAX (65535).")]
    TooManyRows,
    #[error("Number of columns exceeds u16::MAX (65535).")]
    TooManyColumns,
}

/// Parses maze from path. Files should be stored as follows:
/// * Horizontal = '_' (underscore)
/// * Vertical = '|' (pipe)
/// * None = ' ' (space)
fn parse_maze(path: PathBuf) -> Result<Maze> {
    let mut buffer = String::new();
    File::open(path)?.read_to_string(&mut buffer)?;

    let height = u16::try_from(buffer.lines().count()).or(Err(ParsingError::TooManyRows))?;
    let width = u16::try_from(
        buffer
            .lines()
            .next()
            .ok_or(ParsingError::NotEnoughRows)?
            .len(),
    )
    .or(Err(ParsingError::TooManyColumns))?;

    match (width, height) {
        (_, 0..=1) => bail!(ParsingError::NotEnoughRows),
        (0..=2, _) => bail!(ParsingError::NotEnoughColumns),
        (w, _) if w % 2 == 0 => bail!(ParsingError::EvenNumberOfColumns),
        (_, _) => {}
    }

    let frame: Vec<Vec<Wall>> = buffer
        .lines()
        .enumerate()
        .map(|(row, line)| {
            if line.len() == width as usize {
                line.chars()
                    .enumerate()
                    .map(|(column, char)| match char {
                        '_' => Ok(Wall::Horizontal(' ')),
                        '|' => Ok(Wall::Vertical),
                        ' ' => Ok(Wall::None(' ')),
                        c => Err(ParsingError::BadCharacter(row + 1, column + 1, c)),
                    })
                    .collect()
            } else {
                Err(ParsingError::VaryingRowLengths(row + 1))
            }
        })
        .try_collect()?;

    Ok(Maze {
        rows: height - 1,
        columns: (width - 1) / 2,
        frame,
    })
}
