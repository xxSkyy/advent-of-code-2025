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

        let mut val: usize = 0;
        let mut index: usize = 0;

        for num in 0..12 {
            let (max_val_idx, max_val_byte) = get_max_from_range(&line[index..line_len - 11 + num]);

            index += max_val_idx + 1;
            val = (val * 10) + (max_val_byte - b'0') as usize;
        }

        sum + val as usize
    });

    let duration = start.elapsed();

    println!("Password: {}", solution);
    println!("Time: {:.2?}", duration);
}
