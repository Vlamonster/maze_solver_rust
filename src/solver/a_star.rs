use crate::maze::{Maze, Wall};
use anyhow::Result;
use std::collections::HashSet;

use binary_heap_plus::BinaryHeap;
use crossterm::cursor::MoveTo;
use crossterm::QueueableCommand;
use spin_sleep::sleep;
use std::io::Stdout;
use std::time::Duration;

pub fn solve(stdout: &mut Stdout, maze: &mut Maze, delay: u64, trace: bool) -> Result<()> {
    let (tx, ty) = maze.get_end();

    let mut visited = HashSet::new();
    let mut unvisited = BinaryHeap::new_by(|&node_1, &node_2| {
        distance(node_2, (tx, ty)).cmp(&distance(node_1, (tx, ty)))
    });
    unvisited.push((0, 0));

    'top: while let Some((x, y)) = unvisited.pop() {
        visited.insert((x, y));

        // Print central dot in current cell.
        if trace {
            sleep(Duration::from_millis(delay));

            // Calculate the frame indices of the current cell.
            let (cx, cy) = (2 * x + 1, y + 1);

            stdout.queue(MoveTo(cx, cy))?;
            maze.get_wall(cx, cy).print_with_char(stdout, '·')?;
        }

        if (x, y) == (tx, ty) {
            break 'top;
        }

        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let (nx, ny) = (x.wrapping_add_signed(dx), y.wrapping_add_signed(dy));

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
                (_, Wall::Vertical) | (0, Wall::Horizontal(_)) => continue,
                (_, _) => {}
            }

            unvisited.push((nx, ny));
        }
    }

    Ok(())
}

/// Returns manhattan distance between cells.
fn distance((x1, y1): (u16, u16), (x2, y2): (u16, u16)) -> u16 {
    x1.abs_diff(x2) + y1.abs_diff(y2)
}
