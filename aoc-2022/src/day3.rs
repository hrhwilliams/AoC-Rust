use std::error::Error;

const PROBLEM: &str = include_str!("input/day3.txt");

fn priority(c: char) -> Option<u32> {
    if c.is_ascii_alphabetic() {
        match c {
            'a'..='z' => Some(c as u32 - 'a' as u32 + 1),
            'A'..='Z' => Some(c as u32 - 'A' as u32 + 27),
            _ => None, // unreachable
        }
    } else {
        None
    }
}

pub fn solution1() -> Result<(), Box<dyn Error + 'static>> {
    let answer: u32 = PROBLEM
        .split('\n')
        .map(|rucksack: &str| -> u32 {
            let (first, second) = rucksack.split_at(rucksack.len() / 2);
            let mut set = std::collections::HashSet::new();
            set.extend(first.chars());

            for ch in second.chars() {
                if set.contains(&ch) {
                    return priority(ch).expect("Failed to convert item to priority");
                }
            }

            panic!("Expected rucksack to have one duplicate; found none");
        })
        .sum();

    println!("Answer: {}", answer);

    Ok(())
}

pub fn solution2() -> Result<(), Box<dyn Error + 'static>> {
    let rucksacks: Vec<String> = PROBLEM
        .split('\n')
        .map(|string| string.to_string())
        .collect();

    let mut i: usize = 0;
    let mut acc: u32 = 0;

    while i < rucksacks.len() {
        let mut r1 = std::collections::HashSet::<char>::new();
        r1.extend(rucksacks[i].chars());

        let mut r2 = std::collections::HashSet::<char>::new();
        r2.extend(rucksacks[i + 1].chars());

        for c in rucksacks[i + 2].chars() {
            if r1.contains(&c) && r2.contains(&c) {
                acc += priority(c).unwrap();
                break;
            }
        }
        i += 3;
    }

    println!("Answer: {}", acc);

    Ok(())
}
