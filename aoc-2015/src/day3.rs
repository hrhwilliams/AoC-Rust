use std::{collections::HashSet, error::Error};

const PROBLEM: &str = include_str!("input/day3.txt");

pub fn solution1() -> Result<(), Box<dyn Error + 'static>> {
    let mut visited = HashSet::<(i32, i32)>::new();
    let mut last_pos = (0, 0);
    visited.insert(last_pos);
    
    for ch in PROBLEM.chars() {
        match ch {
            '^' => last_pos = (last_pos.0 + 1, last_pos.1),
            '>' => last_pos = (last_pos.0, last_pos.1 + 1),
            'v' => last_pos = (last_pos.0 - 1, last_pos.1),
            '<' => last_pos = (last_pos.0, last_pos.1 - 1),
            _ => unreachable!(),
        }

        visited.insert(last_pos);
    }

    println!("Answer: {}", visited.len());
    Ok(())
}

pub fn solution2() -> Result<(), Box<dyn Error + 'static>> {
    let mut visited = HashSet::<(i32, i32)>::new();
    let mut santa_last_pos = (0, 0);
    let mut robo_santa_last_pos = (0, 0);
    let mut robo_santa: bool = false;
    
    visited.insert(santa_last_pos);

    for ch in PROBLEM.chars() {
        if robo_santa {
            match ch {
                '^' => robo_santa_last_pos = (robo_santa_last_pos.0 + 1, robo_santa_last_pos.1),
                '>' => robo_santa_last_pos = (robo_santa_last_pos.0, robo_santa_last_pos.1 + 1),
                'v' => robo_santa_last_pos = (robo_santa_last_pos.0 - 1, robo_santa_last_pos.1),
                '<' => robo_santa_last_pos = (robo_santa_last_pos.0, robo_santa_last_pos.1 - 1),
                _ => unreachable!(),
            }

            visited.insert(robo_santa_last_pos);
        } else {
            match ch {
                '^' => santa_last_pos = (santa_last_pos.0 + 1, santa_last_pos.1),
                '>' => santa_last_pos = (santa_last_pos.0, santa_last_pos.1 + 1),
                'v' => santa_last_pos = (santa_last_pos.0 - 1, santa_last_pos.1),
                '<' => santa_last_pos = (santa_last_pos.0, santa_last_pos.1 - 1),
                _ => unreachable!(),
            }

            visited.insert(santa_last_pos);
        }

        robo_santa = !robo_santa;
    }
    
    println!("Answer: {}", visited.len());
    Ok(())
}
