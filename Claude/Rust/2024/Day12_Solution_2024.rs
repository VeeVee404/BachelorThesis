use std::time::Instant;
use std::collections::VecDeque;
use std::fs;

fn main() {
    let start = Instant::now();
    let input = fs::read_to_string("Day12_Input.txt").expect("Failed to read input file");

    let grid: Vec<Vec<u8>> = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.bytes().collect())
        .collect();

    let rows = grid.len();
    let cols = grid[0].len();
    let mut visited = vec![vec![false; cols]; rows];
    let dirs: [(i64, i64); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    let mut total = 0u64;

    for sr in 0..rows {
        for sc in 0..cols {
            if visited[sr][sc] {
                continue;
            }
            let plant = grid[sr][sc];
            let mut area = 0u64;
            let mut perimeter = 0u64;
            let mut queue = VecDeque::new();
            queue.push_back((sr, sc));
            visited[sr][sc] = true;

            while let Some((r, c)) = queue.pop_front() {
                area += 1;
                for (dr, dc) in &dirs {
                    let nr = r as i64 + dr;
                    let nc = c as i64 + dc;
                    if nr < 0 || nr >= rows as i64 || nc < 0 || nc >= cols as i64 {
                        perimeter += 1;
                    } else {
                        let (nr, nc) = (nr as usize, nc as usize);
                        if grid[nr][nc] != plant {
                            perimeter += 1;
                        } else if !visited[nr][nc] {
                            visited[nr][nc] = true;
                            queue.push_back((nr, nc));
                        }
                    }
                }
            }

            total += area * perimeter;
        }
    }

    println!("{}", total);

    println!("Runtime: {:.6} seconds", start.elapsed().as_secs_f64());
}
