use std::time::Instant;

const PAPER: u8 = b'@';
const BOUNDARY: u8 = 10;

fn main() {
    let input = include_bytes!("../input.txt");

    let start = Instant::now();
    let width: isize = input.iter().position(|x| x == &BOUNDARY).unwrap() as isize + 1;

    let movement_lut: [isize; 8] = [
        -width - 1,
        -width,
        -width + 1,
        -1,
        1,
        width - 1,
        width,
        width + 1,
    ];

    let solution = input.iter().enumerate().fold(0, |acc, (idx, point)| {
        if point != &PAPER {
            return acc;
        }

        let mut adjacent_rolls = 0;

        for movement in movement_lut.iter() {
            let moved_point = ((idx as isize) + movement) as usize;
            if idx < movement.abs() as usize && movement < &0 || moved_point >= input.len() {
                continue;
            }

            if input[moved_point] == PAPER {
                adjacent_rolls += 1;
            }
        }

        if adjacent_rolls < 4 {
            return acc + 1;
        }

        acc
    });

    let duration = start.elapsed();

    println!("Password: {}", solution);
    println!("Time: {:.2?}", duration);
}
