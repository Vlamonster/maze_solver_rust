use crate::maze::{Maze, Wall};
use anyhow::Result;
use crossterm::cursor::MoveTo;
use crossterm::QueueableCommand;
use rand::seq::SliceRandom;
use spin_sleep::sleep;
use std::collections::HashSet;
use std::io::Stdout;
use std::time::Duration;

/// Generates and draws a maze in the terminal using a randomized breadth-first search.
/// In pseudocode the algorithm looks like this:
/// ```
/// stack.push(start)
/// while let Some(cell) = stack.peek() {
///     if cell.has_unvisited_neighbor(){
///         stack.push(neighbor); // neighbor is picked randomly.
///         stack.shuffle();
///         continue;
///     } else {
///         stack.pop();
///     }
/// }
/// ```
pub fn generate(stdout: &mut Stdout, rows: u16, columns: u16, delay: u64) -> Result<Maze> {
    if delay == 0 {
        return generate_instant(stdout, rows, columns);
    }

    // Create a new walled maze of the specified dimensions.
    let mut maze = Maze::new_walled(rows, columns);

    // Draw the initial maze in the terminal.
    maze.print(stdout)?;

    // Initialize variables for breadth first search algorithm.
    let mut visited = HashSet::<(u16, u16)>::new();
    let mut unvisited = Vec::new();
    unvisited.push((0, 0));

    // Directions to try for neighbors and rng to randomize it.
    let mut offsets = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    let mut rng = rand::thread_rng();

    // Main loop to iterate through the stack.
    'top: while let Some(&(x, y)) = unvisited.last() {
        sleep(Duration::from_millis(delay));
        visited.insert((x, y));

        // Calculate the frame indices of the current cell.
        let (cx, cy) = (2 * x + 1, y + 1);

        // Redraw the current cell, removing previously overwritten characters.
        stdout.queue(MoveTo(cx, cy))?;
        maze.get_wall(cx, cy).print(stdout)?;

        // Randomize order of directions to try.
        offsets.shuffle(&mut rng);
        for (dx, dy) in offsets {
            let (nx, ny) = (x.wrapping_add_signed(dx), y.wrapping_add_signed(dy));

            // Skip out of bounds coordinates.
            if !(0..columns).contains(&nx) || !(0..rows).contains(&ny) {
                continue;
            }

            // Skip if current neighbor has been visited.
            if visited.contains(&(nx, ny)) || unvisited.contains(&(nx, ny)) {
                continue;
            }

            unvisited.push((nx, ny));

            // Update wall between current and next cell.
            let wx = x + nx + 1;
            let wy = if dy == -1 { ny } else { y } + 1;
            stdout.queue(MoveTo(wx, wy))?;

            if dx == 0 {
                Wall::None(' ').print(stdout)?;
                maze.set_wall(wx, wy, Wall::None(' '));
            } else {
                Wall::Horizontal(' ').print(stdout)?;
                maze.set_wall(wx, wy, Wall::Horizontal(' '));
            }

            // Calculate the frame indices of the neighboring cell.
            let (cx, cy) = (2 * nx + 1, ny + 1);

            // Print central dot in neighboring cell.
            stdout.queue(MoveTo(cx, cy))?;
            maze.get_wall(cx, cy).print_with_char(stdout, '·')?;

            unvisited.shuffle(&mut rng);
            continue 'top;
        }

        // No more neighbors to visit at this cell, so pop it.
        unvisited.pop();
    }

    // Set cursor after the maze.
    stdout.queue(MoveTo(0, rows + 1))?;

    Ok(maze)
}

/// Stripped version of `generate()` that *only* draws at the end of generation.
fn generate_instant(stdout: &mut Stdout, rows: u16, columns: u16) -> Result<Maze> {
    // Create a new walled maze of the specified dimensions.
    let mut maze = Maze::new_walled(rows, columns);

    // Initialize variables for breadth first search algorithm.
    let mut visited = HashSet::<(u16, u16)>::new();
    let mut unvisited = Vec::new();
    unvisited.push((0, 0));

    // Directions to try for neighbors and rng to randomize it.
    let mut offsets = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    let mut rng = rand::thread_rng();

    // Main loop to iterate through the stack.
    'top: while let Some(&(x, y)) = unvisited.last() {
        visited.insert((x, y));

        // Randomize order of directions to try.
        offsets.shuffle(&mut rng);
        for (dx, dy) in offsets {
            let (nx, ny) = (x.wrapping_add_signed(dx), y.wrapping_add_signed(dy));

            // Skip out of bounds coordinates.
            if !(0..columns).contains(&nx) || !(0..rows).contains(&ny) {
                continue;
            }

            // Skip if current neighbor has been visited.
            if visited.contains(&(nx, ny)) || unvisited.contains(&(nx, ny)) {
                continue;
            }

            unvisited.push((nx, ny));

            // Update wall between current and next cell.
            let wx = x + nx + 1;
            let wy = if dy == -1 { ny } else { y } + 1;

            if dx == 0 {
                maze.set_wall(wx, wy, Wall::None(' '));
            } else {
                maze.set_wall(wx, wy, Wall::Horizontal(' '));
            }

            unvisited.shuffle(&mut rng);
            continue 'top;
        }

        // No more neighbors to visit at this cell, so pop it.
        unvisited.pop();
    }

    // Draw the generated maze in the terminal.
    maze.print(stdout)?;

    // Set cursor after the maze.
    stdout.queue(MoveTo(0, rows + 1))?;

    Ok(maze)
}
