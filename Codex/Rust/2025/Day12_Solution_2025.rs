use std::fs;

fn main() {
    let input = fs::read_to_string("Day12_Input.txt").expect("failed to read Day12_Input.txt");
    let mut shape_areas = Vec::new();
    let mut lines = input.lines().peekable();

    while let Some(line) = lines.peek().copied() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            lines.next();
            continue;
        }

        if trimmed.contains('x') {
            break;
        }

        if trimmed.ends_with(':') {
            lines.next();
            let mut area = 0_u64;
            while let Some(shape_line) = lines.peek().copied() {
                let shape_trimmed = shape_line.trim();
                if shape_trimmed.is_empty() {
                    lines.next();
                    break;
                }
                if shape_trimmed.ends_with(':') || shape_trimmed.contains('x') {
                    break;
                }
                area += shape_trimmed.bytes().filter(|&b| b == b'#').count() as u64;
                lines.next();
            }
            shape_areas.push(area);
            continue;
        }

        panic!("unexpected input format");
    }

    let mut answer = 0_u64;

    for line in lines {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        let (dims, counts_text) = trimmed.split_once(':').expect("invalid region line");
        let (width_text, height_text) = dims.split_once('x').expect("invalid dimensions");
        let width: u64 = width_text.parse().expect("invalid width");
        let height: u64 = height_text.parse().expect("invalid height");
        let counts: Vec<u64> = counts_text
            .split_whitespace()
            .map(|value| value.parse().expect("invalid count"))
            .collect();

        let needed_area: u64 = shape_areas
            .iter()
            .zip(counts.iter())
            .map(|(area, count)| area * count)
            .sum();

        if needed_area <= width * height {
            answer += 1;
        }
    }

    println!("{answer}");
}
