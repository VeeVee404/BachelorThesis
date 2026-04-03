use std::collections::HashSet;
use std::fs;

fn main() {
    let content = fs::read_to_string("Day7_input.txt").expect("Could not read input file");

    let grid: Vec<Vec<char>> = content
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().collect())
        .collect();

    let height = grid.len();
    let width = if height > 0 { grid[0].len() } else { 0 };

    // Locate S (beam entry point)
    let mut start_row = 0;
    let mut start_col = 0;
    'find_s: for (r, row) in grid.iter().enumerate() {
        for (c, &ch) in row.iter().enumerate() {
            if ch == 'S' {
                start_row = r;
                start_col = c;
                break 'find_s;
            }
        }
    }

    // active: set of column indices that currently carry a downward beam.
    // Using a HashSet merges beams that converge on the same column.
    let mut active: HashSet<usize> = HashSet::new();
    active.insert(start_col);

    let mut split_count: u64 = 0;

    for row in start_row..height {
        if active.is_empty() {
            break;
        }

        let mut next_active: HashSet<usize> = HashSet::new();

        for &col in &active {
            if grid[row][col] == '^' {
                // Beam is split: stop here, emit beams left and right.
                split_count += 1;
                if col > 0 {
                    next_active.insert(col - 1);
                }
                if col + 1 < width {
                    next_active.insert(col + 1);
                }
            } else {
                // Empty space or S: beam passes through downward.
                next_active.insert(col);
            }
        }

        active = next_active;
    }

    println!("{}", split_count);
}
