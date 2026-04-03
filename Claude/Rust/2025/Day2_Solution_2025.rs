use std::time::Instant;
use std::fs;

fn main() {
    let start = Instant::now();
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

    let mut total: u64 = 0;

    // Generate all "doubled" numbers: a number whose decimal form is some
    // sequence X concatenated with itself (e.g. 11, 6464, 123123).
    // No leading zeros means X itself has no leading zero, so X >= 10^(half_len-1).
    // We only need half_len 1..=5 since max input value fits in 10 digits.
    for half_len in 1u32..=5 {
        let half_start = 10u64.pow(half_len - 1);
        let half_end = 10u64.pow(half_len);
        let multiplier = half_end; // 10^half_len shifts left by half_len digits

        for half in half_start..half_end {
            let doubled = half * multiplier + half;
            for &(lo, hi) in &ranges {
                if doubled >= lo && doubled <= hi {
                    total += doubled;
                    break;
                }
            }
        }
    }

    println!("{}", total);

    println!("Runtime: {:.6} seconds", start.elapsed().as_secs_f64());
}
