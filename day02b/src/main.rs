use std::fs;
use std::time::Instant;

const POW10_LUT: [usize; 19] = [
    1,
    10,
    100,
    1_000,
    10_000,
    100_000,
    1_000_000,
    10_000_000,
    100_000_000,
    1_000_000_000,
    10_000_000_000,
    100_000_000_000,
    1_000_000_000_000,
    10_000_000_000_000,
    100_000_000_000_000,
    1_000_000_000_000_000,
    10_000_000_000_000_000,
    100_000_000_000_000_000,
    1_000_000_000_000_000_000,
];

fn main() {
    let input = fs::read_to_string("./input.txt").expect("No input file found");

    let start = Instant::now();

    let solution = input.split(",").fold(0, |mut acc, range| {
        let (start_str, end_str) = range.split_once("-").unwrap();

        let start = start_str.parse::<usize>().unwrap();
        let end = end_str.parse::<usize>().unwrap();

        for num in start..=end {
            let digits: usize = num.checked_ilog10().unwrap() as usize + 1;

            for pattern_len in 1..=(digits / 2) as usize {
                if digits % pattern_len != 0 {
                    continue;
                }
                let multiplier = POW10_LUT[pattern_len];
                let pattern = num / POW10_LUT[digits - pattern_len];

                let mut result = pattern;

                for _ in 0..(digits / pattern_len) - 1 {
                    result *= multiplier;
                    result += pattern;
                }

                if result == num {
                    acc += num;
                    break;
                }
            }
        }

        acc
    });
    let duration = start.elapsed();

    println!("Password: {}", solution);
    println!("Time: {:.2?}", duration);
}
