use std::collections::VecDeque;
use std::fs;

fn main() {
    let input = fs::read_to_string("Day12_Input.txt").expect("Failed to read input file");
    let grid: Vec<Vec<u8>> = input.lines().filter(|l| !l.is_empty()).map(|l| l.bytes().collect()).collect();
    let rows = grid.len();
    let cols = grid[0].len();
    let mut visited = vec![vec![false; cols]; rows];

    let in_region = |r: i64, c: i64, ch: u8| -> bool {
        r >= 0 && c >= 0 && r < rows as i64 && c < cols as i64
            && grid[r as usize][c as usize] == ch
    };

    let mut total = 0u64;

    for sr in 0..rows {
        for sc in 0..cols {
            if visited[sr][sc] {
                continue;
            }
            let ch = grid[sr][sc];
            let mut region = Vec::new();
            let mut queue = VecDeque::new();
            queue.push_back((sr, sc));
            visited[sr][sc] = true;

            while let Some((r, c)) = queue.pop_front() {
                region.push((r, c));
                for (dr, dc) in [(-1i64, 0), (1, 0), (0, -1i64), (0, 1)] {
                    let nr = r as i64 + dr;
                    let nc = c as i64 + dc;
                    if in_region(nr, nc, ch) && !visited[nr as usize][nc as usize] {
                        visited[nr as usize][nc as usize] = true;
                        queue.push_back((nr as usize, nc as usize));
                    }
                }
            }

            let area = region.len() as u64;
            let mut corners = 0u64;

            for (r, c) in &region {
                let r = *r as i64;
                let c = *c as i64;
                for (dr, dc) in [(-1i64, -1i64), (-1, 1), (1, -1), (1, 1)] {
                    let n1 = in_region(r + dr, c, ch);
                    let n2 = in_region(r, c + dc, ch);
                    let d  = in_region(r + dr, c + dc, ch);
                    // convex corner: both orthogonal neighbors outside
                    if !n1 && !n2 {
                        corners += 1;
                    }
                    // concave corner: both orthogonal neighbors inside, diagonal outside
                    if n1 && n2 && !d {
                        corners += 1;
                    }
                }
            }

            total += area * corners;
        }
    }

    println!("{}", total);
}
