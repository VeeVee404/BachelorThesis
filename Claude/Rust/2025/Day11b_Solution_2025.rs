use std::fs;
use std::collections::HashMap;

fn count_paths(
    node: usize,
    flags: u8, // bit 0 = visited dac, bit 1 = visited fft
    graph: &[Vec<usize>],
    out_id: usize,
    dac_id: usize,
    fft_id: usize,
    memo: &mut HashMap<(usize, u8), u128>,
) -> u128 {
    // Update flags on arrival at this node.
    let flags = flags
        | if node == dac_id { 1 } else { 0 }
        | if node == fft_id { 2 } else { 0 };

    if node == out_id {
        return if flags == 3 { 1 } else { 0 };
    }

    if let Some(&cached) = memo.get(&(node, flags)) {
        return cached;
    }

    let count: u128 = graph[node]
        .iter()
        .map(|&child| count_paths(child, flags, graph, out_id, dac_id, fft_id, memo))
        .sum();

    memo.insert((node, flags), count);
    count
}

fn main() {
    let content = fs::read_to_string("Day11_Input.txt").expect("Could not read input file");

    let mut name_to_id: HashMap<String, usize> = HashMap::new();
    let mut raw_edges: Vec<(String, String)> = Vec::new();

    let get_id = |name: &str, map: &mut HashMap<String, usize>| {
        let next = map.len();
        *map.entry(name.to_string()).or_insert(next)
    };

    for line in content.lines().filter(|l| !l.is_empty()) {
        if let Some((node, rest)) = line.split_once(": ") {
            get_id(node, &mut name_to_id);
            for child in rest.split_whitespace() {
                get_id(child, &mut name_to_id);
                raw_edges.push((node.to_string(), child.to_string()));
            }
        }
    }

    let n = name_to_id.len();
    let mut graph: Vec<Vec<usize>> = vec![Vec::new(); n];
    for (src, dst) in raw_edges {
        graph[name_to_id[&src]].push(name_to_id[&dst]);
    }

    let svr_id = name_to_id["svr"];
    let out_id = name_to_id["out"];
    let dac_id = name_to_id["dac"];
    let fft_id = name_to_id["fft"];

    let mut memo: HashMap<(usize, u8), u128> = HashMap::new();
    let result = count_paths(svr_id, 0, &graph, out_id, dac_id, fft_id, &mut memo);

    println!("{}", result);
}
