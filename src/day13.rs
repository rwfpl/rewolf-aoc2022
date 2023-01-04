use core::cmp::Ordering;
use itertools::sorted;
use itertools::Itertools;
use std::{cmp, fs};

#[derive(Debug, Clone)]
enum Packet {
    Int(i32),
    List(Vec<Self>),
}

impl From<&str> for Packet {
    fn from(s: &str) -> Self {
        let mut i = 0;
        let mut p: Vec<Packet> = Vec::new();
        loop {
            match s.chars().nth(i).unwrap() {
                '[' => {
                    p.push(Packet::List(Vec::new()));
                    i += 1;
                }
                ']' => {
                    let tl = p.pop().unwrap();
                    if p.is_empty() {
                        return tl;
                    }
                    if let Packet::List(cp) = p.last_mut().unwrap() {
                        cp.push(tl);
                    }
                    i += 1;
                }
                v if ('0'..='9').contains(&v) => {
                    let ss = s.get(i..).unwrap();
                    let x_end = cmp::min(
                        ss.find(',').or(Some(ss.len())),
                        ss.find(']').or(Some(ss.len())),
                    )
                    .unwrap();
                    if let Packet::List(cp) = p.last_mut().unwrap() {
                        cp.push(Packet::Int(
                            s.get(i..i + x_end).unwrap().parse::<i32>().unwrap(),
                        ));
                    }
                    i += x_end;
                }
                _ => i += 1,
            }
        }
    }
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        self.partial_cmp(other).unwrap() == Ordering::Equal
    }
}

impl Eq for Packet {}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (&self, &other) {
            (Packet::Int(a), Packet::Int(b)) => a.partial_cmp(b),
            (Packet::List(a), Packet::List(b)) => {
                for i in 0..cmp::min(a.len(), b.len()) {
                    if a[i] == b[i] {
                        continue;
                    } else {
                        return a[i].partial_cmp(&b[i]);
                    }
                }
                a.len().partial_cmp(&b.len())
            }
            (Packet::Int(a), &b) => Packet::List(vec![Packet::Int(*a)]).partial_cmp(b),
            (&a, Packet::Int(b)) => a.partial_cmp(&Packet::List(vec![Packet::Int(*b)])),
        }
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[derive(Debug)]
struct Pair {
    left: Packet,
    right: Packet,
}

impl From<&Vec<&str>> for Pair {
    fn from(v: &Vec<&str>) -> Pair {
        Pair {
            left: Packet::from(v[0]),
            right: Packet::from(v[1]),
        }
    }
}

fn solution(filename: &str) -> (usize, usize) {
    let pairs: Vec<Pair> = fs::read_to_string(filename)
        .unwrap()
        .lines()
        .chunks(3)
        .into_iter()
        .map(|chunk| Pair::from(&chunk.collect::<Vec<&str>>()))
        .collect();

    let packet_2 = Packet::from("[[2]]");
    let packet_6 = Packet::from("[[6]]");
    (
        // p1
        pairs
            .iter()
            .enumerate()
            .filter(|(_, p)| p.left < p.right)
            .map(|(i, _)| i + 1)
            .sum(),
        // p2
        sorted(
            pairs
                .into_iter()
                .flat_map(|p| [p.left, p.right])
                .chain([packet_2.clone(), packet_6.clone()].into_iter()),
        )
        .into_iter()
        .enumerate()
        .filter(|(_, p)| p == &packet_2 || p == &packet_6)
        .map(|(i, _)| i + 1)
        .product(),
    )
}

pub fn run() {
    let (p1, p2) = solution("src/inputs/aoc_13.input");
    println!("day13 p1: {p1}");
    println!("day13 p2: {p2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        assert_eq!(solution("src/inputs/aoc_13_sample.input"), (13, 140));
        assert_eq!(solution("src/inputs/aoc_13.input"), (6101, 21909));
    }
}
