extern crate derive_more;

use derive_more::Constructor;
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::fs;

#[derive(Debug, Constructor, Clone, Copy, PartialEq, Eq)]
struct Val {
    i: usize,
    v: i64,
}

fn solution(filename: &str, decryption_key: i64, rounds: usize) -> i64 {
    let mut input: VecDeque<Val> = fs::read_to_string(filename)
        .unwrap()
        .lines()
        .enumerate()
        .map(|(i, l)| Val::new(i, l.parse::<i64>().unwrap() * decryption_key))
        .collect();

    for i in 0..rounds * input.len() {
        let (current_index, v) = input
            .iter()
            .enumerate()
            .find(|(_, val)| val.i == i % input.len())
            .unwrap();

        let vc = *v;
        match vc.v.cmp(&0) {
            Ordering::Greater => {
                input.remove(current_index);
                input.insert((current_index + vc.v as usize) % input.len(), vc);
            }
            Ordering::Less => {
                input.remove(current_index);
                if vc.v.abs() < current_index as i64 {
                    input.insert(current_index - vc.v.unsigned_abs() as usize, vc);
                } else {
                    input.insert(
                        input.len() - (vc.v.unsigned_abs() as usize - current_index) % input.len(),
                        vc,
                    );
                }
            }
            Ordering::Equal => {}
        }
    }
    let zero_pos = input.iter().enumerate().find(|(_, v)| v.v == 0).unwrap().0;
    input[(zero_pos + 1000) % input.len()].v
        + input[(zero_pos + 2000) % input.len()].v
        + input[(zero_pos + 3000) % input.len()].v
}

pub fn run() -> (String, String) {
    (
        solution("src/inputs/aoc_20.input", 1, 1).to_string(),
        solution("src/inputs/aoc_20.input", 811589153, 10).to_string(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        assert_eq!(solution("src/inputs/aoc_20_sample.input", 1, 1), 3);
        assert_eq!(
            solution("src/inputs/aoc_20_sample.input", 811589153, 10),
            1_623_178_306
        );
        assert_eq!(solution("src/inputs/aoc_20.input", 1, 1), 7225);
        assert_eq!(
            solution("src/inputs/aoc_20.input", 811589153, 10),
            548_634_267_428
        );
    }
}
