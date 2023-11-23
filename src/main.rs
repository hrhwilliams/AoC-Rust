use std::error::Error;
mod day1;
mod day2;
mod day3;

fn main() -> Result<(), Box<dyn Error + 'static>> {
    day3::solution1()
}
