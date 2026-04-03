use std::time::Instant;
use std::fs;

fn main() {
    let start = Instant::now();
    let input = fs::read_to_string("Day25_Input.txt").expect("Failed to read input file");

    let mut locks: Vec<[u8; 5]> = Vec::new();
    let mut keys: Vec<[u8; 5]> = Vec::new();

    for block in input.split("\n\n") {
        let rows: Vec<&str> = block.lines().filter(|l| !l.is_empty()).collect();
        if rows.len() != 7 {
            continue;
        }
        let is_lock = rows[0] == "#####";
        let mut heights = [0u8; 5];
        // For locks: count '#' in rows 1..=6 per column (top row is all '#', skip it)
        // For keys:  count '#' in rows 0..=5 per column (bottom row is all '#', skip it)
        let range = if is_lock { 1..7 } else { 0..6 };
        for r in range {
            for (c, ch) in rows[r].chars().enumerate().take(5) {
                if ch == '#' {
                    heights[c] += 1;
                }
            }
        }
        if is_lock {
            locks.push(heights);
        } else {
            keys.push(heights);
        }
    }

    let mut count = 0usize;
    for lock in &locks {
        for key in &keys {
            if (0..5).all(|i| lock[i] + key[i] <= 5) {
                count += 1;
            }
        }
    }

    println!("{}", count);

    println!("Runtime: {:.6} seconds", start.elapsed().as_secs_f64());
}
