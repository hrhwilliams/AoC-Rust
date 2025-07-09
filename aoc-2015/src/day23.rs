use std::error::Error;

const PROBLEM: &str = include_str!("input/day23.txt");

fn execute_program(instructions: &str, a: u32, b: u32) -> (u32, u32) {
    let mut a: u32 = a;
    let mut b: u32 = b;
    let mut ip: usize = 0;

    let instruction_stream: Vec<&str> = instructions.lines().collect();

    while ip < instruction_stream.len() {
        println!("register state - a = {}, b = {}, ip = {}", a, b, ip);
        println!("instr: {}", instruction_stream[ip]);
        let words: Vec<&str> = instruction_stream[ip].split(" ").collect();

        match words[0] {
            "hlf" => match words[1] {
                "a" => {
                    a /= 2;
                    ip += 1
                }
                "b" => {
                    b /= 2;
                    ip += 1
                }
                _ => panic!("unknown register"),
            },
            "tpl" => match words[1] {
                "a" => {
                    a *= 3;
                    ip += 1
                }
                "b" => {
                    b *= 3;
                    ip += 1
                }
                _ => panic!("unknown register"),
            },
            "inc" => match words[1] {
                "a" => {
                    a += 1;
                    ip += 1
                }
                "b" => {
                    b += 1;
                    ip += 1
                }
                _ => panic!("unknown register"),
            },
            "jmp" => {
                let offset = words[1].parse::<i32>().expect("offset not numeric");
                if offset < 0 {
                    ip -= offset.abs() as usize;
                } else {
                    ip += offset as usize;
                }
            }
            "jie" => {
                let offset = words[2].parse::<i32>().expect("offset not numeric");
                let register = words[1].strip_suffix(",").expect("missing comma");

                match register {
                    "a" => {
                        if a % 2 == 0 {
                            if offset < 0 {
                                ip -= offset.abs() as usize;
                            } else {
                                ip += offset as usize;
                            }
                        } else {
                            ip += 1;
                        }
                    }
                    "b" => {
                        if b % 2 == 0 {
                            if offset < 0 {
                                ip -= offset.abs() as usize;
                            } else {
                                ip += offset as usize;
                            }
                        } else {
                            ip += 1;
                        }
                    }
                    _ => panic!("unknown register"),
                }
            }
            "jio" => {
                let offset = words[2].parse::<i32>().expect("offset not numeric");
                let register = words[1].strip_suffix(",").expect("missing comma");

                match register {
                    "a" => {
                        if a == 1 {
                            if offset < 0 {
                                ip -= offset.abs() as usize;
                            } else {
                                ip += offset as usize;
                            }
                        } else {
                            ip += 1;
                        }
                    }
                    "b" => {
                        if b == 1 {
                            if offset < 0 {
                                ip -= offset.abs() as usize;
                            } else {
                                ip += offset as usize;
                            }
                        } else {
                            ip += 1;
                        }
                    }
                    _ => panic!("unknown register"),
                }
            }
            _ => break,
        }
    }

    (a, b)
}

pub fn solution1() -> Result<(), Box<dyn Error + 'static>> {
    let (_a, b) = execute_program(PROBLEM, 0, 0);
    
    println!("{}", b);
    Ok(())
}

pub fn solution2() -> Result<(), Box<dyn Error + 'static>> {
    let (_a, b) = execute_program(PROBLEM, 1, 0);

    println!("{}", b);
    Ok(())
}
