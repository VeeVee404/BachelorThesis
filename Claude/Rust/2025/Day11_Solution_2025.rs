use std::collections::HashMap;
use std::fs;

fn count_paths(node: &str, graph: &HashMap<String, Vec<String>>, memo: &mut HashMap<String, u128>) -> u128 {
    if node == "out" {
        return 1;
    }
    if let Some(&cached) = memo.get(node) {
        return cached;
    }
    let count = match graph.get(node) {
        Some(children) => children
            .iter()
            .map(|child| count_paths(child, graph, memo))
            .sum(),
        None => 0,
    };
    memo.insert(node.to_string(), count);
    count
}

fn main() {
    let content = fs::read_to_string("Day11_input.txt").expect("Could not read input file");

    let mut graph: HashMap<String, Vec<String>> = HashMap::new();

    for line in content.lines().filter(|l| !l.is_empty()) {
        if let Some((node, rest)) = line.split_once(": ") {
            let children: Vec<String> = rest.split_whitespace().map(str::to_string).collect();
            graph.insert(node.to_string(), children);
        }
    }

    let mut memo: HashMap<String, u128> = HashMap::new();
    let result = count_paths("you", &graph, &mut memo);

    println!("{}", result);
}
