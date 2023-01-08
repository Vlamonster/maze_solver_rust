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

    // Initialize depth first search.
    let mut visited = HashSet::new();
    let mut unvisited = Vec::new();
    unvisited.push((0, 0));

    // Directions to try for neighbors and rng to randomize it.
    let mut offsets = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    let mut rng = rand::thread_rng();

    // Check if top of stack has unvisited neighbors.
    'top: while let Some(&(x, y)) = unvisited.last() {
        sleep(Duration::from_millis(delay));

        // Set current node as visited.
        visited.insert((x, y));

        // Randomize order of directions to try.
        offsets.shuffle(&mut rng);

        // Calculate cell indices.
        let (cx, cy) = (2 * x + 1, y + 1);

        // Check per neighbor if we have visited it.
        for (dx, dy) in offsets {
            let (nx, ny) = ((x as isize + dx) as usize, (y as isize + dy) as usize);

            // Check that we are still in bounds.
            if !(0..columns).contains(&nx) || !(0..rows).contains(&ny) {
                continue;
            }

            // Check if we have already seen this node.
            if visited.contains(&(nx, ny)) {
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

            let dir = match (dx, dy) {
                (1, _) => '→',
                (-1, _) => '←',
                (_, 1) => '↓',
                (_, -1) => '↑',
                (_, _) => unreachable!(),
            };

            stdout.queue(MoveTo(cx as u16, cy as u16)).unwrap();
            print_cell(stdout, maze.get_cell(2 * x + 1, y + 1), dir);

            // Continue with the top of the stack.
            continue 'top;
        }

        // No more neighbors to visit at this node, so pop it.
        unvisited.pop();

        stdout.queue(MoveTo(cx as u16, cy as u16)).unwrap();
        print_cell(stdout, maze.get_cell(2 * x + 1, y + 1), ' ');
    }

    // Set cursor after the maze.
    stdout.queue(MoveTo(0, rows as u16 + 1)).unwrap();

    maze
}
