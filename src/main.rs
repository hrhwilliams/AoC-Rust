#![allow(dead_code)]

use std::error::Error;

mod day1;
mod day2;
mod day3;
mod day4;

fn main() -> Result<(), Box<dyn Error + 'static>> {
    day4::solution2()
}
