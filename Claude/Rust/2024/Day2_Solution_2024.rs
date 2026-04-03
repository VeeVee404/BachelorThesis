use std::time::Instant;
use std::fs;

fn is_safe(levels: &[i64]) -> bool {
    if levels.len() < 2 {
        return true;
    }
    let increasing = levels[1] > levels[0];
    for w in levels.windows(2) {
        let diff = w[1] - w[0];
        if increasing {
            if diff < 1 || diff > 3 {
                return false;
            }
        } else {
            if diff > -1 || diff < -3 {
                return false;
            }
        }
    }
    true
}

fn main() {
    let start = Instant::now();
    let input = fs::read_to_string("Day2_Input.txt").expect("Failed to read input file");

    let count = input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .filter(|line| {
            let levels: Vec<i64> = line
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect();
            is_safe(&levels)
        })
        .count();

    println!("{}", count);

    println!("Runtime: {:.6} seconds", start.elapsed().as_secs_f64());
}
