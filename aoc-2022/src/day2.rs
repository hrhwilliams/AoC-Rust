use std::error::Error;

const PROBLEM: &str = include_str!("input/day2.txt");

pub fn solution1() -> Result<(), Box<dyn Error + 'static>> {
    let total_score: u32 = PROBLEM
        .split('\n')
        .map(|line| -> u32 {
            match line {
                "A X" => 1 + 3, // rock vs rock         -> draw + 3
                "A Y" => 2 + 6, // rock vs paper        -> win  + 6
                "A Z" => 3,     // rock vs scissors     -> lose + 0
                "B X" => 1,     // paper vs rock        -> lose + 0
                "B Y" => 2 + 3, // paper vs paper       -> draw + 3
                "B Z" => 3 + 6, // paper vs scissors    -> win  + 6
                "C X" => 1 + 6, // scissors vs rock     -> win  + 6
                "C Y" => 2,     // scissors vs paper    -> lose + 0
                "C Z" => 3 + 3, // scissors vs scissors -> draw + 3
                &_ => panic!("Unexpected combination"),
            }
        })
        .sum();

    println!("Answer: {}", total_score);

    Ok(())
}

pub fn solution2() -> Result<(), Box<dyn Error + 'static>> {
    let total_score: u32 = PROBLEM
        .split('\n')
        .map(|line| -> u32 {
            match line {
                "A X" => 3,     // rock vs scissors -> lose + 0
                "A Y" => 1 + 3, // rock vs rock -> draw + 3
                "A Z" => 2 + 6, // rock vs paper -> win + 6
                "B X" => 1,     // paper vs rock -> lose + 0
                "B Y" => 2 + 3, // paper vs paper -> draw + 3
                "B Z" => 3 + 6, // paper vs scissors -> win + 6
                "C X" => 2,     // scissors vs paper -> lose + 0
                "C Y" => 3 + 3, // scissors vs scissors -> draw + 3
                "C Z" => 1 + 6, // scissors vs rock -> win + 6
                &_ => panic!("Unexpected combination"),
            }
        })
        .sum();

    println!("Answer: {}", total_score);

    Ok(())
}
