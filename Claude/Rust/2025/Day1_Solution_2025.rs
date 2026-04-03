use std::time::Instant;
use std::fs;

fn main() {
    let start = Instant::now();
    let content = fs::read_to_string("Day1_Input.txt").expect("Could not read input file");

    let mut position: i64 = 50;
    let mut count = 0u64;

    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let (dir, dist) = line.split_at(1);
        let dist: i64 = dist.parse().expect("Invalid distance");
        position = match dir {
            "R" => (position + dist).rem_euclid(100),
            "L" => (position - dist).rem_euclid(100),
            _ => panic!("Unknown direction"),
        };
        if position == 0 {
            count += 1;
        }
    }

    println!("{}", count);

    println!("Runtime: {:.6} seconds", start.elapsed().as_secs_f64());
}
