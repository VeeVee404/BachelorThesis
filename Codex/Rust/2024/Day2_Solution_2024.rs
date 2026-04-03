use std::time::Instant;
use std::fs;

fn is_safe(levels: &[i64]) -> bool {
    if levels.len() < 2 {
        return true;
    }

    let first_diff = levels[1] - levels[0];
    if first_diff == 0 || first_diff.abs() > 3 {
        return false;
    }

    let increasing = first_diff > 0;

    for window in levels.windows(2) {
        let diff = window[1] - window[0];
        if diff == 0 || diff.abs() > 3 {
            return false;
        }
        if increasing && diff < 0 {
            return false;
        }
        if !increasing && diff > 0 {
            return false;
        }
    }

    true
}

fn main() {
    let start = Instant::now();
    let input = fs::read_to_string("Day2_Input.txt").expect("failed to read Day2_Input.txt");

    let safe_count = input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .filter(|line| {
            let levels: Vec<i64> = line
                .split_whitespace()
                .map(|value| value.parse().expect("invalid integer in input"))
                .collect();
            is_safe(&levels)
        })
        .count();

    println!("{safe_count}");

    println!("Runtime: {:.6} seconds", start.elapsed().as_secs_f64());
}
