use regex::Regex;
use std::error::Error;

const PROBLEM: &str = include_str!("input/day6.txt");

fn turn_off_1(lights: &mut [[bool; 1000]; 1000], x: usize, y: usize, u: usize, v: usize) {
    for i in x..=u {
        for j in y..=v {
            lights[i][j] = false;
        }
    }
}

fn turn_on_1(lights: &mut [[bool; 1000]; 1000], x: usize, y: usize, u: usize, v: usize) {
    for i in x..=u {
        for j in y..=v {
            lights[i][j] = true;
        }
    }
}

fn toggle_1(lights: &mut [[bool; 1000]; 1000], x: usize, y: usize, u: usize, v: usize) {
    for i in x..=u {
        for j in y..=v {
            lights[i][j] = !lights[i][j];
        }
    }
}

fn turn_off_2(lights: &mut [[i32; 1000]; 1000], x: usize, y: usize, u: usize, v: usize) {
    for i in x..=u {
        for j in y..=v {
            if lights[i][j] > 0 {
                lights[i][j] -= 1;
            }
        }
    }
}

fn turn_on_2(lights: &mut [[i32; 1000]; 1000], x: usize, y: usize, u: usize, v: usize) {
    for i in x..=u {
        for j in y..=v {
            lights[i][j] += 1;
        }
    }
}

fn toggle_2(lights: &mut [[i32; 1000]; 1000], x: usize, y: usize, u: usize, v: usize) {
    for i in x..=u {
        for j in y..=v {
            lights[i][j] += 2;
        }
    }
}

pub fn solution1() -> Result<(), Box<dyn Error + 'static>> {
    let mut lights = [[false; 1000]; 1000];
    let re = Regex::new(r"([0-9]+),([0-9]+) through ([0-9]+),([0-9]+)")
        .expect("Invalid regular expression");

    for line in PROBLEM.lines() {
        let caps = re.captures(line).unwrap();
        let x: usize = caps.get(1).unwrap().as_str().parse().expect("x");
        let y: usize = caps.get(2).unwrap().as_str().parse().expect("y");
        let u: usize = caps.get(3).unwrap().as_str().parse().expect("u");
        let v: usize = caps.get(4).unwrap().as_str().parse().expect("v");

        match &line[..7] {
            "turn on" => turn_on_1(&mut lights, x, y, u, v),
            "turn of" => turn_off_1(&mut lights, x, y, u, v),
            "toggle " => toggle_1(&mut lights, x, y, u, v),
            _ => unreachable!(),
        }
    }

    let lights_on: i32 = lights
        .iter()
        .map(|row| {
            row.iter()
                .map(|light| if *light { 1 } else { 0 })
                .sum::<i32>()
        })
        .sum();

    println!("Answer: {}", lights_on);

    Ok(())
}

pub fn solution2() -> Result<(), Box<dyn Error + 'static>> {
    let mut lights = [[0 as i32; 1000]; 1000];
    let re = regex::Regex::new(r"([0-9]+),([0-9]+) through ([0-9]+),([0-9]+)")
        .expect("Invalid regular expression");

    for line in PROBLEM.lines() {
        let caps = re.captures(line).unwrap();
        let x: usize = caps.get(1).unwrap().as_str().parse().expect("x");
        let y: usize = caps.get(2).unwrap().as_str().parse().expect("y");
        let u: usize = caps.get(3).unwrap().as_str().parse().expect("u");
        let v: usize = caps.get(4).unwrap().as_str().parse().expect("v");

        match &line[..7] {
            "turn on" => turn_on_2(&mut lights, x, y, u, v),
            "turn of" => turn_off_2(&mut lights, x, y, u, v),
            "toggle " => toggle_2(&mut lights, x, y, u, v),
            _ => unreachable!(),
        }
    }

    let lights_on: i32 = lights.iter().map(|row| row.iter().sum::<i32>()).sum();

    println!("Answer: {}", lights_on);

    Ok(())
}
