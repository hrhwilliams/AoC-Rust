use std::error::Error;

const PROBLEM: &str = include_str!("input/day11.txt");

fn valid_password(password: &str) -> bool {
    if password.len() != 8 {
        false
    } else if password.contains("i") || password.contains("l") || password.contains("o") {
        false
    } else {
        true
    }
}

pub fn solution1() -> Result<(), Box<dyn Error + 'static>> {
    Ok(())
}

pub fn solution2() -> Result<(), Box<dyn Error + 'static>> {
    Ok(())
}
