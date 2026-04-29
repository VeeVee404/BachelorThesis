use std::fs;

const OFFSET: i64 = 10_000_000_000_000;

fn parse_xy(line: &str) -> (i64, i64) {
    let line = line.trim();
    // Works for both "X+N, Y+N" and "X=N, Y=N"
    let after_x = line.find('X').unwrap() + 1;
    let rest = &line[after_x..];
    let comma = rest.find(',').unwrap();
    let x: i64 = rest[1..comma].trim().parse().unwrap();
    let after_y = rest.find('Y').unwrap() + 1;
    let y: i64 = rest[after_y + 1..].trim().parse().unwrap();
    (x, y)
}

fn solve(ax: i64, ay: i64, bx: i64, by: i64, px: i64, py: i64) -> Option<i64> {
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
    let input = fs::read_to_string("Day13_Input.txt").expect("Failed to read input file");
    let lines: Vec<&str> = input.lines().collect();
    let mut total = 0i64;
    let mut i = 0;

    while i < lines.len() {
        while i < lines.len() && lines[i].trim().is_empty() {
            i += 1;
        }
        if i + 2 >= lines.len() {
            break;
        }
        let (ax, ay) = parse_xy(lines[i]);
        let (bx, by) = parse_xy(lines[i + 1]);
        let (px, py) = parse_xy(lines[i + 2]);
        let px = px + OFFSET;
        let py = py + OFFSET;
        i += 3;

        if let Some(cost) = solve(ax, ay, bx, by, px, py) {
            total += cost;
        }
    }

    println!("{}", total);
}
