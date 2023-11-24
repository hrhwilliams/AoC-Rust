#![allow(dead_code)]

use std::error::Error;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

fn main() -> Result<(), Box<dyn Error + 'static>> {
    day7::solution2()
}
