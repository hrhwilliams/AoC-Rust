use std::{
    collections::{HashSet, VecDeque},
    error::Error,
};

const PROBLEM: &str = include_str!("input/day1.txt");

pub fn solution1() -> Result<(), Box<dyn Error + 'static>> {
    let mut accumulator: i32 = 0;

    for line in PROBLEM.lines() {
        let inc: i32 = line.parse().expect("Parse");
        accumulator += inc
    }

    println!("Answer: {}", accumulator);

    Ok(())
}

pub fn solution2() -> Result<(), Box<dyn Error + 'static>> {
    let mut seen = HashSet::<i32>::new();
    let mut accumulator: i32 = 0;

    let mut queue: VecDeque<&str> = PROBLEM.lines().collect();

    loop {
        let line = queue.pop_front().unwrap();
        queue.push_back(line);

        let inc: i32 = line.parse().expect("Parse");
        accumulator += inc;

        if seen.contains(&accumulator) {
            break;
        } else {
            seen.insert(accumulator);
        }
    }

    println!("Answer: {}", accumulator);

    Ok(())
}
