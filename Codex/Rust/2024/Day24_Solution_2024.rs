use std::collections::HashMap;
use std::fs;

#[derive(Clone)]
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

fn eval_wire(
    wire: &str,
    values: &mut HashMap<String, u8>,
    gates: &HashMap<String, Gate>,
) -> u8 {
    if let Some(&value) = values.get(wire) {
        return value;
    }

    let gate = gates.get(wire).expect("missing gate for wire");
    let left = eval_wire(&gate.left, values, gates);
    let right = eval_wire(&gate.right, values, gates);

    let value = match gate.op {
        Op::And => left & right,
        Op::Or => left | right,
        Op::Xor => left ^ right,
    };

    values.insert(wire.to_string(), value);
    value
}

fn main() {
    let input = fs::read_to_string("Day24_Input.txt").expect("failed to read Day24_Input.txt");
    let (initial_section, gates_section) = input.split_once("\n\n").expect("invalid input");

    let mut values: HashMap<String, u8> = HashMap::new();
    for line in initial_section.lines().filter(|line| !line.trim().is_empty()) {
        let (wire, value) = line.split_once(": ").expect("invalid wire assignment");
        let value = value.parse::<u8>().expect("invalid wire value");
        values.insert(wire.to_string(), value);
    }

    let mut gates: HashMap<String, Gate> = HashMap::new();
    let mut z_wires: Vec<String> = Vec::new();

    for line in gates_section.lines().filter(|line| !line.trim().is_empty()) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() != 5 || parts[3] != "->" {
            panic!("invalid gate line");
        }

        let op = match parts[1] {
            "AND" => Op::And,
            "OR" => Op::Or,
            "XOR" => Op::Xor,
            _ => panic!("unknown operation"),
        };

        let output = parts[4].to_string();
        if output.starts_with('z') {
            z_wires.push(output.clone());
        }

        gates.insert(
            output,
            Gate {
                left: parts[0].to_string(),
                right: parts[2].to_string(),
                op,
            },
        );
    }

    z_wires.sort();

    let mut result: u64 = 0;
    for wire in z_wires {
        let bit_index: u32 = wire[1..].parse().expect("invalid z wire index");
        let value = eval_wire(&wire, &mut values, &gates) as u64;
        result |= value << bit_index;
    }

    println!("{result}");
}
