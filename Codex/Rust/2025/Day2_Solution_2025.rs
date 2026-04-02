use std::fs;

fn parse_ranges(input: &str) -> Vec<(u64, u64)> {
    let mut ranges = Vec::new();

    for part in input.trim().split(',').filter(|s| !s.is_empty()) {
        let (start, end) = part
            .split_once('-')
            .expect("each range must contain a dash");
        let start: u64 = start.parse().expect("invalid range start");
        let end: u64 = end.parse().expect("invalid range end");
        ranges.push((start, end));
    }

    ranges
}

fn merge_ranges(mut ranges: Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    if ranges.is_empty() {
        return ranges;
    }

    ranges.sort_unstable();

    let mut merged = Vec::with_capacity(ranges.len());
    let mut current = ranges[0];

    for &(start, end) in &ranges[1..] {
        if start <= current.1.saturating_add(1) {
            current.1 = current.1.max(end);
        } else {
            merged.push(current);
            current = (start, end);
        }
    }

    merged.push(current);
    merged
}

fn generate_invalid_ids(max_value: u64) -> Vec<u64> {
    let mut values = Vec::new();
    let mut power10 = 10_u64;

    for _half_len in 1..=10 {
        let start = power10 / 10;
        let end = power10 - 1;

        for half in start..=end {
            let value = half * power10 + half;
            if value > max_value {
                return values;
            }
            values.push(value);
        }

        if power10 > max_value {
            break;
        }
        power10 = match power10.checked_mul(10) {
            Some(next) => next,
            None => break,
        };
    }

    values
}

fn main() {
    let input = fs::read_to_string("Day2_Input.txt").expect("failed to read Day2_Input.txt");
    let ranges = parse_ranges(&input);
    let merged_ranges = merge_ranges(ranges);
    let max_end = merged_ranges.iter().map(|&(_, end)| end).max().unwrap_or(0);
    let invalid_ids = generate_invalid_ids(max_end);

    let mut sum: u128 = 0;
    let mut idx = 0usize;

    for (start, end) in merged_ranges {
        while idx < invalid_ids.len() && invalid_ids[idx] < start {
            idx += 1;
        }

        while idx < invalid_ids.len() && invalid_ids[idx] <= end {
            sum += invalid_ids[idx] as u128;
            idx += 1;
        }
    }

    println!("{sum}");
}
