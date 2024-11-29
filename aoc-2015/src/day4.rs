use md5;
use std::error::Error;

const PROBLEM: &str = include_str!("input/day4.txt");

fn find_nonce_with_leading_zeros(input: &str, zeros: usize) -> u32 {
    let mut nonce = 0;

    loop {
        let hash = md5::compute(format!("{}{}", input, nonce));
        let hex_string = format!("{:?}", hash);

        if hex_string[0..zeros].chars().all(|x| x == '0') {
            break;
        } else {
            nonce += 1;
        }
    }

    nonce
}

pub fn solution1() -> Result<(), Box<dyn Error + 'static>> {
    println!("Answer: {}", find_nonce_with_leading_zeros(PROBLEM, 5));
    Ok(())
}

pub fn solution2() -> Result<(), Box<dyn Error + 'static>> {
    println!("Answer: {}", find_nonce_with_leading_zeros(PROBLEM, 6));
    Ok(())
}
