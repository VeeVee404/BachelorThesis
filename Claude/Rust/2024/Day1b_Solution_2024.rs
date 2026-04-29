use std::collections::HashMap;
use std::fs;

fn main() {
    let input = fs::read_to_string("Day1_Input.txt").expect("Failed to read input file");

    let mut left = Vec::new();
    let mut right_counts: HashMap<i64, i64> = HashMap::new();

    for line in input.lines() {
        let mut nums = line.split_whitespace();
        if let (Some(a), Some(b)) = (nums.next(), nums.next()) {
            left.push(a.parse::<i64>().unwrap());
            *right_counts.entry(b.parse::<i64>().unwrap()).or_insert(0) += 1;
        }
    }

    let similarity: i64 = left.iter().map(|n| n * right_counts.get(n).copied().unwrap_or(0)).sum();

    println!("{}", similarity);
}
