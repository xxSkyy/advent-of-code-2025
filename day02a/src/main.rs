use std::fs;
use std::time::Instant;

fn main() {
    let input = fs::read_to_string("./input.txt").expect("No input file found");

    let start = Instant::now();

    let solution = input.split(",").fold(0, |mut acc, range| {
        let (start_str, end_str) = range.split_once("-").unwrap();

        let start = start_str.parse::<usize>().unwrap();
        let end = end_str.parse::<usize>().unwrap();

        for num in start..=end {
            let digits = num.checked_ilog10().unwrap() + 1;
            if digits % 2 != 0 {
                continue;
            }

            let divisor = 10_usize.pow(digits / 2);

            if num / divisor == num % divisor {
                acc += num;
            }
        }

        acc
    });
    let duration = start.elapsed();

    println!("Password: {}", solution);
    println!("Time: {:.2?}", duration);
}
