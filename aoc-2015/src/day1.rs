use std::error::Error;

const PROBLEM: &str = include_str!("input/day1.txt");

pub fn solution1() -> Result<(), Box<dyn Error + 'static>> {
    let mut accumulator: i32 = 0;

    for ch in PROBLEM.chars() {
        match ch {
            '(' => accumulator += 1,
            ')' => accumulator -= 1,
            _ => panic!(),
        }
    }

    println!("Answer: {}", accumulator);

    Ok(())
}

pub fn solution2() -> Result<(), Box<dyn Error + 'static>> {
    let mut accumulator: i32 = 0;

    for (pos, ch) in PROBLEM.char_indices() {
        match ch {
            '(' => accumulator += 1,
            ')' => accumulator -= 1,
            _ => panic!(),
        }

        if accumulator < 0 {
            println!("Answer: {}", pos + 1);
            break;
        }
    }

    Ok(())
}
