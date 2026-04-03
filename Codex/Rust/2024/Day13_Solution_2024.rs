use std::time::Instant;
use std::fs;

#[derive(Clone, Copy)]
struct Machine {
    ax: i64,
    ay: i64,
    bx: i64,
    by: i64,
    px: i64,
    py: i64,
}

fn parse_button(line: &str) -> (i64, i64) {
    let (_, rest) = line.split_once(": ").expect("invalid button line");
    let (x_part, y_part) = rest.split_once(", ").expect("invalid button coordinates");
    let x = x_part
        .strip_prefix("X+")
        .expect("missing X+")
        .parse()
        .expect("invalid button X value");
    let y = y_part
        .strip_prefix("Y+")
        .expect("missing Y+")
        .parse()
        .expect("invalid button Y value");
    (x, y)
}

fn parse_prize(line: &str) -> (i64, i64) {
    let (_, rest) = line.split_once(": ").expect("invalid prize line");
    let (x_part, y_part) = rest.split_once(", ").expect("invalid prize coordinates");
    let x = x_part
        .strip_prefix("X=")
        .expect("missing X=")
        .parse()
        .expect("invalid prize X value");
    let y = y_part
        .strip_prefix("Y=")
        .expect("missing Y=")
        .parse()
        .expect("invalid prize Y value");
    (x, y)
}

fn parse_input(input: &str) -> Vec<Machine> {
    input
        .split("\n\n")
        .filter(|block| !block.trim().is_empty())
        .map(|block| {
            let mut lines = block.lines().filter(|line| !line.trim().is_empty());
            let (ax, ay) = parse_button(lines.next().expect("missing button A line"));
            let (bx, by) = parse_button(lines.next().expect("missing button B line"));
            let (px, py) = parse_prize(lines.next().expect("missing prize line"));
            Machine {
                ax,
                ay,
                bx,
                by,
                px,
                py,
            }
        })
        .collect()
}

fn min_tokens(machine: Machine) -> Option<i64> {
    let mut best: Option<i64> = None;

    for a_presses in 0..=100 {
        for b_presses in 0..=100 {
            if a_presses * machine.ax + b_presses * machine.bx == machine.px
                && a_presses * machine.ay + b_presses * machine.by == machine.py
            {
                let cost = a_presses * 3 + b_presses;
                best = Some(best.map_or(cost, |current| current.min(cost)));
            }
        }
    }

    best
}

fn main() {
    let start = Instant::now();
    let input = fs::read_to_string("Day13_Input.txt").expect("failed to read Day13_Input.txt");
    let machines = parse_input(&input);

    let total_tokens: i64 = machines.into_iter().filter_map(min_tokens).sum();

    println!("{total_tokens}");

    println!("Runtime: {:.6} seconds", start.elapsed().as_secs_f64());
}
