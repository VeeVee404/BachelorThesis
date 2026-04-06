use std::collections::HashMap;
use std::fs;

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
enum Op {
    And,
    Or,
    Xor,
}

#[derive(Clone)]
struct Gate {
    left: String,
    right: String,
    op: Op,
}

fn parse_op(text: &str) -> Op {
    match text {
        "AND" => Op::And,
        "OR" => Op::Or,
        "XOR" => Op::Xor,
        _ => panic!("unknown operation"),
    }
}

fn canonical_pair(left: &str, right: &str) -> (String, String) {
    if left <= right {
        (left.to_string(), right.to_string())
    } else {
        (right.to_string(), left.to_string())
    }
}

fn rebuild_lookup(gates: &HashMap<String, Gate>) -> HashMap<(Op, String, String), String> {
    let mut lookup = HashMap::new();

    for (output, gate) in gates {
        let (left, right) = canonical_pair(&gate.left, &gate.right);
        lookup.insert((gate.op, left, right), output.clone());
    }

    lookup
}

fn find_output(
    lookup: &HashMap<(Op, String, String), String>,
    op: Op,
    left: &str,
    right: &str,
) -> Option<String> {
    let (left, right) = canonical_pair(left, right);
    lookup.get(&(op, left, right)).cloned()
}

fn swap_outputs(
    gates: &mut HashMap<String, Gate>,
    first: &str,
    second: &str,
    swaps: &mut Vec<String>,
) {
    let first_gate = gates.get(first).expect("missing first swap output").clone();
    let second_gate = gates.get(second).expect("missing second swap output").clone();

    gates.insert(first.to_string(), second_gate);
    gates.insert(second.to_string(), first_gate);
    swaps.push(first.to_string());
    swaps.push(second.to_string());
}

fn main() {
    let input = fs::read_to_string("Day24_Input.txt").expect("failed to read Day24_Input.txt");
    let (_, gates_section) = input.split_once("\n\n").expect("invalid input");

    let mut gates: HashMap<String, Gate> = HashMap::new();
    let mut highest_z = 0usize;

    for line in gates_section.lines().filter(|line| !line.trim().is_empty()) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() != 5 || parts[3] != "->" {
            panic!("invalid gate line");
        }

        let output = parts[4].to_string();
        if let Some(index) = output.strip_prefix('z') {
            let value: usize = index.parse().expect("invalid z wire index");
            highest_z = highest_z.max(value);
        }

        gates.insert(
            output,
            Gate {
                left: parts[0].to_string(),
                right: parts[2].to_string(),
                op: parse_op(parts[1]),
            },
        );
    }

    let mut swaps = Vec::new();
    let mut lookup = rebuild_lookup(&gates);

    let mut carry = find_output(&lookup, Op::And, "x00", "y00").expect("missing initial carry");
    let z00 = find_output(&lookup, Op::Xor, "x00", "y00").expect("missing z00 gate");
    assert_eq!(z00, "z00");

    for bit in 1..highest_z {
        let x = format!("x{:02}", bit);
        let y = format!("y{:02}", bit);
        let z = format!("z{:02}", bit);

        let mut propagate = find_output(&lookup, Op::Xor, &x, &y).expect("missing propagate gate");
        let mut generate = find_output(&lookup, Op::And, &x, &y).expect("missing generate gate");

        let z_gate = gates.get(&z).expect("missing z output gate");
        let z_inputs_match_generate = z_gate.op == Op::Xor
            && ((z_gate.left == carry && z_gate.right == generate)
                || (z_gate.left == generate && z_gate.right == carry));

        if z_inputs_match_generate && propagate != generate {
            swap_outputs(&mut gates, &propagate, &generate, &mut swaps);
            lookup = rebuild_lookup(&gates);
            propagate = find_output(&lookup, Op::Xor, &x, &y).expect("missing propagate gate");
            generate = find_output(&lookup, Op::And, &x, &y).expect("missing generate gate");
        }

        let z_gate = gates.get(&z).expect("missing z output gate");
        let z_is_correct = z_gate.op == Op::Xor
            && ((z_gate.left == carry && z_gate.right == propagate)
                || (z_gate.left == propagate && z_gate.right == carry));

        if !z_is_correct {
            let correct_sum =
                find_output(&lookup, Op::Xor, &propagate, &carry).expect("missing sum gate");
            if correct_sum != z {
                swap_outputs(&mut gates, &z, &correct_sum, &mut swaps);
                lookup = rebuild_lookup(&gates);
                propagate = find_output(&lookup, Op::Xor, &x, &y).expect("missing propagate gate");
                generate = find_output(&lookup, Op::And, &x, &y).expect("missing generate gate");
            }
        }

        let carry_mix =
            find_output(&lookup, Op::And, &propagate, &carry).expect("missing carry-mix gate");
        carry = find_output(&lookup, Op::Or, &generate, &carry_mix).expect("missing carry gate");
    }

    assert_eq!(carry, format!("z{:02}", highest_z));

    swaps.sort();
    println!("{}", swaps.join(","));
}
