use std::{
    collections::{HashMap, HashSet, VecDeque},
    error::Error,
};

const PROBLEM: &str = include_str!("input/day7.txt");

enum Operation<'a> {
    And(Box<Operation<'a>>, Box<Operation<'a>>),
    Or(Box<Operation<'a>>, Box<Operation<'a>>),
    LShift(Box<Operation<'a>>, Box<Operation<'a>>),
    RShift(Box<Operation<'a>>, Box<Operation<'a>>),
    Not(Box<Operation<'a>>),
    Variable(&'a str),
    Value(u16),
}

impl<'a> Operation<'a> {
    fn vars(&self) -> Vec<&'a str> {
        match self {
            Operation::And(lhs, rhs)
            | Operation::Or(lhs, rhs)
            | Operation::LShift(lhs, rhs)
            | Operation::RShift(lhs, rhs) => {
                let mut v = lhs.vars();
                v.append(&mut rhs.vars());
                v
            }
            Operation::Not(lhs) => lhs.vars(),
            Operation::Variable(var) => vec![var],
            Operation::Value(_) => vec![],
        }
    }

    fn eval(&self, values: &HashMap<&str, u16>) -> u16 {
        match self {
            Operation::And(lhs, rhs) => lhs.eval(values) & rhs.eval(values),
            Operation::Or(lhs, rhs) => lhs.eval(values) | rhs.eval(values),
            Operation::LShift(lhs, rhs) => lhs.eval(values) << rhs.eval(values),
            Operation::RShift(lhs, rhs) => lhs.eval(values) >> rhs.eval(values),
            Operation::Not(lhs) => !lhs.eval(values),
            Operation::Variable(var) => *values.get(var).expect("get"),
            Operation::Value(v) => *v,
        }
    }
}

fn parse<'a>(op: &'a str) -> Operation<'a> {
    let split: Vec<&str> = op.split_ascii_whitespace().collect();

    if split.len() == 1 {
        if let Ok(num) = split[0].parse::<u16>() {
            Operation::Value(num)
        } else {
            Operation::Variable(op)
        }
    } else if split.len() == 2 {
        if split[0] == "NOT" {
            Operation::Not(Box::new(parse(split[1])))
        } else {
            unreachable!()
        }
    } else if split.len() == 3 {
        let lhs = parse(split[0]);
        let rhs = parse(split[2]);

        match split[1] {
            "AND" => Operation::And(Box::new(lhs), Box::new(rhs)),
            "OR" => Operation::Or(Box::new(lhs), Box::new(rhs)),
            "LSHIFT" => Operation::LShift(Box::new(lhs), Box::new(rhs)),
            "RSHIFT" => Operation::RShift(Box::new(lhs), Box::new(rhs)),
            _ => unreachable!(),
        }
    } else {
        unreachable!();
    }
}

struct TopologicalSort<'a> {
    _data: std::marker::PhantomData<&'a str>,
}

impl<'a> TopologicalSort<'a> {
    pub fn sort(dependencies: &'a HashMap<&str, Vec<&str>>) -> VecDeque<&'a str> {
        let mut seen = HashSet::<&'a str>::new();
        let mut l = VecDeque::<&str>::new();

        for &dep in dependencies.keys() {
            Self::_visit(dep, dependencies, &mut seen, &mut l);
        }

        l
    }

    fn _visit(
        dep: &'a str,
        dependencies: &'a HashMap<&str, Vec<&str>>,
        seen: &mut HashSet<&'a str>,
        l: &mut VecDeque<&'a str>,
    ) {
        if !seen.contains(dep) {
            for &dep2 in dependencies.get(dep).expect("get") {
                Self::_visit(dep2, dependencies, seen, l);
            }

            seen.insert(dep);
            l.push_back(dep);
        }
    }
}

pub fn solution1() -> Result<(), Box<dyn Error + 'static>> {
    let mut dependencies = HashMap::<&str, Vec<&str>>::new();
    let mut instructions = HashMap::<&str, Operation>::new();
    let mut values = HashMap::<&str, u16>::new();

    for line in PROBLEM.lines() {
        let operands: Vec<&str> = line.split("->").map(|s| s.trim()).collect();

        assert_eq!(operands.len(), 2);
        let instuction = parse(operands[0]);
        dependencies.insert(operands[1], instuction.vars());
        instructions.insert(operands[1], instuction);
    }

    let sorted = TopologicalSort::sort(&dependencies);
    for dep in sorted {
        let op = instructions.get(dep).expect("get");
        values.insert(dep, op.eval(&values));
    }

    println!("Answer: {}", values.get("a").expect("get")); // 16076
    Ok(())
}

pub fn solution2() -> Result<(), Box<dyn Error + 'static>> {
    let mut dependencies = HashMap::<&str, Vec<&str>>::new();
    let mut instructions = HashMap::<&str, Operation>::new();
    let mut values = HashMap::<&str, u16>::new();

    for line in PROBLEM.lines() {
        let operands: Vec<&str> = line.split("->").map(|s| s.trim()).collect();

        assert_eq!(operands.len(), 2);
        let instuction = parse(operands[0]);
        dependencies.insert(operands[1], instuction.vars());
        instructions.insert(operands[1], instuction);
    }

    let sorted = TopologicalSort::sort(&dependencies);
    for &dep in &sorted {
        let op = instructions.get(dep).expect("get");
        values.insert(dep, op.eval(&values));
    }

    let a = *values.get("a").expect("get");

    values.clear();
    instructions.insert("b", Operation::Value(a));

    for &dep in &sorted {
        let op = instructions.get(dep).expect("get");
        values.insert(dep, op.eval(&values));
    }

    println!("Answer: {}", values.get("a").expect("get"));
    Ok(())
}
