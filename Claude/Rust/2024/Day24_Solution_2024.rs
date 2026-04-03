use std::time::Instant;
use std::collections::HashMap;
use std::fs;

fn main() {
    let start = Instant::now();
    let input = fs::read_to_string("Day24_Input.txt").expect("Failed to read input file");
    let mut sections = input.splitn(2, "\n\n");

    // Parse initial wire values
    let mut wires: HashMap<String, u8> = HashMap::new();
    for line in sections.next().unwrap_or("").lines() {
        if let Some((name, val)) = line.split_once(':') {
            wires.insert(name.trim().to_string(), val.trim().parse().unwrap());
        }
    }

    // Parse gates: "A OP B -> C"
    let gates_str = sections.next().unwrap_or("");
    let mut gates: Vec<(String, String, String, String)> = Vec::new();
    for line in gates_str.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        // e.g. "x00 AND y00 -> z00"
        if let Some((lhs, out)) = line.split_once(" -> ") {
            let out = out.trim().to_string();
            let parts: Vec<&str> = lhs.split_whitespace().collect();
            if parts.len() == 3 {
                gates.push((
                    parts[0].to_string(),
                    parts[1].to_string(),
                    parts[2].to_string(),
                    out,
                ));
            }
        }
    }

    // Evaluate gates repeatedly until no progress
    let mut remaining = gates;
    loop {
        let mut unresolved = Vec::new();
        let mut progress = false;
        for (a, op, b, out) in remaining {
            match (wires.get(&a), wires.get(&b)) {
                (Some(&va), Some(&vb)) => {
                    let result = match op.as_str() {
                        "AND" => va & vb,
                        "OR"  => va | vb,
                        "XOR" => va ^ vb,
                        _     => panic!("Unknown op: {}", op),
                    };
                    wires.insert(out, result);
                    progress = true;
                }
                _ => unresolved.push((a, op, b, out)),
            }
        }
        remaining = unresolved;
        if !progress || remaining.is_empty() {
            break;
        }
    }

    // Collect z-wires and assemble the number (z00 = LSB)
    let mut z_bits: Vec<(u32, u8)> = wires
        .iter()
        .filter(|(k, _)| k.starts_with('z'))
        .map(|(k, &v)| (k[1..].parse::<u32>().unwrap(), v))
        .collect();
    z_bits.sort_by_key(|&(idx, _)| idx);

    let result: u64 = z_bits
        .iter()
        .fold(0u64, |acc, &(idx, bit)| acc | ((bit as u64) << idx));

    println!("{}", result);

    println!("Runtime: {:.6} seconds", start.elapsed().as_secs_f64());
}
