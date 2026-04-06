use std::time::Instant;
use std::fs;

fn parse_heights(block: &str) -> [u8; 5] {
    let rows: Vec<&[u8]> = block
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.as_bytes())
        .collect();

    assert!(rows.len() == 7, "each schematic must have 7 rows");

    let mut heights = [0u8; 5];
    for col in 0..5 {
        let filled = rows.iter().filter(|row| row[col] == b'#').count();
        heights[col] = (filled - 1) as u8;
    }

    heights
}

fn main() {
    let start = Instant::now();
    let input = fs::read_to_string("Day25_Input.txt").expect("failed to read Day25_Input.txt");

    let mut locks: Vec<[u8; 5]> = Vec::new();
    let mut keys: Vec<[u8; 5]> = Vec::new();

    for block in input.split("\n\n").filter(|block| !block.trim().is_empty()) {
        let first_line = block.lines().next().expect("empty schematic block");
        let heights = parse_heights(block);

        if first_line.as_bytes().iter().all(|&cell| cell == b'#') {
            locks.push(heights);
        } else {
            keys.push(heights);
        }
    }

    let mut count: u64 = 0;
    for lock in &locks {
        for key in &keys {
            if (0..5).all(|i| lock[i] + key[i] <= 5) {
                count += 1;
            }
        }
    }

    println!("{count}");

    println!("Runtime: {:.6} seconds", start.elapsed().as_secs_f64());
}
