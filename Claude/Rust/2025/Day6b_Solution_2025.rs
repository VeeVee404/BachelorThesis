use std::fs;

fn main() {
    let content = fs::read_to_string("Day6_Input.txt").expect("Could not read input file");

    let lines: Vec<&str> = content.lines().filter(|l| !l.is_empty()).collect();

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
    let op_row_idx = num_rows - 1;
    let number_rows = &grid[..op_row_idx];

    // Separator column: every row is a space.
    let mut is_separator = vec![true; max_len];
    for row in &grid {
        for (col, &ch) in row.iter().enumerate() {
            if ch != ' ' {
                is_separator[col] = false;
            }
        }
    }

    // Collect contiguous runs of non-separator columns as problem groups.
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
        // Find the operator in the op row for this group.
        let op = match grid[op_row_idx][start..=end]
            .iter()
            .find(|&&c| c == '+' || c == '*')
            .copied()
        {
            Some(c) => c,
            None => continue,
        };

        // Read numbers right-to-left: each character column forms one number,
        // digits are the non-space chars in number rows read top-to-bottom.
        let mut numbers: Vec<i128> = Vec::new();
        for col in (start..=end).rev() {
            let digits: String = number_rows
                .iter()
                .map(|row| row[col])
                .filter(|&ch| ch != ' ')
                .collect();
            if !digits.is_empty() {
                if let Ok(n) = digits.parse::<i128>() {
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
}
