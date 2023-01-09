use crate::maze::{Maze, Wall};
use crossterm::cursor::MoveTo;
use crossterm::QueueableCommand;
use itertools::Itertools;
use rand::seq::SliceRandom;
use spin_sleep::sleep;
use std::io::Stdout;
use std::time::Duration;

pub fn generate(stdout: &mut Stdout, rows: usize, columns: usize, delay: u64) -> Maze {
    let maze = Maze::new_walled(rows, columns);
    maze.print(stdout);

    // Initialize kruskal algorithm.
    let mut ids = (0..columns)
        .cartesian_product(0..rows)
        .map(|(x, y)| y * columns + x)
        .collect_vec();

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
            // Vertical wall.
            wall = Wall::Vertical;
            node_1 = ((wx - 1 - 1) / 2, wy - 1); // Node left of wall.
            node_2 = ((wx + 1 - 1) / 2, wy - 1); // Node right of wall.
        } else {
            // Horizontal wall.
            wall = Wall::Horizontal(' ');
            node_1 = ((wx - 1) / 2, wy - 1); // Node above wall.
            node_2 = ((wx - 1) / 2, wy + 1 - 1); // Node below wall.
        }

        let id_1 = ids[node_1.1 * columns + node_1.0];
        let id_2 = ids[node_2.1 * columns + node_2.0];

        // If cells have same id continue.
        if id_1 == id_2 {
            continue;
        }

        for id in ids.iter_mut() {
            if *id == id_1 {
                *id = id_2;
            }
        }

        // Set cursor to wall and open it.
        stdout.queue(MoveTo(wx as u16, wy as u16)).unwrap();
        match wall {
            Wall::Horizontal(_) => Wall::None(' ').print(stdout),
            Wall::Vertical => Wall::Horizontal(' ').print(stdout),
            _ => unreachable!(),
        }
    }

    // Set cursor after the maze.
    stdout.queue(MoveTo(0, rows as u16 + 1)).unwrap();

    maze
}
