use std::fs;

fn count_zeros_during(pos: i64, dir: &str, dist: i64) -> i64 {
    // First step where dial hits 0 when moving in direction dir from pos
    let k1 = match dir {
        "R" => if pos == 0 { 100 } else { 100 - pos },
        "L" => if pos == 0 { 100 } else { pos },
        _ => panic!("Unknown direction"),
    };
    if k1 > dist {
        0
    } else {
        (dist - k1) / 100 + 1
    }
}

fn main() {
    let content = fs::read_to_string("Day1_Input.txt").expect("Could not read input file");

    let mut position: i64 = 50;
    let mut count: i64 = 0;

    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let (dir, dist_str) = line.split_at(1);
        let dist: i64 = dist_str.parse().expect("Invalid distance");

        count += count_zeros_during(position, dir, dist);

        position = match dir {
            "R" => (position + dist).rem_euclid(100),
            "L" => (position - dist).rem_euclid(100),
            _ => panic!("Unknown direction"),
        };
    }

    println!("{}", count);
}
