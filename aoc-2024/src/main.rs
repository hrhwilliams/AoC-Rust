#![allow(dead_code)]

use aoc_2024::*;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error + 'static>> {
    println!("Answer: {}", day1::solution2()?);
    Ok(())
}
