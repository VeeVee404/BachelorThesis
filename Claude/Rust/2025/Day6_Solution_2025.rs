use std::time::Instant;
use std::fs;

fn main() {
    let start = Instant::now();
    let content = fs::read_to_string("Day6_Input.txt").expect("Could not read input file");

    let lines: Vec<&str> = content.lines().filter(|l| !l.is_empty()).collect();

    // Pad all rows to the same width with spaces
    let max_len = lines.iter().map(|l| l.len()).max().unwrap_or(0);
    let grid: Vec<Vec<char>> = lines
        .iter()
        .map(|l| {
            let mut chars: Vec<char> = l.chars().collect();
            chars.resize(max_len, ' ');
            chars
        })
        .collect();

    let num_rows = grid.len();
    let op_row_idx = num_rows - 1; // last row is the operator row
    let number_rows = &grid[..op_row_idx];

    // A separator column is one where every row has a space character.
    // Non-separator columns belong to problems.
    let mut is_separator = vec![true; max_len];
    for row in &grid {
        for (col, &ch) in row.iter().enumerate() {
            if ch != ' ' {
                is_separator[col] = false;
            }
        }
    }

    // Collect contiguous runs of non-separator columns as problem column ranges.
    let mut groups: Vec<(usize, usize)> = Vec::new();
    let mut in_group = false;
    let mut group_start = 0;

    for col in 0..max_len {
        if !is_separator[col] {
            if !in_group {
                group_start = col;
                in_group = true;
            }
        } else if in_group {
            groups.push((group_start, col - 1));
            in_group = false;
        }
    }
    if in_group {
        groups.push((group_start, max_len - 1));
    }

    let mut grand_total: i128 = 0;

    for (start, end) in groups {
        // Find the operator (+/*) for this problem.
        let op = match grid[op_row_idx][start..=end]
            .iter()
            .find(|&&c| c == '+' || c == '*')
            .copied()
        {
            Some(c) => c,
            None => continue,
        };

        // Collect all numbers from number rows within this column range.
        let mut numbers: Vec<i128> = Vec::new();
        for row in number_rows {
            let slice: String = row[start..=end].iter().collect();
            for token in slice.split_whitespace() {
                if let Ok(n) = token.parse::<i128>() {
                    numbers.push(n);
                }
            }
        }

        if numbers.is_empty() {
            continue;
        }

        let result: i128 = match op {
            '+' => numbers.iter().sum(),
            '*' => numbers.iter().product(),
            _ => continue,
        };

        grand_total += result;
    }

    println!("{}", grand_total);

    println!("Runtime: {:.6} seconds", start.elapsed().as_secs_f64());
}
