use itertools::Itertools;
use regex::Regex;
use std::{collections::BTreeSet, fs};

#[derive(Debug)]
enum Operator {
    None,
    Mul,
    Add,
}

impl From<&str> for Operator {
    fn from(s: &str) -> Self {
        match s {
            "*" => Operator::Mul,
            "+" => Operator::Add,
            _ => Operator::None,
        }
    }
}

#[derive(Debug)]
enum Value {
    Old,
    Int(i64),
}

impl From<&str> for Value {
    fn from(s: &str) -> Self {
        match s {
            "old" => Value::Old,
            _ => Value::Int(s.parse::<i64>().unwrap()),
        }
    }
}

impl Value {
    fn get_or(&self, or: i64) -> i64 {
        match self {
            Value::Old => or,
            Value::Int(v) => *v,
        }
    }
}

#[derive(Debug)]
struct Operation {
    a: Value,
    b: Value,
    op: Operator,
}

impl Operation {
    fn new(a: &str, b: &str, op: &str) -> Operation {
        Operation {
            a: Value::from(a),
            b: Value::from(b),
            op: Operator::from(op),
        }
    }

    fn execute(&self, old: i64) -> i64 {
        match self.op {
            Operator::Add => self.a.get_or(old) + self.b.get_or(old),
            Operator::Mul => self.a.get_or(old) * self.b.get_or(old),
            _ => 0,
        }
    }
}

impl From<regex::Captures<'_>> for Operation {
    fn from(c: regex::Captures) -> Self {
        Operation::new(
            c.name("a").unwrap().as_str(),
            c.name("b").unwrap().as_str(),
            c.name("op").unwrap().as_str(),
        )
    }
}

#[derive(Debug)]
struct Monkey {
    items: Vec<i64>,
    operation: Operation,
    divisible_by: i64,
    if_true_throw_to: i32,
    if_false_throw_to: i32,
    number_of_inspections: i64,
}

fn play_monkey_business(monkeys: &mut [Monkey], stress_relieve: i64, modulo: i64) {
    for i in 0..monkeys.len() {
        let monkey: &mut Monkey = &mut monkeys[i];
        let throws = monkey
            .items
            .iter()
            .map(|item| {
                let new_item = monkey.operation.execute(*item) / stress_relieve;
                if new_item % monkey.divisible_by == 0 {
                    (monkey.if_true_throw_to, new_item)
                } else {
                    (monkey.if_false_throw_to, new_item)
                }
            })
            .collect::<Vec<(i32, i64)>>();
        monkey.number_of_inspections += monkey.items.len() as i64;
        monkey.items.clear();
        throws.iter().for_each(|throw| {
            monkeys[throw.0 as usize].items.push(throw.1 % modulo);
        });
    }
}

pub fn solution(filename: &str, rounds: i32, stress_relieve: i64) -> i64 {
    let operation_re =
        Regex::new(r"new\s=\s(?P<a>[[:alnum:]]*)\s(?P<op>\*|\+)\s(?P<b>[[:alnum:]]*)").unwrap();
    let mut monkeys = fs::read_to_string(filename)
        .unwrap()
        .lines()
        .chunks(7)
        .into_iter()
        .map(|mut chunk| {
            chunk.next();
            Monkey {
                items: chunk
                    .next()
                    .unwrap()
                    .trim()
                    .strip_prefix("Starting items: ")
                    .unwrap()
                    .split(',')
                    .map(|x| x.trim().parse::<i64>().unwrap())
                    .collect(),
                operation: Operation::from(
                    operation_re
                        .captures(
                            chunk
                                .next()
                                .unwrap()
                                .trim()
                                .strip_prefix("Operation: ")
                                .unwrap(),
                        )
                        .unwrap(),
                ),
                divisible_by: chunk
                    .next()
                    .unwrap()
                    .trim()
                    .strip_prefix("Test: divisible by ")
                    .unwrap()
                    .parse::<i64>()
                    .unwrap(),
                if_true_throw_to: chunk
                    .next()
                    .unwrap()
                    .trim()
                    .strip_prefix("If true: throw to monkey ")
                    .unwrap()
                    .parse::<i32>()
                    .unwrap(),
                if_false_throw_to: chunk
                    .next()
                    .unwrap()
                    .trim()
                    .strip_prefix("If false: throw to monkey ")
                    .unwrap()
                    .parse::<i32>()
                    .unwrap(),
                number_of_inspections: 0,
            }
        })
        .collect::<Vec<Monkey>>();
    let modulo = monkeys.iter().map(|monkey| monkey.divisible_by).product();

    (0..rounds).for_each(|_| {
        play_monkey_business(&mut monkeys, stress_relieve, modulo);
    });

    monkeys
        .iter()
        .map(|monkey| monkey.number_of_inspections)
        .collect::<BTreeSet<i64>>()
        .iter()
        .rev()
        .take(2)
        .product()
}

pub fn run() {
    println!("day11 p1: {}", solution("src/inputs/aoc_11.input", 20, 3));
    println!(
        "day11 p2: {}",
        solution("src/inputs/aoc_11.input", 10_000, 1)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        assert_eq!(solution("src/inputs/aoc_11_sample.input", 20, 3), 10605);
        assert_eq!(
            solution("src/inputs/aoc_11_sample.input", 10_000, 1),
            2713310158
        );
        assert_eq!(solution("src/inputs/aoc_11.input", 20, 3), 55458);
        assert_eq!(solution("src/inputs/aoc_11.input", 10_000, 1), 14508081294);
    }
}
