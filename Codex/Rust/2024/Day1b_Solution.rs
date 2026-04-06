use std::collections::HashMap;
use std::fs;

fn main() {
    let input = fs::read_to_string("Day1_Input.txt").expect("failed to read Day1_Input.txt");

    let mut left_values = Vec::new();
    let mut right_counts: HashMap<i64, i64> = HashMap::new();

    for line in input.lines() {
        if line.trim().is_empty() {
            continue;
        }

        let mut parts = line.split_whitespace();
        let left: i64 = parts
            .next()
            .expect("missing left value")
            .parse()
            .expect("invalid left value");
        let right: i64 = parts
            .next()
            .expect("missing right value")
            .parse()
            .expect("invalid right value");

        left_values.push(left);
        *right_counts.entry(right).or_insert(0) += 1;
    }

    let similarity_score: i64 = left_values
        .into_iter()
        .map(|value| value * right_counts.get(&value).copied().unwrap_or(0))
        .sum();

    println!("{}", similarity_score);
}
