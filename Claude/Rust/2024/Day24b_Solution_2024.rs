use std::collections::{HashMap, HashSet};
use std::fs;

fn main() {
    let input = fs::read_to_string("Day24_Input.txt").expect("Failed to read input");

    let mut gates: Vec<(String, String, String, String)> = Vec::new();
    let mut parsing_gates = false;

    for line in input.lines() {
        if line.trim().is_empty() {
            parsing_gates = true;
            continue;
        }
        if parsing_gates {
            let p: Vec<&str> = line.split_whitespace().collect();
            if p.len() >= 5 {
                gates.push((p[0].to_string(), p[1].to_string(), p[2].to_string(), p[4].to_string()));
            }
        }
    }

    // Map each wire to the indices of gates that consume it
    let mut consumers: HashMap<&str, Vec<usize>> = HashMap::new();
    for (i, (a, _, b, _)) in gates.iter().enumerate() {
        consumers.entry(a.as_str()).or_default().push(i);
        consumers.entry(b.as_str()).or_default().push(i);
    }

    let max_z_num = gates.iter()
        .filter_map(|(_, _, _, out)| out.strip_prefix('z').and_then(|s| s.parse::<u32>().ok()))
        .max()
        .unwrap_or(0);
    let max_z = format!("z{:02}", max_z_num);

    let mut wrong: HashSet<String> = HashSet::new();

    for (a, op, b, out) in &gates {
        let a_direct = a.starts_with('x') || a.starts_with('y');
        let b_direct = b.starts_with('x') || b.starts_with('y');
        let out_is_z  = out.starts_with('z');
        let is_bit0   = (a == "x00" && b == "y00") || (a == "y00" && b == "x00");

        // Rule 1: every z output (except the final carry) must come from XOR
        if out_is_z && out != &max_z && op != "XOR" {
            wrong.insert(out.clone());
        }

        // Rule 2: XOR gates whose inputs are not direct x/y bits must output to z
        if op == "XOR" && !a_direct && !b_direct && !out_is_z {
            wrong.insert(out.clone());
        }

        // Rule 3: AND gate output (except x00 AND y00) must only feed OR gates
        if op == "AND" && !is_bit0 {
            if let Some(cons) = consumers.get(out.as_str()) {
                for &ci in cons {
                    if gates[ci].1 != "OR" {
                        wrong.insert(out.clone());
                        break;
                    }
                }
            }
        }

        // Rule 4: OR gate must not directly output a z wire (except max_z, the final carry out)
        if op == "OR" && out_is_z && out != &max_z {
            wrong.insert(out.clone());
        }

        // Rule 5: XOR of direct input bits (except bit 0) must NOT output a z wire
        // (for i>0: x_i XOR y_i → half-sum, not z_i directly)
        if op == "XOR" && a_direct && b_direct && !is_bit0 && out_is_z {
            wrong.insert(out.clone());
        }
    }

    let mut result: Vec<String> = wrong.into_iter().collect();
    result.sort();
    println!("{}", result.join(","));
}
