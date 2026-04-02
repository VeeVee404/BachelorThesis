use std::collections::HashMap;
use std::fs;

fn count_paths(
    node: &str,
    graph: &HashMap<String, Vec<String>>,
    memo: &mut HashMap<String, u128>,
) -> u128 {
    if node == "out" {
        return 1;
    }

    if let Some(&cached) = memo.get(node) {
        return cached;
    }

    let total = graph
        .get(node)
        .map(|neighbors| {
            neighbors
                .iter()
                .map(|next| count_paths(next, graph, memo))
                .sum()
        })
        .unwrap_or(0);

    memo.insert(node.to_string(), total);
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

    let mut memo: HashMap<String, u128> = HashMap::new();
    let answer = count_paths("you", &graph, &mut memo);
    println!("{answer}");
}
