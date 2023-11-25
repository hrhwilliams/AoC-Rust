use std::error::Error;

const PROBLEM: &str = include_str!("input/day8.txt");

pub fn solution1() -> Result<(), Box<dyn Error + 'static>> {
    let trees: Vec<Vec<u32>> = PROBLEM.split('\n')
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect()
        })
        .collect();

    let mut visibility: Vec<Vec<u32>> = trees.iter()
        .map(|row| {
            vec![0; row.len()]
        })
        .collect();

    // visibility from left
    for i in 0..trees.len() {
        let mut tallest: i32 = -1;

        for j in 0..trees[i].len() {
            if trees[i][j] as i32 > tallest {
                tallest = trees[i][j] as i32;
                visibility[i][j] += 1;
            }
        }
    }

    // visibility from right
    for i in 0..trees.len() {
        let mut tallest: i32 = -1;

        for j in 0..trees[i].len() {
            if trees[i][trees[i].len() - j - 1] as i32 > tallest {
                tallest = trees[i][trees[i].len() - j - 1]  as i32;
                visibility[i][trees[i].len() - j - 1]  += 1;
            }
        }
    }

    // visibility from top
    for j in 0 .. trees[0].len() {
        let mut tallest: i32 = -1;

        for i in 0 .. trees.len() {
            if trees[i][j] as i32 > tallest {
                tallest = trees[i][j] as i32;
                visibility[i][j] += 1;
            }
        }
    }

    // // visibility from bottom
    for j in 0 .. trees[0].len() {
        let mut tallest: i32 = -1;

        for i in 0..trees.len() {
            if trees[trees.len() - i - 1][j] as i32 > tallest {
                tallest = trees[trees.len() - i - 1][j] as i32;
                visibility[trees.len() - i - 1][j] += 1;
            }
        }
    }

    let mut sum: u32 = 0;

    for i in 0 .. trees.len() {
        for j in 0 .. trees[i].len() {
            if visibility[i][j] > 0 {
                sum += 1;
            }
        }
    }

    println!("Answer: {}", sum);

    for row in visibility {
        for visible in row {
            print!("{}", visible);
        }
        println!();
    }

    Ok(())
}

pub fn solution2() -> Result<(), Box<dyn Error + 'static>> {
    let trees: Vec<Vec<u32>> = PROBLEM.split('\n')
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect()
        })
        .collect();

    let mut scenic_score: u32 = 0;

    for i in 0..trees.len() {
        for j in 0..trees[i].len() {
            let height = trees[i][j];
            let mut left: u32  = 0;
            let mut right: u32 = 0;
            let mut top: u32   = 0;
            let mut bot: u32   = 0;

            for l in 1..trees[i].len() - j {
                left += 1;
                if trees[i][j+l] >= height {
                    break;
                }
            }

            for r in 1..=j {
                right += 1;
                if trees[i][j-r] >= height {
                    break;
                }
            }

            for t in 1..trees.len() - i {
                top += 1;
                if trees[i+t][j] >= height {
                    break;
                }
            }

            for b in 1..=i {
                bot += 1;
                if trees[i-b][j] >= height {
                    break;
                }
            }

            let score = left * right * top * bot;
            if score > scenic_score {
                scenic_score = score;
            }
        }
    }

    println!("Answer: {}", scenic_score);

    Ok(())
}
