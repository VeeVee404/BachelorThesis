use std::fs;
use std::collections::HashMap;

fn main() {
    let content = fs::read_to_string("Day7_Input.txt").expect("Could not read input file");

    let grid: Vec<Vec<char>> = content
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().collect())
        .collect();

    let height = grid.len();
    let width = if height > 0 { grid[0].len() } else { 0 };

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

    // Maps each active column to the number of timelines currently at that column.
    // Unlike part a (HashSet), we keep counts so that timelines are never merged.
    let mut active: HashMap<usize, u64> = HashMap::new();
    active.insert(start_col, 1);

    for row in start_row..height {
        if active.is_empty() {
            break;
        }

        let mut next_active: HashMap<usize, u64> = HashMap::new();

        for (&col, &count) in &active {
            if grid[row][col] == '^' {
                // Split: each of the `count` timelines forks left and right.
                if col > 0 {
                    *next_active.entry(col - 1).or_insert(0) += count;
                }
                if col + 1 < width {
                    *next_active.entry(col + 1).or_insert(0) += count;
                }
            } else {
                // Empty space or S: all timelines pass straight through.
                *next_active.entry(col).or_insert(0) += count;
            }
        }

        active = next_active;
    }

    let total: u64 = active.values().sum();
    println!("{}", total);
}
