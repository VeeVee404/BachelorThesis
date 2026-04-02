use std::fs;

fn main() {
    let input = fs::read_to_string("Day7_input.txt").expect("failed to read Day7_input.txt");
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

    let mut current = vec![false; width];
    current[start_col] = true;

    let mut splits: u64 = 0;

    for row in &grid[1..] {
        let mut next = vec![false; width];

        for col in 0..width {
            if !current[col] {
                continue;
            }

            match row[col] {
                b'^' => {
                    splits += 1;
                    if col > 0 {
                        next[col - 1] = true;
                    }
                    if col + 1 < width {
                        next[col + 1] = true;
                    }
                }
                b'.' | b'S' => {
                    next[col] = true;
                }
                _ => panic!("unexpected cell"),
            }
        }

        current = next;
    }

    println!("{splits}");
}
