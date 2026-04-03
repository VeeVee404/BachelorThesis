use std::time::Instant;
use std::fs;

fn main() {
    let start = Instant::now();
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

    let mut total: u128 = 0;
    let mut col = 0usize;

    while col < width {
        let is_separator = grid.iter().all(|row| row[col] == b' ');
        if is_separator {
            col += 1;
            continue;
        }

        let start = col;
        while col < width && grid.iter().any(|row| row[col] != b' ') {
            col += 1;
        }
        let end = col;

        let mut numbers = Vec::new();
        for row in &grid[..grid.len() - 1] {
            let text = std::str::from_utf8(&row[start..end]).expect("invalid UTF-8").trim();
            if !text.is_empty() {
                numbers.push(text.parse::<u128>().expect("invalid number"));
            }
        }

        let op = std::str::from_utf8(&grid[grid.len() - 1][start..end])
            .expect("invalid UTF-8")
            .trim();

        let value = match op {
            "+" => numbers.into_iter().sum::<u128>(),
            "*" => numbers.into_iter().product::<u128>(),
            _ => panic!("invalid operator"),
        };

        total += value;
    }

    println!("{total}");

    println!("Runtime: {:.6} seconds", start.elapsed().as_secs_f64());
}
