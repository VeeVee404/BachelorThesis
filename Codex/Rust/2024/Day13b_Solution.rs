use std::fs;

const OFFSET: i128 = 10_000_000_000_000;

#[derive(Clone, Copy)]
struct Machine {
    ax: i128,
    ay: i128,
    bx: i128,
    by: i128,
    px: i128,
    py: i128,
}

fn parse_button(line: &str) -> (i128, i128) {
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

fn parse_prize(line: &str) -> (i128, i128) {
    let (_, rest) = line.split_once(": ").expect("invalid prize line");
    let (x_part, y_part) = rest.split_once(", ").expect("invalid prize coordinates");
    let x = x_part
        .strip_prefix("X=")
        .expect("missing X=")
        .parse::<i128>()
        .expect("invalid prize X value");
    let y = y_part
        .strip_prefix("Y=")
        .expect("missing Y=")
        .parse::<i128>()
        .expect("invalid prize Y value");
    (x + OFFSET, y + OFFSET)
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

fn extended_gcd(a: i128, b: i128) -> (i128, i128, i128) {
    if b == 0 {
        (a.abs(), a.signum(), 0)
    } else {
        let (g, x1, y1) = extended_gcd(b, a.rem_euclid(b));
        let x = y1;
        let y = x1 - (a.div_euclid(b)) * y1;
        (g, x, y)
    }
}

fn ceil_div(a: i128, b: i128) -> i128 {
    debug_assert!(b > 0);
    a.div_euclid(b) + if a.rem_euclid(b) == 0 { 0 } else { 1 }
}

fn floor_div(a: i128, b: i128) -> i128 {
    debug_assert!(b > 0);
    a.div_euclid(b)
}

fn min_tokens_degenerate(machine: Machine) -> Option<i128> {
    if machine.ax * machine.py != machine.ay * machine.px {
        return None;
    }
    if machine.bx * machine.py != machine.by * machine.px {
        return None;
    }

    let (u, v, target) = if machine.ax != 0 || machine.bx != 0 || machine.px != 0 {
        (machine.ax, machine.bx, machine.px)
    } else {
        (machine.ay, machine.by, machine.py)
    };

    if u == 0 && v == 0 {
        return if target == 0 { Some(0) } else { None };
    }
    if u == 0 {
        return if target % v == 0 && target / v >= 0 {
            Some(target / v)
        } else {
            None
        };
    }
    if v == 0 {
        return if target % u == 0 && target / u >= 0 {
            Some(3 * (target / u))
        } else {
            None
        };
    }

    let (g, x0, y0) = extended_gcd(u, v);
    if target % g != 0 {
        return None;
    }

    let scale = target / g;
    let mut a0 = x0 * scale;
    let mut b0 = y0 * scale;
    let step_a = v / g;
    let step_b = u / g;

    let lower = ceil_div(-a0, step_a);
    let upper = floor_div(b0, step_b);
    if lower > upper {
        return None;
    }

    let slope = 3 * step_a - step_b;
    let t = if slope > 0 {
        lower
    } else if slope < 0 {
        upper
    } else {
        lower
    };

    a0 += step_a * t;
    b0 -= step_b * t;

    if a0 >= 0 && b0 >= 0 {
        Some(3 * a0 + b0)
    } else {
        None
    }
}

fn min_tokens(machine: Machine) -> Option<i128> {
    let det = machine.ax * machine.by - machine.ay * machine.bx;

    if det != 0 {
        let a_num = machine.px * machine.by - machine.py * machine.bx;
        let b_num = machine.ax * machine.py - machine.ay * machine.px;

        if a_num % det != 0 || b_num % det != 0 {
            return None;
        }

        let a = a_num / det;
        let b = b_num / det;

        if a < 0 || b < 0 {
            return None;
        }

        return Some(3 * a + b);
    }

    min_tokens_degenerate(machine)
}

fn main() {
    let input = fs::read_to_string("Day13_Input.txt").expect("failed to read Day13_Input.txt");
    let machines = parse_input(&input);

    let total_tokens: i128 = machines.into_iter().filter_map(min_tokens).sum();
    println!("{}", total_tokens);
}
