use std::collections::HashMap;
use std::fs;

fn count_paths(
    node: &str,
    visited: u8,
    graph: &HashMap<String, Vec<String>>,
    memo: &mut HashMap<(String, u8), u64>,
) -> u64 {
    let mut state = visited;
    if node == "dac" {
        state |= 1;
    }
    if node == "fft" {
        state |= 2;
    }

    if node == "out" {
        return u64::from(state == 3);
    }

    let key = (node.to_string(), state);
    if let Some(&cached) = memo.get(&key) {
        return cached;
    }

    let total = graph
        .get(node)
        .map(|neighbors| {
            neighbors
                .iter()
                .map(|next| count_paths(next, state, graph, memo))
                .sum()
        })
        .unwrap_or(0);

    memo.insert(key, total);
    total
}

fn main() {
    let input = fs::read_to_string("Day11_Input.txt").expect("failed to read Day11_Input.txt");
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();

    for line in input.lines().filter(|line| !line.trim().is_empty()) {
        let (name, rest) = line.split_once(':').expect("invalid line format");
        let outputs = rest
            .split_whitespace()
            .map(str::to_string)
            .collect::<Vec<_>>();
        graph.insert(name.to_string(), outputs);
    }

    let mut memo: HashMap<(String, u8), u64> = HashMap::new();
    let answer = count_paths("svr", 0, &graph, &mut memo);
    println!("{answer}");
}
