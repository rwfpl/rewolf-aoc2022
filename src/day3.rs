use itertools::Itertools;
use std::collections::HashSet;
use std::fs;

fn day3_p1() -> u64 {
    fs::read_to_string("src/inputs/aoc_3.input")
        .unwrap()
        .lines()
        .map(|line| {
            HashSet::from_iter(line[..line.len() / 2].bytes())
                .intersection(&HashSet::<u8>::from_iter(line[line.len() / 2..].bytes()))
                .map(|d| {
                    if (&b'a'..=&b'z').contains(&d) {
                        (*d as u64) - 0x60
                    } else {
                        (*d as u64) - 0x40 + 26
                    }
                })
                .sum::<u64>()
        })
        .sum()
}

fn day3_p2() -> u64 {
    fs::read_to_string("src/inputs/aoc_3.input")
        .unwrap()
        .lines()
        .chunks(3)
        .into_iter()
        .map(|chunk| {
            chunk
                .map(|l| HashSet::from_iter(l.bytes()))
                .reduce(|acc: HashSet<u8>, hs| acc.intersection(&hs).copied().collect())
                .unwrap()
                .iter()
                .map(|d| {
                    if (&b'a'..=&b'z').contains(&d) {
                        (*d as u64) - 0x60
                    } else {
                        (*d as u64) - 0x40 + 26
                    }
                })
                .sum::<u64>()
        })
        .sum()
}

pub fn run() -> (String, String) {
    (day3_p1().to_string(), day3_p2().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        assert_eq!(day3_p1(), 7674);
        assert_eq!(day3_p2(), 2805);
    }
}
