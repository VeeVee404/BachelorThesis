use std::fs;

fn main() {
    let input = fs::read_to_string("Day1_Input.txt").expect("failed to read Day1_Input.txt");

    let mut position: i64 = 50;
    let mut count = 0_u64;

    for line in input.lines().filter(|line| !line.trim().is_empty()) {
        let direction = line.as_bytes()[0];
        let distance: i64 = line[1..]
            .trim()
            .parse()
            .expect("invalid rotation distance");

        let step = distance.rem_euclid(100);
        position = match direction {
            b'L' => (position - step).rem_euclid(100),
            b'R' => (position + step).rem_euclid(100),
            _ => panic!("invalid rotation direction"),
        };

        if position == 0 {
            count += 1;
        }
    }

    println!("{count}");
}
