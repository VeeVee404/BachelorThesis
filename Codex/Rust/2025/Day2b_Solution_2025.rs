use std::fs;

fn parse_ranges(input: &str) -> Vec<(u64, u64)> {
    let mut ranges = Vec::new();

    for part in input.trim().split(",") {
        if part.is_empty() {
            continue;
        }

        let (start, end) = part
            .split_once("-")
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

fn repeat_block(block: u64, repeat_count: usize) -> u64 {
    let block_str = block.to_string();
    let mut value = 0_u64;

    for _ in 0..repeat_count {
        for &byte in block_str.as_bytes() {
            value = value * 10 + u64::from(byte - 48);
        }
    }

    value
}

fn generate_invalid_ids(max_value: u64) -> Vec<u64> {
    let max_digits = max_value.to_string().len();
    let mut values = Vec::new();

    for total_len in 2..=max_digits {
        for block_len in 1..=(total_len / 2) {
            if total_len % block_len != 0 {
                continue;
            }

            let repeat_count = total_len / block_len;
            let start = 10_u64.pow((block_len - 1) as u32);
            let end = 10_u64.pow(block_len as u32) - 1;

            for block in start..=end {
                let value = repeat_block(block, repeat_count);
                if value <= max_value {
                    values.push(value);
                }
            }
        }
    }

    values.sort_unstable();
    values.dedup();
    values
}

fn main() {
    let input = fs::read_to_string("Day2_Input.txt").expect("failed to read Day2_Input.txt");
    let ranges = merge_ranges(parse_ranges(&input));
    let max_end = ranges.iter().map(|&(_, end)| end).max().unwrap_or(0);
    let invalid_ids = generate_invalid_ids(max_end);

    let mut sum = 0_u128;
    let mut idx = 0_usize;

    for (start, end) in ranges {
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
