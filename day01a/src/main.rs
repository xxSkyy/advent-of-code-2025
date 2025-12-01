use std::fs;
use std::time::Instant;

fn main() {
    let input = fs::read_to_string("./input.txt").expect("No input file found");

    let start = Instant::now();

    let password = input.lines().fold([50, 0], |mut acc, line| {
        let dir = if line.starts_with("R") { 1 } else { -1 };

        acc[0] = (acc[0] + (line[1..].parse::<i32>().unwrap() * dir)).rem_euclid(100);

        if acc[0] == 0 {
            acc[1] += 1
        };

        acc
    })[1];

    let duration = start.elapsed();

    println!("Password: {}", password);
    println!("Time: {:.2?}", duration);
}
