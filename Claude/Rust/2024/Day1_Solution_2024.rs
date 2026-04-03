use std::fs;

fn main() {
    let input = fs::read_to_string("Day1_Input.txt").expect("Failed to read input file");

    let mut left: Vec<i64> = Vec::new();
    let mut right: Vec<i64> = Vec::new();

    for line in input.lines() {
        let mut parts = line.split_whitespace();
        if let (Some(a), Some(b)) = (parts.next(), parts.next()) {
            left.push(a.parse().unwrap());
            right.push(b.parse().unwrap());
        }
    }

    left.sort_unstable();
    right.sort_unstable();

    let total: i64 = left.iter().zip(right.iter()).map(|(a, b)| (a - b).abs()).sum();

    println!("{}", total);
}
