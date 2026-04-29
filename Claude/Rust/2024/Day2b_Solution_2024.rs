use std::fs;

fn is_safe(levels: &[i64]) -> bool {
    if levels.len() < 2 {
        return true;
    }
    let increasing = levels[1] > levels[0];
    levels.windows(2).all(|w| {
        let diff = w[1] - w[0];
        if increasing { diff >= 1 && diff <= 3 } else { diff <= -1 && diff >= -3 }
    })
}

fn is_safe_with_dampener(levels: &[i64]) -> bool {
    if is_safe(levels) {
        return true;
    }
    (0..levels.len()).any(|i| {
        let reduced: Vec<i64> = levels.iter().enumerate().filter(|&(j, _)| j != i).map(|(_,&v)| v).collect();
        is_safe(&reduced)
    })
}

fn main() {
    let input = fs::read_to_string("Day2_Input.txt").expect("Failed to read input file");

    let count = input.lines()
        .filter(|line| !line.is_empty())
        .filter(|line| {
            let levels: Vec<i64> = line.split_whitespace().map(|n| n.parse().unwrap()).collect();
            is_safe_with_dampener(&levels)
        })
        .count();

    println!("{}", count);
}
