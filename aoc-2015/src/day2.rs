use regex::Regex;
use std::cmp::min;
use std::error::Error;

macro_rules! min {
    ($val:expr) => {
        $val
    };

    ($val:expr, $($vals:expr),+) => {{
        min($val, min! { $($vals),+ })
    }};
}

const PROBLEM: &str = include_str!("input/day2.txt");

pub fn solution1() -> Result<(), Box<dyn Error + 'static>> {
    let m = Regex::new(r"([0-9]+)x([0-9]+)x([0-9]+)").expect("Invalid regex");
    let mut accumulator: u32 = 0;

    for line in PROBLEM.lines() {
        let caps = m.captures(line).unwrap();
        let l: u32 = caps.get(1).unwrap().as_str().parse().expect("Length");
        let w: u32 = caps.get(2).unwrap().as_str().parse().expect("Width");
        let h: u32 = caps.get(3).unwrap().as_str().parse().expect("Height");

        accumulator += 2 * (l * w + l * h + h * w) + min!(l * w, w * h, h * l);
    }

    println!("Answer: {}", accumulator);
    Ok(())
}

pub fn solution2() -> Result<(), Box<dyn Error + 'static>> {
    let m = Regex::new(r"([0-9]+)x([0-9]+)x([0-9]+)").expect("Invalid regex");
    let mut accumulator: u32 = 0;

    for line in PROBLEM.lines() {
        let caps = m.captures(line).unwrap();
        let l: u32 = caps.get(1).unwrap().as_str().parse().expect("Length");
        let w: u32 = caps.get(2).unwrap().as_str().parse().expect("Width");
        let h: u32 = caps.get(3).unwrap().as_str().parse().expect("Height");

        let perimeter = 2 * min!(l + w, w + h, h + l);
        let bow = l * w * h;

        accumulator += perimeter + bow;
    }

    println!("Answer: {}", accumulator);
    Ok(())
}
