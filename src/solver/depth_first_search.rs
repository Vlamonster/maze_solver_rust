use crate::maze::{Maze, Wall};
use crossterm::cursor::MoveTo;
use crossterm::QueueableCommand;
use itertools::Itertools;
use spin_sleep::sleep;
use std::collections::HashSet;
use std::io::Stdout;
use std::time::Duration;

pub fn solve(stdout: &mut Stdout, maze: &mut Maze, delay: u64, trace: bool) {
    let (tx, ty) = maze.get_end();

    let mut visited = HashSet::new();
    let mut unvisited = Vec::new();
    unvisited.push((0, 0));

    'top: while let Some(&(x, y)) = unvisited.last() {
        visited.insert((x, y));

        // Print central dot in current cell.
        if trace {
            sleep(Duration::from_millis(delay));

            // Calculate the frame indices of the current cell.
            let (cx, cy) = (2 * x + 1, y + 1);

            stdout.queue(MoveTo(cx as u16, cy as u16)).unwrap();
            maze.get_wall(cx, cy).print_with_char(stdout, '·');
        }

        if (x, y) == (tx, ty) {
            unvisited.push((tx, ty + 1));
            break 'top;
        }

        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let (nx, ny) = ((x as isize + dx) as usize, (y as isize + dy) as usize);

            // Skip out of bounds coordinates.
            if !(0..=tx).contains(&nx) || !(0..=ty).contains(&ny) {
                continue;
            }

            // Skip if current neighbor has been visited.
            if visited.contains(&(nx, ny)) {
                continue;
            }

            // Calculate the frame indices of the wall between the current cell and its neighbor.
            let wx = x + nx + 1;
            let wy = if dy == -1 { ny } else { y } + 1;

            // Skip if wall between current cell and neighbor
            match (dx, maze.get_wall(wx, wy)) {
                (0, Wall::Horizontal(_)) => continue,
                (_, Wall::Vertical) => continue,
                (_, _) => {}
            }

            unvisited.push((nx, ny));

            continue 'top;
        }

        // No more neighbors to visit at this cell, so pop it.
        unvisited.pop();
    }

    // Draw path.
    for (&(x, y), &(nx, ny)) in unvisited.iter().tuple_windows() {
        sleep(Duration::from_millis(delay));

        let (dx, dy) = (nx as isize - x as isize, ny as isize - y as isize);

        // Print arrow pointing to neighbor in current cell.
        #[rustfmt::skip]
        let dir = match (dx, dy) {
            ( 1,  _) => '→',
            (-1,  _) => '←',
            ( _,  1) => '↓',
            ( _, -1) => '↑',
            ( _,  _) => unreachable!(),
        };

        // Calculate the frame indices of the current cell.
        let (cx, cy) = (2 * x + 1, y + 1);

        stdout.queue(MoveTo(cx as u16, cy as u16)).unwrap();
        maze.get_wall(cx, cy).print_with_char(stdout, dir);
    }

    // Set cursor after the maze.
    stdout.queue(MoveTo(0, ty as u16 + 2)).unwrap();
}
