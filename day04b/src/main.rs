use std::time::Instant;

const PAPER: u8 = 64;
const BOUNDARY: u8 = 10;
const EMPTY: u8 = 46;

fn main() {
    let mut input = include_bytes!("../input.txt").to_vec();

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

    let mut solution: u32 = 0;
    let mut remove_buffer: Vec<u32> = vec![];

    for idx in 0..input.len() {
        if input[idx] == EMPTY {
            continue;
        }

        let mut adjacent_rolls = 0;

        if input[idx] == PAPER {
            for movement in movement_lut.iter() {
                let moved_point = ((idx as isize) + movement) as usize;
                if idx < movement.abs() as usize && movement < &0 || moved_point >= input.len() {
                    continue;
                }

                if input[moved_point] == PAPER || input[moved_point] <= 8 {
                    adjacent_rolls += 1;
                }
            }

            input[idx] = adjacent_rolls;
        }

        adjacent_rolls = input[idx];

        if adjacent_rolls < 4 {
            remove_buffer.push(idx as u32);
        }
    }

    while let Some(idx) = remove_buffer.pop() {
        input[idx as usize] = EMPTY;

        for movement in movement_lut.iter() {
            let moved_point = ((idx as isize) + movement) as usize;

            if idx < (movement.abs() as u32) && movement < &0 || moved_point >= input.len() {
                continue;
            }

            if input[moved_point] <= 8 && input[moved_point] > 0 {
                input[moved_point] -= 1;
            }
            if input[moved_point] == 3 {
                remove_buffer.push(moved_point as u32);
            }
        }

        solution += 1;
    }

    let duration = start.elapsed();

    println!("Password: {}", solution);
    println!("Time: {:.2?}", duration);
}
