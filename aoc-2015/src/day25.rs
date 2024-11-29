use regex::Regex;
use std::error::Error;

const PROBLEM: &str = include_str!("input/day25.txt");

fn tri(row: u32, col:u32) -> u32 {
    // tri(1, col) = (col * (col + 1)) / 2
    // tri(2, col) = tri(1, col + 1) - 1
    // tri(3, col) = tri(2, col + 1) - 1 = tri(1, col + 2) - 2
    // tri(row, col) = tri(1, col + row - 1) - row + 1

    let n = col + row - 1;
    (n * (n + 1)) / 2 - (row - 1)
}

pub fn solution1() -> Result<(), Box<dyn Error + 'static>> {
    let re = Regex::new(r"row ([0-9]+), column ([0-9]+)").expect("Invalid regular expression");
    let caps = re.captures(PROBLEM).unwrap();
    let row: u32 = caps.get(1).unwrap().as_str().parse().expect("row");
    let col: u32 = caps.get(2).unwrap().as_str().parse().expect("col");
    let iter = tri(row, col);
    let mut accumulator: u64 = 20151125;

    for _ in 1..iter {
        accumulator = (accumulator * 252533) % 33554393;
    }

    println!("Answer: {}", accumulator);

    Ok(())
}

pub fn solution2() -> Result<(), Box<dyn Error + 'static>> {
    Ok(())
}