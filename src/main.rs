use std::error::Error;
mod day1;

fn main() -> Result<(), Box<dyn Error + 'static>> {
    day1::solution2()
}
