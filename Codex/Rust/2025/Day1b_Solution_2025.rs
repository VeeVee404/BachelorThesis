use std::fs;

fn zero_hits(position: u32, direction: u8, distance: u128) -> u128 {
    let first = match direction {
        b'L' => {
            if position == 0 {
                100
            } else {
                position as u128
            }
        }
        b'R' => {
            let steps_to_zero = (100 - position) % 100;
            if steps_to_zero == 0 {
                100
            } else {
                steps_to_zero as u128
            }
        }
        _ => panic!("invalid rotation direction"),
    };

    if distance < first {
        0
    } else {
        1 + (distance - first) / 100
    }
}

fn main() {
    let input = fs::read_to_string("Day1_Input.txt").expect("failed to read Day1_Input.txt");

    let mut position: u32 = 50;
    let mut answer: u128 = 0;

    for line in input.lines().filter(|line| !line.trim().is_empty()) {
        let bytes = line.as_bytes();
        let direction = bytes[0];
        let distance: u128 = line[1..]
            .trim()
            .parse()
            .expect("invalid rotation distance");

        answer += zero_hits(position, direction, distance);

        let step = (distance % 100) as u32;
        position = match direction {
            b'L' => (position + 100 - step) % 100,
            b'R' => (position + step) % 100,
            _ => panic!("invalid rotation direction"),
        };
    }

    println!("{answer}");
}
