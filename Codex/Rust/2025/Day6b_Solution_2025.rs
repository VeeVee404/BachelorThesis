use std::fs;

fn main() {
    let input = fs::read_to_string("Day6_Input.txt").expect("failed to read Day6_Input.txt");
    let lines: Vec<&str> = input.lines().collect();

    if lines.is_empty() {
        println!("0");
        return;
    }

    let width = lines.iter().map(|line| line.len()).max().unwrap_or(0);
    let grid: Vec<Vec<u8>> = lines
        .iter()
        .map(|line| {
            let mut row = line.as_bytes().to_vec();
            row.resize(width, b' ');
            row
        })
        .collect();

    let value_rows = grid.len() - 1;
    let mut total = 0_u128;
    let mut col = 0_usize;

    while col < width {
        if grid.iter().all(|row| row[col] == b' ') {
            col += 1;
            continue;
        }

        let start = col;
        while col < width && grid.iter().any(|row| row[col] != b' ') {
            col += 1;
        }
        let end = col;

        let op = grid[value_rows][start..end]
            .iter()
            .copied()
            .find(|&ch| ch == b'+' || ch == b'*')
            .expect("missing operator");

        let mut numbers = Vec::new();
        for current_col in (start..end).rev() {
            let mut value = 0_u128;
            let mut has_digit = false;

            for row in 0..value_rows {
                let ch = grid[row][current_col];
                if ch.is_ascii_digit() {
                    value = value * 10 + u128::from(ch - b'0');
                    has_digit = true;
                }
            }

            if has_digit {
                numbers.push(value);
            }
        }

        let answer = match op {
            b'+' => numbers.into_iter().sum::<u128>(),
            b'*' => numbers.into_iter().product::<u128>(),
            _ => unreachable!(),
        };

        total += answer;
    }

    println!("{total}");
}
