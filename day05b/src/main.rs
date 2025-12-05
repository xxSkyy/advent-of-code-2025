use std::time::Instant;

const SEPARATOR: u8 = b'-';

fn main() {
    let input = include_str!("../input.txt");

    let start = Instant::now();

    let mut ranges: Vec<(usize, usize)> = vec![];
    let mut solution = 0;

    for line in input.lines() {
        if line.len() == 0 {
            ranges.sort_unstable_by_key(|(range_min, _)| *range_min);

            let mut i = 1;

            while i < ranges.len() {
                if !(ranges[i - 1].1 >= ranges[i].0 - 1) {
                    i += 1;
                    continue;
                }

                ranges[i - 1].1 = ranges[i - 1].1.max(ranges[i].1);
                ranges.remove(i);
            }

            ranges.iter().for_each(|(min, max)| {
                solution += max - min + 1;
            });

            break;
        }

        let mut current: usize = 0;

        let mut range_start: usize = 0;
        for byte in line.as_bytes() {
            if byte == &SEPARATOR {
                range_start = current;
                current = 0;
                continue;
            }

            current = current * 10 + (byte - b'0') as usize;
        }
        ranges.push((range_start, current));
    }

    let duration = start.elapsed();

    println!("Password: {}", solution);
    println!("Time: {:.2?}", duration);
}
