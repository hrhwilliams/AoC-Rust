use std::error::Error;
mod day1;
mod day2;

fn main() -> Result<(), Box<dyn Error + 'static>> {
    day2::solution2()
}
