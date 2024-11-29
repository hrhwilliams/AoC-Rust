use std::error::Error;

const PROBLEM: &str = include_str!("input/day4.txt");

fn parse_pair(pair: &str) -> (u32, u32) {
    let dash = pair.find('-').expect("Missing '-'");
    let lhs: u32 = pair[0..dash]
        .parse::<u32>()
        .expect("Failed to parse to int");
    let rhs: u32 = pair[dash + 1..]
        .parse::<u32>()
        .expect("Failed to parse to int");
    (lhs, rhs)
}

pub fn solution1() -> Result<(), Box<dyn Error + 'static>> {
    let answer: u32 = PROBLEM
        .split('\n')
        .map(|line| {
            let comma = line.find(',').expect("Missing ','");
            let (first_lhs, first_rhs) = parse_pair(&line[0..comma]);
            let (second_lhs, second_rhs) = parse_pair(&line[comma + 1..]);

            if first_lhs <= second_lhs && first_rhs >= second_rhs {
                1
            } else {
                0
            }
        })
        .sum();

    println!("Answer: {}", answer);

    Ok(())
}

pub fn solution2() -> Result<(), Box<dyn Error + 'static>> {
    let answer: u32 = PROBLEM
        .split('\n')
        .map(|line| {
            let comma = line.find(',').expect("Missing ','");
            let (first_lhs, first_rhs) = parse_pair(&line[0..comma]);
            let (second_lhs, second_rhs) = parse_pair(&line[comma + 1..]);

            if first_rhs < second_lhs || first_lhs > second_rhs {
                0
            } else {
                1
            }
        })
        .sum();

    println!("Answer: {}", answer);

    Ok(())
}
