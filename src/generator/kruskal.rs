use crate::maze::{Maze, Wall};
use crossterm::cursor::MoveTo;
use crossterm::QueueableCommand;
use disjoint_sets::UnionFind;
use itertools::Itertools;
use rand::seq::SliceRandom;
use spin_sleep::sleep;
use std::io::Stdout;
use std::time::Duration;

pub fn generate(stdout: &mut Stdout, rows: usize, columns: usize, delay: u64) -> Maze {
    let mut maze = Maze::new_walled(rows, columns);

    maze.print(stdout);

    // Initialize kruskal algorithm.
    let mut cells = UnionFind::new(columns * rows);

    // Construct vector of all walls and randomize the order.
    let horizontal_walls = (1..2 * columns).step_by(2).cartesian_product(1..rows);
    let vertical_walls = (2..2 * columns).step_by(2).cartesian_product(1..=rows);
    let mut walls = horizontal_walls.chain(vertical_walls).collect_vec();
    walls.shuffle(&mut rand::thread_rng());

    // Loop while we have unvisited walls.
    while let Some((wx, wy)) = walls.pop() {
        sleep(Duration::from_millis(delay));

        let node_1;
        let node_2;
        let wall;

        if wx % 2 == 0 {
            wall = Wall::Vertical;
            node_1 = ((wx - 1 - 1) / 2, wy - 1); // Node left of wall.
            node_2 = ((wx + 1 - 1) / 2, wy - 1); // Node right of wall.
        } else {
            wall = Wall::Horizontal(' ');
            node_1 = ((wx - 1) / 2, wy - 1); // Node above wall.
            node_2 = ((wx - 1) / 2, wy + 1 - 1); // Node below wall.
        }

        // Reduce nodes to unique identifiers.
        let id1 = node_1.1 * columns + node_1.0;
        let id2 = node_2.1 * columns + node_2.0;

        if cells.equiv(id1, id2) {
            continue;
        }

        cells.union(id1, id2);

        // Set cursor to wall and open it.
        stdout.queue(MoveTo(wx as u16, wy as u16)).unwrap();
        match wall {
            Wall::Horizontal(_) => {
                Wall::None(' ').print(stdout);
                maze.set_wall(wx, wy, Wall::None(' '));
            }
            Wall::Vertical => {
                Wall::Horizontal(' ').print(stdout);
                maze.set_wall(wx, wy, Wall::Horizontal(' '));
            }
            Wall::None(_) => unreachable!(),
        }
    }

    // Set cursor after the maze.
    stdout.queue(MoveTo(0, rows as u16 + 1)).unwrap();

    maze
}

/// Stripped version of `generate()` that *only* draws at the end of generation.
pub fn generate_instant(stdout: &mut Stdout, rows: usize, columns: usize) -> Maze {
    let mut maze = Maze::new_walled(rows, columns);

    // Initialize kruskal algorithm.
    let mut cells = UnionFind::new(columns * rows);

    // Construct vector of all walls and randomize the order.
    let horizontal_walls = (1..2 * columns).step_by(2).cartesian_product(1..rows);
    let vertical_walls = (2..2 * columns).step_by(2).cartesian_product(1..=rows);
    let mut walls = horizontal_walls.chain(vertical_walls).collect_vec();
    walls.shuffle(&mut rand::thread_rng());

    // Loop while we have unvisited walls.
    while let Some((wx, wy)) = walls.pop() {
        let node_1;
        let node_2;
        let wall;

        if wx % 2 == 0 {
            wall = Wall::Vertical;
            node_1 = ((wx - 1 - 1) / 2, wy - 1); // Node left of wall.
            node_2 = ((wx + 1 - 1) / 2, wy - 1); // Node right of wall.
        } else {
            wall = Wall::Horizontal(' ');
            node_1 = ((wx - 1) / 2, wy - 1); // Node above wall.
            node_2 = ((wx - 1) / 2, wy + 1 - 1); // Node below wall.
        }

        // Reduce nodes to unique identifiers.
        let id1 = node_1.1 * columns + node_1.0;
        let id2 = node_2.1 * columns + node_2.0;

        // Skip if nodes belong to the same set.
        if cells.equiv(id1, id2) {
            continue;
        }

        cells.union(id1, id2);

        // Update wall between the two cells.
        match wall {
            Wall::Horizontal(_) => maze.set_wall(wx, wy, Wall::None(' ')),
            Wall::Vertical => maze.set_wall(wx, wy, Wall::Horizontal(' ')),
            Wall::None(_) => unreachable!(),
        }
    }

    // Draw the generated maze in the terminal.
    maze.print(stdout);

    // Set cursor after the maze.
    stdout.queue(MoveTo(0, rows as u16 + 1)).unwrap();

    maze
}
