use crate::maze::{Maze, Wall};
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
pub fn generate(stdout: &mut Stdout, rows: usize, columns: usize, delay: u64) -> Maze {
    let mut maze = Maze::new_walled(rows, columns);

    maze.print(stdout);

    // Initialize breadth first search.
    let mut visited = HashSet::new();
    let mut unvisited = Vec::new();
    unvisited.push((0, 0));

    // Directions to try for neighbors.
    let mut offsets = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    let mut rng = rand::thread_rng();

    // Loop while we have unvisited nodes.
    'top: while let Some(&(x, y)) = unvisited.last() {
        sleep(Duration::from_millis(delay));

        // Set current node as visited.
        visited.insert((x, y));

        // Calculate cell indices.
        let (cx, cy) = (2 * x + 1, y + 1);

        // Remove center dot.
        stdout.queue(MoveTo(cx as u16, cy as u16)).unwrap();
        maze.get_wall(cx, cy).print(stdout);

        // Randomize order of directions to try.
        offsets.shuffle(&mut rng);

        // Check per neighbor if we have visited it.
        for (dx, dy) in offsets {
            let (nx, ny) = ((x as isize + dx) as usize, (y as isize + dy) as usize);

            // Check that we are still in bounds.
            if !(0..columns).contains(&nx) || !(0..rows).contains(&ny) {
                continue;
            }

            // Check if we have already seen this node before.
            if visited.contains(&(nx, ny)) || unvisited.contains(&(nx, ny)) {
                continue;
            }

            // Push new node to top of stack.
            unvisited.push((nx, ny));

            // Set cursor to wall between current node and neighbor.
            let wx = x + nx + 1;
            let wy = if dy == -1 { ny } else { y } + 1;
            stdout.queue(MoveTo(wx as u16, wy as u16)).unwrap();

            // Check if we moved horizontally.
            if dx != 0 {
                Wall::Horizontal(' ').print(stdout);
                maze.set_wall(wx, wy, Wall::Horizontal(' '))
            } else {
                Wall::None(' ').print(stdout);
                maze.set_wall(wx, wy, Wall::None(' '))
            }

            // Calculate cell indices.
            let (cx, cy) = (2 * nx + 1, ny + 1);

            // Draw center dot.
            stdout.queue(MoveTo(cx as u16, cy as u16)).unwrap();
            maze.get_wall(cx, cy).print_with_char(stdout, '·');

            // Randomize order of unvisited nodes and continue with the new top node.
            unvisited.shuffle(&mut rng);
            continue 'top;
        }

        // No more neighbors to visit at this node, so pop it.
        unvisited.pop();
    }

    // Set cursor after the maze.
    stdout.queue(MoveTo(0, rows as u16 + 1)).unwrap();

    maze
}

/// Generates and draws a maze in the terminal using a randomized breadth-first search.
/// *Only* draws at the end of generation.
pub fn generate_instant(stdout: &mut Stdout, rows: usize, columns: usize) -> Maze {
    let mut maze = Maze::new_walled(rows, columns);

    // Initialize breadth first search.
    let mut visited = HashSet::new();
    let mut unvisited = Vec::new();
    unvisited.push((0, 0));

    // Directions to try for neighbors.
    let mut offsets = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    let mut rng = rand::thread_rng();

    // Loop while we have unvisited nodes.
    'top: while let Some(&(x, y)) = unvisited.last() {
        // Set current node as visited.
        visited.insert((x, y));

        // Randomize order of directions to try.
        offsets.shuffle(&mut rng);

        // Check per neighbor if we have visited it.
        for (dx, dy) in offsets {
            let (nx, ny) = ((x as isize + dx) as usize, (y as isize + dy) as usize);

            // Check that we are still in bounds.
            if !(0..columns).contains(&nx) || !(0..rows).contains(&ny) {
                continue;
            }

            // Check if we have already seen this node before.
            if visited.contains(&(nx, ny)) || unvisited.contains(&(nx, ny)) {
                continue;
            }

            // Push new node to top of stack.
            unvisited.push((nx, ny));

            // Set cursor to wall between current node and neighbor.
            let wx = x + nx + 1;
            let wy = if dy == -1 { ny } else { y } + 1;

            // Check if we moved horizontally.
            if dx != 0 {
                maze.set_wall(wx, wy, Wall::Horizontal(' '))
            } else {
                maze.set_wall(wx, wy, Wall::None(' '))
            }

            // Randomize order of unvisited nodes and continue with the new top node.
            unvisited.shuffle(&mut rng);
            continue 'top;
        }

        // No more neighbors to visit at this node, so pop it.
        unvisited.pop();
    }

    maze.print(stdout);

    // Set cursor after the maze.
    stdout.queue(MoveTo(0, rows as u16 + 1)).unwrap();

    maze
}
