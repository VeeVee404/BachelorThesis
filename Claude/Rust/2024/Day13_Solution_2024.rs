use std::time::Instant;
use std::fs;

fn solve(ax: i64, ay: i64, bx: i64, by: i64, px: i64, py: i64) -> Option<i64> {
    // Cramer's rule: solve ax*a + bx*b = px, ay*a + by*b = py
    let det = ax * by - ay * bx;
    if det == 0 {
        return None;
    }
    let a_num = px * by - py * bx;
    let b_num = ax * py - ay * px;
    if a_num % det != 0 || b_num % det != 0 {
        return None;
    }
    let a = a_num / det;
    let b = b_num / det;
    if a < 0 || b < 0 {
        return None;
    }
    Some(3 * a + b)
}

fn main() {
    let start = Instant::now();
    let input = fs::read_to_string("Day13_Input.txt").expect("Failed to read input file");

    let mut total = 0i64;
    let mut lines = input.lines().peekable();

    while lines.peek().is_some() {
        // Skip blank lines between machines
        let line_a = loop {
            match lines.next() {
                None => return println!("{}", total),
                Some(l) if l.trim().is_empty() => continue,
                Some(l) => break l,
            }
        };
        let line_b = lines.next().unwrap();
        let line_p = lines.next().unwrap();

        let parse_xy = |s: &str, sep: char| -> (i64, i64) {
            // e.g. "Button A: X+46, Y+89" or "Prize: X=5826, Y=7443"
            let s = s.split(':').nth(1).unwrap().trim();
            let mut parts = s.split(',');
            let x: i64 = parts
                .next().unwrap().trim()
                .trim_start_matches(|c: char| !c.is_ascii_digit() && c != '-')
                .parse().unwrap();
            let y: i64 = parts
                .next().unwrap().trim()
                .trim_start_matches(|c: char| !c.is_ascii_digit() && c != '-')
                .parse().unwrap();
            let _ = sep;
            (x, y)
        };

        let (ax, ay) = parse_xy(line_a, '+');
        let (bx, by) = parse_xy(line_b, '+');
        let (px, py) = parse_xy(line_p, '=');

        if let Some(cost) = solve(ax, ay, bx, by, px, py) {
            total += cost;
        }
    }

    println!("{}", total);

    println!("Runtime: {:.6} seconds", start.elapsed().as_secs_f64());
}

