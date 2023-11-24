use std::error::Error;

const PROBLEM: &str = include_str!("input/day6.txt");

fn all_unique(chars: &[char]) -> bool {
    let mut hs = std::collections::HashSet::<char>::new();
    hs.extend(chars.iter());

    hs.len() == chars.len()
}

pub fn solution1() -> Result<(), Box<dyn Error + 'static>> {
    let chars: Vec<char> = PROBLEM.chars().collect();
    for i in 4..chars.len() {
        if all_unique(&chars[i-4..i]) {
            println!("Answer: {}", i);
            return Ok(());
        }
    }
    Ok(())
}

pub fn solution2() -> Result<(), Box<dyn Error + 'static>> {
    let chars: Vec<char> = PROBLEM.chars().collect();
    for i in 14..chars.len() {
        if all_unique(&chars[i-14..i]) {
            println!("Answer: {}", i);
            return Ok(());
        }
    }
    Ok(())
}
