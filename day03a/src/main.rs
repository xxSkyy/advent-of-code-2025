use std::time::Instant;

fn get_max_from_range(range: &[u8]) -> (usize, &u8) {
    range
        .iter()
        .enumerate()
        .rev()
        .max_by_key(|(_idx, val)| *val)
        .unwrap()
}

fn main() {
    let input = include_str!("../input.txt");

    let start = Instant::now();

    let solution = input.lines().fold(0_usize, |sum, line| {
        let line = line.as_bytes();
        let line_len = line.len();

        let (max_val_idx, max_val_byte) = get_max_from_range(&line[0..line_len - 1]);
        let (_, second_val_byte) = get_max_from_range(&line[max_val_idx + 1..line_len]);

        let max_val = ((max_val_byte - b'0') * 10) + second_val_byte - b'0';

        sum + max_val as usize
    });

    let duration = start.elapsed();

    println!("Password: {}", solution);
    println!("Time: {:.2?}", duration);
}
