use std::fs;

fn main() {
    let input = fs::read_to_string("Day7_Input.txt")
        .or_else(|_| fs::read_to_string("Day7_input.txt"))
        .expect("failed to read Day7_Input.txt");

    let lines: Vec<&str> = input.lines().collect();
    if lines.is_empty() {
        println!("0");
        return;
    }

    let width = lines[0].len();
    let grid: Vec<&[u8]> = lines.iter().map(|line| line.as_bytes()).collect();

    let start_col = grid[0]
        .iter()
        .position(|&cell| cell == b'S')
        .expect("missing start position");

    let mut current = vec![0_u64; width];
    current[start_col] = 1;

    for row in &grid[1..] {
        let mut next = vec![0_u64; width];

        for col in 0..width {
            let count = current[col];
            if count == 0 {
                continue;
            }

            match row[col] {
                b'^' => {
                    if col > 0 {
                        next[col - 1] += count;
                    }
                    if col + 1 < width {
                        next[col + 1] += count;
                    }
                }
                b'.' | b'S' => {
                    next[col] += count;
                }
                _ => panic!("unexpected cell"),
            }
        }

        current = next;
    }

    let answer: u64 = current.into_iter().sum();
    println!("{answer}");
}
