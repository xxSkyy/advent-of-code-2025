use std::time::Instant;

const SEPARATOR: u8 = b'-';

fn main() {
    let input = include_str!("../input.txt");

    let start = Instant::now();

    let mut ranges: Vec<(usize, usize)> = vec![];
    let mut solution = 0;

    let mut range_mode = true;

    for line in input.lines() {
        if line.len() == 0 {
            range_mode = false;

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

            continue;
        }

        let mut current: usize = 0;

        if range_mode {
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

            continue;
        }

        for byte in line.as_bytes() {
            current = current * 10 + (byte - b'0') as usize;
        }

        for (range_min, range_max) in &ranges {
            if !(&current >= range_min && &current <= range_max) {
                continue;
            }
            solution += 1;
            break;
        }
    }

    let duration = start.elapsed();

    println!("Password: {}", solution);
    println!("Time: {:.2?}", duration);
}
