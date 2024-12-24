use regex::Regex;
use std::{
    collections::{HashMap, HashSet},
    error::Error,
};

const PROBLEM: &str = include_str!("input/day7.txt");

struct Context {
    values: HashMap<String, u16>,
}

impl Context {
    fn new() -> Self {
        Self {
            values: HashMap::<String, u16>::new(),
        }
    }

    fn get_value(&self, w: &str) -> Option<&u16> {
        self.values.get(w)
    }

    fn set_value(&mut self, w: &str, v: u16) {
        self.values.insert(w.to_string(), v);
    }
}

enum Op {
    Const(u16),
    Wire(String),
    And(Box<Op>, Box<Op>),
    Or(Box<Op>, Box<Op>),
    Not(Box<Op>),
    LShift(Box<Op>, u16),
    RShift(Box<Op>, u16),
}

impl Op {
    fn eval(&self, context: &Context) -> Option<u16> {
        match self {
            Self::Const(n) => Some(*n),
            Self::Wire(w) => Some(*(context.get_value(&w)?)),
            Self::And(l, r) => Some(l.eval(context)? & r.eval(context)?),
            Self::Or(l, r) => Some(l.eval(context)? | r.eval(context)?),
            Self::Not(op) => Some(!op.eval(context)?),
            Self::LShift(op, n) => Some(op.eval(context)? << n),
            Self::RShift(op, n) => Some(op.eval(context)? >> n),
        }
    }
}

fn is_num(s: &str) -> bool {
    s.chars().all(|c| c.is_digit(10))
}

fn expand_rule(context: &HashMap<String, String>, rule: &str) -> Option<Op> {
    println!("{}", rule);

    if is_num(rule) {
        Some(Op::Const(rule.parse().ok()?))
    } else {
        let next_rule = context.get(rule)?;
        let split: Vec<&str> = next_rule.split(" ").collect();

        match split.len() {
            1 => {
                if is_num(split[0]) {
                    Some(Op::Const(rule.parse().ok()?))
                } else {
                    Some(Op::Wire(split[0].to_string()))
                }
            }
            2 => match split[0] {
                "NOT" => Some(Op::Not(Box::new(expand_rule(context, split[1])?))),
                _ => None,
            },
            3 => match split[1] {
                "AND" => Some(Op::And(
                    Box::new(expand_rule(context, split[0])?),
                    Box::new(expand_rule(context, split[2])?),
                )),
                "OR" => Some(Op::Or(
                    Box::new(expand_rule(context, split[0])?),
                    Box::new(expand_rule(context, split[2])?),
                )),
                "LSHIFT" => Some(Op::LShift(
                    Box::new(expand_rule(context, split[0])?),
                    split[2].parse().ok()?,
                )),
                "RSHIFT" => Some(Op::RShift(
                    Box::new(expand_rule(context, split[0])?),
                    split[2].parse().ok()?,
                )),
                _ => None,
            },
            _ => None,
        }
    }
}

fn dependencies(context: &HashMap<String, String>, w: &str) -> Option<Vec<String>> {
    if is_num(w) {
        Some(Vec::new())
    } else {
        let rewrite = context.get(w)?;
        let split: Vec<&str> = rewrite.split(" ").collect();

        match split.len() {
            1 => {
                if is_num(split[0]) {
                    Some(vec![])
                } else {
                    Some(vec![split[0].to_string()])
                }
            }
            2 => match split[0] {
                "NOT" => {
                    if is_num(split[0]) {
                        Some(vec![])
                    } else {
                        Some(vec![split[1].to_string()])
                    }
                }
                _ => None,
            },
            3 => match split[1] {
                "AND" | "OR" => {
                    let mut v = vec![];
                    if !is_num(split[0]) {
                        v.push(split[0].to_string());
                    }
                    if !is_num(split[2]) {
                        v.push(split[2].to_string());
                    }
                    Some(v)
                }
                "RSHIFT" | "LSHIFT" => {
                    if is_num(split[0]) {
                        Some(vec![split[0].to_string()])
                    } else {
                        Some(vec![])
                    }
                }
                _ => None,
            },
            _ => None,
        }
    }
}

pub fn solution1() -> Result<(), Box<dyn Error + 'static>> {
    let re = Regex::new(r"(.+) -> (.+)")?;
    let mut rules = HashMap::<String, String>::new();
    let mut ctx = Context::new();

    for line in PROBLEM.lines() {
        let caps = re.captures(line).unwrap();
        let rewrite = caps.get(1).unwrap().as_str();
        let target = caps.get(2).unwrap().as_str();
        rules.insert(target.to_string(), rewrite.to_string());
    }

    rules.get("b").unwrap();

    let deps: HashMap<String, Vec<String>> = rules
        .iter()
        .map(|(k, v)| (k.clone(), dependencies(&rules, k).unwrap()))
        .collect();

    let consts: Vec<String> = deps
        .iter()
        .filter(|(k, v)| v.len() == 0)
        .map(|(k, v)| k.clone())
        .collect();

    for c in consts {
        let op = expand_rule(&rules, &c).unwrap();
        ctx.set_value(&c, op.eval(&ctx).unwrap());
    }

    println!("{:?}", ctx.values);

    // println!("{}", expand_rule(&rules, "aa").unwrap());

    Ok(())
}

pub fn solution2() -> Result<(), Box<dyn Error + 'static>> {
    Ok(())
}
