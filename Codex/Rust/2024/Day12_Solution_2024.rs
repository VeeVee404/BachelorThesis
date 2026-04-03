use std::collections::VecDeque;
use std::fs;

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

    let mut total_price: u64 = 0;

    for r in 0..rows {
        for c in 0..cols {
            if visited[r][c] {
                continue;
            }

            let plant = grid[r][c];
            let mut queue = VecDeque::new();
            queue.push_back((r, c));
            visited[r][c] = true;

            let mut area: u64 = 0;
            let mut perimeter: u64 = 0;

            while let Some((cr, cc)) = queue.pop_front() {
                area += 1;

                for (dr, dc) in directions {
                    let nr = cr as isize + dr;
                    let nc = cc as isize + dc;

                    if nr < 0 || nr >= rows as isize || nc < 0 || nc >= cols as isize {
                        perimeter += 1;
                        continue;
                    }

                    let nr = nr as usize;
                    let nc = nc as usize;

                    if grid[nr][nc] != plant {
                        perimeter += 1;
                    } else if !visited[nr][nc] {
                        visited[nr][nc] = true;
                        queue.push_back((nr, nc));
                    }
                }
            }

            total_price += area * perimeter;
        }
    }

    println!("{total_price}");
}
