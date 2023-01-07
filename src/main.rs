use crossterm::cursor::{Hide, MoveTo, RestorePosition, SavePosition};
use crossterm::style::Print;
use crossterm::terminal::{Clear, ClearType};
use crossterm::QueueableCommand;
use rand::seq::SliceRandom;
use std::collections::HashSet;
use std::io::{stdout, Stdout, Write};

/// Initializes walled maze in the terminal.
fn init_maze(stdout: &mut Stdout, rows: isize, cols: isize) {
    stdout.queue(Hide).unwrap();
    stdout.queue(MoveTo(0, 0)).unwrap();
    stdout.queue(Clear(ClearType::All)).unwrap();

    let mut maze = String::new();

    // draw top row
    maze.push('_');
    for _ in 0..cols {
        maze.push_str("__");
    }
    maze.push('\n');

    // draw rest of the rows
    for _ in 0..rows {
        maze.push('│');
        for _ in 0..cols {
            maze.push_str("_│");
        }
        maze.push('\n');
    }

    for line in maze.lines() {
        stdout.queue(Clear(ClearType::UntilNewLine)).unwrap();
        stdout.queue(Print(line)).unwrap();
        stdout.queue(Print('\n')).unwrap();
    }

    // flush to make sure it's drawn
    stdout.flush().unwrap();
}

/// Randomizes an initialized maze in the terminal.
/// todo: should return Maze structure that can be used to solve later.
fn generate_maze(stdout: &mut Stdout, rows: isize, cols: isize) {
    let mut rng = rand::thread_rng();
    let mut visited = HashSet::new();
    let mut unvisited = Vec::new();
    unvisited.push((0, 0));

    'outer: while let Some(&(x, y)) = unvisited.last() {
        visited.insert((x, y));
        let mut offsets = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        offsets.shuffle(&mut rng);
        for (dx, dy) in offsets {
            let (nx, ny) = (x + dx, y + dy);
            if !visited.contains(&(nx, ny)) && (0..rows).contains(&ny) && (0..cols).contains(&nx) {
                stdout
                    .queue(MoveTo(
                        (2 * x + 1 + dx) as u16,
                        (y + 1 + dy.clamp(-1, 0)) as u16,
                    ))
                    .unwrap();
                if dx != 0 {
                    stdout.queue(Print('_')).unwrap();
                } else {
                    stdout.queue(Print(' ')).unwrap();
                }
                unvisited.push((nx, ny));
                continue 'outer;
            }
        }
        unvisited.pop();
    }
}

fn main() {
    let mut stdout = stdout();
    init_maze(&mut stdout, 18, 48);
    stdout.queue(SavePosition).unwrap();
    generate_maze(&mut stdout, 18, 48);
    stdout.queue(RestorePosition).unwrap();
}
