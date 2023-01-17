use std::{collections::BTreeSet, fs};

fn solution() -> (i32, i32) {
    let mut current_calories = 0;
    let maxes: BTreeSet<i32> = fs::read_to_string("src/inputs/aoc_1.input")
        .unwrap()
        .lines()
        .map(|line| {
            if line.is_empty() {
                let r = current_calories;
                current_calories = 0;
                r
            } else {
                current_calories += line.parse::<i32>().unwrap();
                0
            }
        })
        .collect();
    (*maxes.last().unwrap(), maxes.iter().rev().take(3).sum())
}

pub fn run() -> (String, String) {
    let (p1, p2) = solution();
    (p1.to_string(), p2.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        assert_eq!(solution(), (72017, 212520));
    }
}
