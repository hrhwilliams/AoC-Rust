use std::error::Error;

const PROBLEM: &str = include_str!("day3.txt");

fn priority(c: char) -> Option<u32> {
    let ascii_value: u32 = c.into();

    if ascii_value >= 'a' as u32 && ascii_value <= 'z' as u32 {
        Some(ascii_value - 'a' as u32 + 1)
    } else if ascii_value >= 'A'.into() && ascii_value <= 'Z'.into() {
        Some(ascii_value - 'A' as u32 + 27)
    } else {
        None
    }
}

pub fn solution1() -> Result<(), Box<dyn Error + 'static>> {
    let answer: u32 = PROBLEM.split("\n").map(|rucksack: &str| -> u32 {
        let (first, second) = rucksack.split_at(rucksack.len() / 2);
        let mut set = std::collections::HashSet::new();
        set.extend(first.chars());

        for ch in second.chars() {
            if set.contains(&ch) {
                return priority(ch).expect("Failed to convert item to priority");
            }
        }

        panic!("Expected rucksack to have one duplicate; found none");
    }).sum();

    println!("Answer: {}", answer);

    Ok(())
}

pub fn solution2() -> Result<(), Box<dyn Error + 'static>> {
    Ok(())
}
