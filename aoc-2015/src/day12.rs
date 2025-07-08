use regex::Regex;
use std::error::Error;

const PROBLEM: &str = include_str!("input/day12.txt");

pub fn solution1() -> Result<(), Box<dyn Error + 'static>> {
    let re = Regex::new(r"-?[0-9]+").expect("invalid regex");

    let acc: i32 = re
        .find_iter(PROBLEM)
        .map(|m| m.as_str())
        .map(|x| x.parse::<i32>().expect("not a number"))
        .sum();

    println!("{}", acc);
    Ok(())
}

pub fn solution2() -> Result<(), Box<dyn Error + 'static>> {
    Ok(())
}
