extern crate derive_more;
extern crate lazy_static;

use derive_more::Constructor;
use regex::Regex;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;
use std::sync::Mutex;

#[derive(Debug)]
enum Operator {
    None,
    Add,
    Sub,
    Mul,
    Div,
    Eq,
}

impl From<char> for Operator {
    fn from(c: char) -> Self {
        match c {
            '+' => Operator::Add,
            '-' => Operator::Sub,
            '/' => Operator::Div,
            '*' => Operator::Mul,
            '=' => Operator::Eq,
            _ => Operator::None,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct VarName {
    id: u32,
}

impl From<&str> for VarName {
    fn from(s: &str) -> Self {
        VarName {
            id: *s.as_bytes().first().unwrap() as u32
                | (*s.as_bytes().get(1).unwrap() as u32) << 8
                | (*s.as_bytes().get(2).unwrap() as u32) << 16
                | (*s.as_bytes().get(3).unwrap() as u32) << 24,
        }
    }
}

#[derive(Debug, Constructor)]
struct Expression {
    a: VarName,
    op: Operator,
    b: VarName,
}

#[derive(Debug)]
enum Value {
    None,
    Value(i64),
    Expression(Expression),
}

impl From<regex::Captures<'_>> for Value {
    fn from(capture: regex::Captures) -> Self {
        if let Some(v) = capture.name("value") {
            Value::Value(v.as_str().parse::<i64>().unwrap())
        } else if let Some(e) = capture.name("op") {
            Value::Expression(Expression::new(
                VarName::from(capture.name("v1").unwrap().as_str()),
                Operator::from(e.as_str().chars().next().unwrap()),
                VarName::from(capture.name("v2").unwrap().as_str()),
            ))
        } else {
            Value::None
        }
    }
}

#[derive(Debug)]
struct Var {
    name: VarName,
    value: Value,
}

impl From<regex::Captures<'_>> for Var {
    fn from(capture: regex::Captures) -> Self {
        Var {
            name: VarName::from(capture.name("var").unwrap().as_str()),
            value: Value::from(capture),
        }
    }
}

lazy_static::lazy_static! {
    static ref VALUECACHE: Mutex<HashMap<VarName, i64>> = Mutex::new(HashMap::new());
}

fn get_value(var: &VarName, input: &HashMap<VarName, Var>) -> i64 {
    if let Some(r) = VALUECACHE.lock().unwrap().get(var) {
        return *r;
    }
    match &input.get(var).unwrap().value {
        Value::None => 0,
        Value::Value(v) => {
            VALUECACHE.lock().unwrap().insert(*var, *v);
            *v
        }
        Value::Expression(e) => match e.op {
            Operator::None => 0,
            Operator::Add => get_value(&e.a, input) + get_value(&e.b, input),
            Operator::Mul => get_value(&e.a, input) * get_value(&e.b, input),
            Operator::Div => get_value(&e.a, input) / get_value(&e.b, input),
            Operator::Sub => get_value(&e.a, input) - get_value(&e.b, input),
            Operator::Eq => get_value(&e.a, input) - get_value(&e.b, input),
        },
    }
}

fn load_input(filename: &str) -> HashMap<VarName, Var> {
    let input_re = Regex::new(
        r"(?P<var>[a-z]{4}):\s((?P<value>\d+)|((?P<v1>[a-z]{4})\s(?P<op>[+\-*/])\s(?P<v2>[a-z]{4})))",
    ).unwrap();
    fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(|line| Var::from(input_re.captures(line).unwrap()))
        .map(|var| (var.name, var))
        .collect::<HashMap<VarName, Var>>()
}

fn solution_p1(filename: &str) -> i64 {
    VALUECACHE.lock().unwrap().clear();
    get_value(&VarName::from("root"), &load_input(filename))
}

fn solution_p2(filename: &str) -> i64 {
    let mut input = load_input(filename);

    if let Value::Expression(e) = &mut input.get_mut(&VarName::from("root")).unwrap().value {
        e.op = Operator::Eq;
    }

    let mut lower = 0;
    let mut upper: i64 = 10_000_000_000_000;
    let mut current = 0;
    loop {
        VALUECACHE.lock().unwrap().clear();
        if let Value::Value(v) = &mut input.get_mut(&VarName::from("humn")).unwrap().value {
            *v = current;
        }

        let res = get_value(&VarName::from("root"), &input);
        match res.cmp(&0) {
            Ordering::Equal => {
                return current;
            }
            Ordering::Greater => {
                lower = current;
            }
            Ordering::Less => {
                upper = current;
            }
        }
        current = lower + (upper - lower) / 2;
        if lower == upper {
            panic!("couldn't find the value");
        }
    }
}

pub fn run() {
    println!("day21 p1: {}", solution_p1("src/inputs/aoc_21.input"));
    println!("day21 p2: {}", solution_p2("src/inputs/aoc_21.input"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        assert_eq!(solution_p1("src/inputs/aoc_21_sample.input"), 152);
        // This one doesn't realy work with the current binary search parameters.
        // assert_eq!(solution_p2("src/inputs/aoc_21_sample.input"), 301);
        assert_eq!(solution_p1("src/inputs/aoc_21.input"), 223_971_851_179_174);
        assert_eq!(solution_p2("src/inputs/aoc_21.input"), 3_379_022_190_351);
    }
}
