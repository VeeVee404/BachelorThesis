use std::fs;

fn main() {
    let input =
        fs::read_to_string("Day1_Input.txt").expect("failed to read Day1_Input.txt");

    let mut left = Vec::new();
    let mut right = Vec::new();

    for line in input.lines() {
        if line.trim().is_empty() {
            continue;
        }

        let mut parts = line.split_whitespace();
        let a: i64 = parts
            .next()
            .expect("missing left value")
            .parse()
            .expect("invalid left value");
        let b: i64 = parts
            .next()
            .expect("missing right value")
            .parse()
            .expect("invalid right value");

        assert!(parts.next().is_none(), "too many values on a line");

        left.push(a);
        right.push(b);
    }

    left.sort_unstable();
    right.sort_unstable();

    let total_distance: i64 = left
        .iter()
        .zip(&right)
        .map(|(a, b)| (a - b).abs())
        .sum();

    println!("{total_distance}");
}
