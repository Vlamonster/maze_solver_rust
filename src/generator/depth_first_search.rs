use crate::maze::{Maze, Wall};
use crossterm::cursor::MoveTo;
use crossterm::QueueableCommand;
use rand::seq::SliceRandom;
use spin_sleep::sleep;
use std::collections::HashSet;
use std::io::Stdout;
use std::time::Duration;

/// Generates and draws a maze in the terminal using a randomized depth-first search.
/// In pseudocode the algorithm looks like this:
/// ```
/// stack.push(start)
/// while let Some(cell) = stack.peek() {
///     if cell.has_unvisited_neighbor(){
///         stack.push(neighbor); // neighbor is picked randomly.
///         continue;
///     } else {
///         stack.pop();
///     }
/// }
/// ```
pub fn generate(stdout: &mut Stdout, rows: usize, columns: usize, delay: u64) -> Maze {
    // Create a new walled maze of the specified dimensions.
    let mut maze = Maze::new_walled(rows, columns);

    // Draw the initial maze in the terminal.
    maze.print(stdout);

    // Initialize variables for depth first search algorithm.
    let mut visited = HashSet::new();
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

        // Randomize order of directions to try.
        offsets.shuffle(&mut rng);
        for (dx, dy) in offsets {
            let (nx, ny) = ((x as isize + dx) as usize, (y as isize + dy) as usize);

            // Skip out of bounds coordinates.
            if !(0..columns).contains(&nx) || !(0..rows).contains(&ny) {
                continue;
            }

            // Skip if current neighbor has been visited.
            if visited.contains(&(nx, ny)) {
                continue;
            }

            unvisited.push((nx, ny));

            // Update wall between current and next cell.
            let wx = x + nx + 1;
            let wy = if dy == -1 { ny } else { y } + 1;
            stdout.queue(MoveTo(wx as u16, wy as u16)).unwrap();

            if dx == 0 {
                Wall::None(' ').print(stdout);
                maze.set_wall(wx, wy, Wall::None(' '));
            } else {
                Wall::Horizontal(' ').print(stdout);
                maze.set_wall(wx, wy, Wall::Horizontal(' '));
            }

            // Print arrow pointing to neighbor in current cell.
            #[rustfmt::skip]
            let dir = match (dx, dy) {
                ( 1,  _) => '→',
                (-1,  _) => '←',
                ( _,  1) => '↓',
                ( _, -1) => '↑',
                ( _,  _) => unreachable!(),
            };

            stdout.queue(MoveTo(cx as u16, cy as u16)).unwrap();
            maze.get_wall(cx, cy).print_with_char(stdout, dir);

            continue 'top;
        }

        // No more neighbors to visit at this cell, so pop it.
        unvisited.pop();

        // Redraw the current cell, removing previously overwritten characters.
        stdout.queue(MoveTo(cx as u16, cy as u16)).unwrap();
        maze.get_wall(cx, cy).print(stdout);
    }

    // Set cursor after the maze.
    stdout.queue(MoveTo(0, rows as u16 + 1)).unwrap();

    maze
}

/// Stripped version of `generate()` that *only* draws at the end of generation.
pub fn generate_instant(stdout: &mut Stdout, rows: usize, columns: usize) -> Maze {
    // Create a new walled maze of the specified dimensions.
    let mut maze = Maze::new_walled(rows, columns);

    // Initialize variables for depth first search algorithm.
    let mut visited = HashSet::new();
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
            let (nx, ny) = ((x as isize + dx) as usize, (y as isize + dy) as usize);

            // Skip out of bounds coordinates.
            if !(0..columns).contains(&nx) || !(0..rows).contains(&ny) {
                continue;
            }

            // Skip if current neighbor has been visited.
            if visited.contains(&(nx, ny)) {
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

            continue 'top;
        }

        // No more neighbors to visit at this cell, so pop it.
        unvisited.pop();
    }

    // Draw the generated maze in the terminal.
    maze.print(stdout);

    // Set cursor after the maze.
    stdout.queue(MoveTo(0, rows as u16 + 1)).unwrap();

    maze
}
