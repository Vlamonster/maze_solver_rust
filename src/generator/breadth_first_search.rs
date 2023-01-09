use crate::maze::{init_maze, print_cell, Maze, Wall};
use crossterm::cursor::MoveTo;
use crossterm::QueueableCommand;
use rand::seq::SliceRandom;
use spin_sleep::sleep;
use std::collections::HashSet;
use std::io::Stdout;
use std::time::Duration;

/// Randomizes an initialized maze in the terminal.
pub fn generate_maze(stdout: &mut Stdout, rows: usize, columns: usize, delay: u64) -> Maze {
    // Initialize walled maze without edges.
    let mut maze = init_maze(stdout, rows, columns);

    // Initialize breadth first search.
    let mut visited = HashSet::new();
    let mut unvisited = Vec::new();
    let mut rng = rand::thread_rng();
    unvisited.push((0, 0));

    // Directions to try for neighbors.
    let offsets = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    // Loop while we have unvisited nodes.
    while let Some((x, y)) = unvisited.pop() {
        // Calculate cell indices.
        let (cx, cy) = (2 * x + 1, y + 1);

        // Remove center dot.
        stdout.queue(MoveTo(cx as u16, cy as u16)).unwrap();
        print_cell(stdout, maze.get_cell(cx, cy), ' ');

        sleep(Duration::from_millis(delay));

        // Set current node as visited.
        visited.insert((x, y));

        // Check per neighbor if we have visited it.
        for (dx, dy) in offsets {
            let (nx, ny) = ((x as isize + dx) as usize, (y as isize + dy) as usize);

            // Check that we are still in bounds.
            if !(0..columns).contains(&nx) || !(0..rows).contains(&ny) {
                continue;
            }

            // Check if we have already seen this node or added it to unvisited.
            if visited.contains(&(nx, ny)) || unvisited.contains(&(nx, ny)) {
                continue;
            }

            // Store new edge.
            maze.insert_edge((x, y), (nx, ny));
            maze.insert_edge((nx, ny), (x, y));

            // Push new node to top of stack.
            unvisited.push((nx, ny));

            // Set cursor to wall between current node and neighbor.
            let wx = x + nx + 1;
            let wy = if dy == -1 { ny } else { y } + 1;
            stdout.queue(MoveTo(wx as u16, wy as u16)).unwrap();

            // If we moved horizontally, replace wall with '_', otherwise replace with ' '.
            if dx != 0 {
                print_cell(stdout, Wall::Horizontal, ' ');
                maze.set_cell(wx, wy, Wall::Horizontal)
            } else {
                print_cell(stdout, Wall::None, ' ');
                maze.set_cell(wx, wy, Wall::None)
            }

            // Draw center dot.
            stdout
                .queue(MoveTo(2 * nx as u16 + 1, ny as u16 + 1))
                .unwrap();
            print_cell(stdout, maze.get_cell(2 * nx + 1, ny + 1), '·');
        }

        // Randomize order of unvisited nodes and continue with the new top node.
        unvisited.shuffle(&mut rng);
    }

    // Set cursor after the maze.
    stdout.queue(MoveTo(0, rows as u16 + 1)).unwrap();

    maze
}
