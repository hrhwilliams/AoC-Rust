use std::error::Error;

const PROBLEM: &str = "1113222113";

fn iterate(num: &str) -> String {
    let mut current_char = '\0';
    let mut current_count: usize = 0;
    let mut next_string = String::new();

    for n in num.chars() {
        if n == current_char {
            current_count += 1;
        } else {
            if current_count > 0 {
                next_string.push_str(&current_count.to_string());
                next_string.push(current_char);
            }
            current_char = n;
            current_count = 1;
        }
    }

    if current_count > 0 {
        next_string.push_str(&current_count.to_string());
        next_string.push(current_char);
    }

    next_string
}

pub fn solution1() -> Result<(), Box<dyn Error + 'static>> {
    let mut n = PROBLEM.to_string();
    for _ in 0..40 {
        n = iterate(&n);
    }
    println!("{}", n.len());

    Ok(())
}

pub fn solution2() -> Result<(), Box<dyn Error + 'static>> {
    let mut n = PROBLEM.to_string();
    for _ in 0..50 {
        n = iterate(&n);
    }
    println!("{}", n.len());

    Ok(())
}
