use std::error::Error;

const PROBLEM: &str = include_str!("input/day8.txt");

pub fn solution1() -> Result<(), Box<dyn Error + 'static>> {
    let mut code_chars: usize = 0;
    let mut memory_chars: usize = 0;

    for line in PROBLEM.lines() {
        code_chars += line.len();
        let mut escaped: u32 = 0;

        for chr in line.chars() {
            if escaped == 0 {
                if chr == '\\' {
                    escaped = 1;
                }

                if chr != '"' {
                    memory_chars += 1;
                }
            } else {
                if escaped == 1 && chr == 'x' {
                    escaped = 2;
                } else {
                    escaped -= 1;
                }
            }
        }
    }

    println!("Answer: {}", code_chars - memory_chars);
    Ok(())
}

fn encode(string: &str) -> String {
    let mut chars = Vec::<char>::new();
    chars.push('"');

    for chr in string.chars() {
        match chr {
            '"' => {
                chars.push('\\');
                chars.push('"');
            },
            '\\' => {
                chars.push('\\');
                chars.push('\\');
            },
            _ => chars.push(chr),
        }
    }
    
    chars.push('"');
    chars.into_iter().collect()
}

pub fn solution2() -> Result<(), Box<dyn Error + 'static>> {
    let mut code_chars: usize = 0;
    let mut encoded_chars: usize = 0;

    for line in PROBLEM.lines() {
        code_chars += line.len();
        encoded_chars += encode(&line).len();
    }

    println!("Answer: {}", encoded_chars - code_chars);
    Ok(())
}
