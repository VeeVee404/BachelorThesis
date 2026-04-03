use std::time::Instant;
use std::fs;

fn main() {
    let start = Instant::now();
    let content = fs::read_to_string("Day12_input.txt").expect("Could not read input file");
    let mut lines = content.lines().peekable();

    // --- Parse shape definitions and count filled cells per shape ---
    let mut cells_per_shape: Vec<usize> = Vec::new();

    loop {
        // Skip blank lines
        while lines.peek().map(|l| l.trim().is_empty()) == Some(true) {
            lines.next();
        }
        // A shape header looks like "0:", "1:", ...
        match lines.peek() {
            Some(&line) => {
                let t = line.trim();
                if t.ends_with(':') && t[..t.len() - 1].parse::<usize>().is_ok() {
                    lines.next(); // consume header
                    let mut count = 0usize;
                    // Read body rows until blank or non-shape line
                    loop {
                        match lines.peek() {
                            Some(&row) => {
                                let r = row.trim();
                                if r.is_empty() {
                                    break;
                                }
                                // Shape rows contain only '#' and '.'
                                if r.chars().all(|c| c == '#' || c == '.') {
                                    count += r.chars().filter(|&c| c == '#').count();
                                    lines.next();
                                } else {
                                    break;
                                }
                            }
                            None => break,
                        }
                    }
                    cells_per_shape.push(count);
                } else {
                    break; // reached region lines
                }
            }
            None => break,
        }
    }

    // --- Parse region lines and count how many can fit ---
    let mut fits = 0usize;

    for line in lines {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        // Format: "WxH: n0 n1 n2 ..."
        let (dims, rest) = match line.split_once(": ") {
            Some(p) => p,
            None => continue,
        };
        let (w_str, h_str) = match dims.split_once('x') {
            Some(p) => p,
            None => continue,
        };
        let w: usize = match w_str.parse() {
            Ok(v) => v,
            Err(_) => continue,
        };
        let h: usize = match h_str.parse() {
            Ok(v) => v,
            Err(_) => continue,
        };
        let area = w * h;

        let total: usize = rest
            .split_whitespace()
            .filter_map(|s| s.parse::<usize>().ok())
            .zip(cells_per_shape.iter())
            .map(|(count, &cell_size)| count * cell_size)
            .sum();

        if total <= area {
            fits += 1;
        }
    }

    println!("{}", fits);

    println!("Runtime: {:.6} seconds", start.elapsed().as_secs_f64());
}
