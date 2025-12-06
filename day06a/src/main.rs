use std::time::Instant;

const SPACE: u8 = b' ';
const NEW_LINE: u8 = b'\n';

fn main() {
    let input = include_bytes!("../input.txt");

    let start = Instant::now();

    let mut solution = 0;

    let line_len = input.iter().position(|byte| byte == &NEW_LINE).unwrap() + 1;
    let lines = input.len().div_ceil(line_len);

    for symbol_idx in 0..line_len - 1 {
        let symbol = input[line_len * (lines - 1) + symbol_idx];

        if symbol == SPACE {
            continue;
        }

        let multiply = symbol == b'*';
        let mut sum = 0;

        for line_num in 0..lines - 1 {
            let mut checked_idx = (line_len * line_num) + symbol_idx;
            let mut found_num = false;
            let mut acc: usize = 0;

            loop {
                let checked_byte = &input[checked_idx as usize];

                if checked_byte == &NEW_LINE {
                    break;
                }
                if checked_byte == &SPACE {
                    checked_idx += 1;
                    if found_num {
                        break;
                    }
                    continue;
                }

                acc = acc * 10 + (*checked_byte - b'0') as usize;

                checked_idx += 1;
                found_num = true;
            }

            if multiply {
                if sum == 0 {
                    sum = 1;
                }
                sum *= acc;
            } else {
                sum += acc;
            }
        }

        solution += sum;
    }

    let duration = start.elapsed();

    println!("Password: {}", solution);
    println!("Time: {:.2?}", duration);
}
