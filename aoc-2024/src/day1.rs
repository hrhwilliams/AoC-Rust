use std::error::Error;

const PROBLEM: &str = include_str!("input/day1.txt");

trait Count {
    fn count(&self, n: i32) -> usize;
}

impl Count for Vec<i32> {
    fn count(&self, n: i32) -> usize {
        // let idx = self.binary_search(|x| x < &n);
        let low = self.partition_point(|x| x < &n);
        let high = self.partition_point(|x| x <= &n);
        high - low
    }
}

pub fn solution1() -> Result<i32, Box<dyn Error + 'static>> {
    let mut left: Vec<i32> = vec![];
    let mut right: Vec<i32> = vec![];

    PROBLEM.lines().for_each(|line| {
        let v: Vec<i32> = line
            .split_ascii_whitespace()
            .map(|l| l.parse::<i32>().expect("parse"))
            .collect();
        assert!(v.len() == 2);
        left.push(v[0]);
        right.push(v[1]);
    });

    left.sort();
    right.sort();

    let mut acc: i32 = 0;
    for i in 0..left.len() {
        acc += (left[i] - right[i]).abs();
    }

    Ok(acc)
}

pub fn solution2() -> Result<i32, Box<dyn Error + 'static>> {
    let mut left: Vec<i32> = vec![];
    let mut right: Vec<i32> = vec![];

    PROBLEM.lines().for_each(|line| {
        let v: Vec<i32> = line
            .split_ascii_whitespace()
            .map(|l| l.parse::<i32>().expect("parse"))
            .collect();
        assert!(v.len() == 2);
        left.push(v[0]);
        right.push(v[1]);
    });

    left.sort();
    right.sort();

    let mut acc: i32 = 0;
    for i in 0..left.len() {
        acc += left[i] * right.count(left[i]) as i32;
    }

    Ok(acc)
}
