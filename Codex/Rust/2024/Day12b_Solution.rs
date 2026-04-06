use std::collections::{HashMap, VecDeque};
use std::fs;

fn count_runs(lines: HashMap<usize, Vec<usize>>) -> u64 {
    let mut sides = 0u64;

    for mut positions in lines.into_values() {
        positions.sort_unstable();

        let mut previous: Option<usize> = None;
        for position in positions {
            if previous.is_none_or(|prev| position != prev + 1) {
                sides += 1;
            }
            previous = Some(position);
        }
    }

    sides
}

fn main() {
    let input = fs::read_to_string("Day12_Input.txt").expect("failed to read Day12_Input.txt");
    let grid: Vec<Vec<u8>> = input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.as_bytes().to_vec())
        .collect();

    let rows = grid.len();
    let cols = grid.first().map_or(0, Vec::len);
    let mut visited = vec![vec![false; cols]; rows];
    let directions = [(-1isize, 0isize), (1, 0), (0, -1), (0, 1)];

    let mut total_price = 0u64;

    for r in 0..rows {
        for c in 0..cols {
            if visited[r][c] {
                continue;
            }

            let plant = grid[r][c];
            let mut queue = VecDeque::new();
            queue.push_back((r, c));
            visited[r][c] = true;

            let mut area = 0u64;
            let mut top_edges: HashMap<usize, Vec<usize>> = HashMap::new();
            let mut bottom_edges: HashMap<usize, Vec<usize>> = HashMap::new();
            let mut left_edges: HashMap<usize, Vec<usize>> = HashMap::new();
            let mut right_edges: HashMap<usize, Vec<usize>> = HashMap::new();

            while let Some((cr, cc)) = queue.pop_front() {
                area += 1;

                for (dr, dc) in directions {
                    let nr = cr as isize + dr;
                    let nc = cc as isize + dc;

                    let same_region = if nr < 0 || nr >= rows as isize || nc < 0 || nc >= cols as isize {
                        false
                    } else {
                        grid[nr as usize][nc as usize] == plant
                    };

                    if !same_region {
                        match (dr, dc) {
                            (-1, 0) => top_edges.entry(cr).or_default().push(cc),
                            (1, 0) => bottom_edges.entry(cr + 1).or_default().push(cc),
                            (0, -1) => left_edges.entry(cc).or_default().push(cr),
                            (0, 1) => right_edges.entry(cc + 1).or_default().push(cr),
                            _ => unreachable!(),
                        }
                        continue;
                    }

                    let nr = nr as usize;
                    let nc = nc as usize;
                    if !visited[nr][nc] {
                        visited[nr][nc] = true;
                        queue.push_back((nr, nc));
                    }
                }
            }

            let sides = count_runs(top_edges)
                + count_runs(bottom_edges)
                + count_runs(left_edges)
                + count_runs(right_edges);

            total_price += area * sides;
        }
    }

    println!("{}", total_price);
}
