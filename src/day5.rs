use std::error::Error;

use regex::Regex;

const PROBLEM: &str = include_str!("input/day5.txt");

/* input:
 *     [M]             [Z]     [V]    
 *     [Z]     [P]     [L]     [Z] [J]
 * [S] [D]     [W]     [W]     [H] [Q]
 * [P] [V] [N] [D]     [P]     [C] [V]
 * [H] [B] [J] [V] [B] [M]     [N] [P]
 * [V] [F] [L] [Z] [C] [S] [P] [S] [G]
 * [F] [J] [M] [G] [R] [R] [H] [R] [L]
 * [G] [G] [G] [N] [V] [V] [T] [Q] [F]
 *  1   2   3   4   5   6   7   8   9 
*/
pub fn solution1() -> Result<(), Box<dyn Error + 'static>> {
    let move_re = Regex::new(r"move (?<n>\d+) from (?<src>\d+) to (?<dest>\d+)")
        .expect("Failed to parse regex");

    let mut stacks = vec![Vec::<char>::new(); 9];
    stacks[0] = vec!('G', 'F', 'V', 'H', 'P', 'S');
    stacks[1] = vec!('G', 'J', 'F', 'B', 'V', 'D', 'Z', 'M');
    stacks[2] = vec!('G', 'M', 'L', 'J', 'N');
    stacks[3] = vec!('N', 'G', 'Z', 'V', 'D', 'W', 'P');
    stacks[4] = vec!('V', 'R', 'C', 'B');
    stacks[5] = vec!('V', 'R', 'S', 'M', 'P', 'W', 'L', 'Z');
    stacks[6] = vec!('T', 'H', 'P');
    stacks[7] = vec!('Q', 'R', 'S', 'N', 'C', 'H', 'Z', 'V');
    stacks[8] = vec!('F', 'L', 'G', 'P', 'V', 'Q', 'J');

    for line in PROBLEM.split('\n') {
        if let Some(captures) = move_re.captures(line) {
            let n = captures["n"].parse::<usize>()
                .expect("Failed to parse n");
            let src = captures["src"].parse::<usize>()
                .expect("Failed to parse src") - 1;
            let dest = captures["dest"].parse::<usize>()
                .expect("Failed to parse dest") - 1;

            for _ in 0..n {
                let item = stacks[src].pop().expect("Tried to pop from empty stack");
                stacks[dest].push(item);
            }
        }
    }

    for stack in stacks {
        if let Some(top) = stack.last() {
            print!("{}",top);
        }
    }

    println!();

    Ok(())
}

pub fn solution2() -> Result<(), Box<dyn Error + 'static>> {
    let move_re = Regex::new(r"move (?<n>\d+) from (?<src>\d+) to (?<dest>\d+)")
        .expect("Failed to parse regex");

    let mut stacks = vec![Vec::<char>::new(); 9];
    stacks[0] = vec!('G', 'F', 'V', 'H', 'P', 'S');
    stacks[1] = vec!('G', 'J', 'F', 'B', 'V', 'D', 'Z', 'M');
    stacks[2] = vec!('G', 'M', 'L', 'J', 'N');
    stacks[3] = vec!('N', 'G', 'Z', 'V', 'D', 'W', 'P');
    stacks[4] = vec!('V', 'R', 'C', 'B');
    stacks[5] = vec!('V', 'R', 'S', 'M', 'P', 'W', 'L', 'Z');
    stacks[6] = vec!('T', 'H', 'P');
    stacks[7] = vec!('Q', 'R', 'S', 'N', 'C', 'H', 'Z', 'V');
    stacks[8] = vec!('F', 'L', 'G', 'P', 'V', 'Q', 'J');

    for line in PROBLEM.split('\n') {
        if let Some(captures) = move_re.captures(line) {
            let mut moved = Vec::<char>::new();
            let n = captures["n"].parse::<usize>()
                .expect("Failed to parse n");
            let src = captures["src"].parse::<usize>()
                .expect("Failed to parse src") - 1;
            let dest = captures["dest"].parse::<usize>()
                .expect("Failed to parse dest") - 1;

            for _ in 0..n {
                moved.push(stacks[src].pop().expect("Tried to pop from empty stack"));
            }

            for _ in 0..n {
                stacks[dest].push(moved.pop().unwrap());
            }
        }
    }

    for stack in stacks {
        if let Some(top) = stack.last() {
            print!("{}",top);
        }
    }

    println!();

    Ok(())
}
