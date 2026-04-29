use std::fs;
use std::collections::HashSet;

fn main() {
    let content = fs::read_to_string("Day2_Input.txt").expect("Could not read input file");
    let content = content.trim();

    let ranges: Vec<(u64, u64)> = content
        .split(',')
        .filter(|s| !s.is_empty())
        .map(|range| {
            let (lo, hi) = range.split_once('-').expect("Invalid range format");
            (lo.parse().unwrap(), hi.parse().unwrap())
        })
        .collect();

    let mut invalid_ids: HashSet<u64> = HashSet::new();

    // Enumerate all numbers whose digit string is some pattern P repeated k>=2 times.
    // For total digit length `n` and pattern length `d` where d|n and d<n,
    // enumerate all d-digit patterns (no leading zeros) and build the repeated number.
    for total_len in 2u32..=10 {
        for d in 1..total_len {
            if total_len % d != 0 {
                continue;
            }
            let reps = total_len / d;
            let pat_start: u64 = if d == 1 { 1 } else { 10u64.pow(d - 1) };
            let pat_end: u64 = 10u64.pow(d);
            let shift: u64 = pat_end; // multiply by 10^d to shift left by d digits

            for pat in pat_start..pat_end {
                // Build: pat repeated `reps` times
                let mut num = pat;
                for _ in 1..reps {
                    num = num * shift + pat;
                }

                for &(lo, hi) in &ranges {
                    if num >= lo && num <= hi {
                        invalid_ids.insert(num);
                        break;
                    }
                }
            }
        }
    }

    let total: u64 = invalid_ids.iter().sum();
    println!("{}", total);
}
