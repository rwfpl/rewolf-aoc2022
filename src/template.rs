use std::fs;

fn solution(filename: &str) -> i32{
    fs::read_to_string(filename).unwrap().lines().count() as i32
}

#[test]
fn test_run() {
    assert_eq!(solution("src/inputs/aoc_XX_sample.input"), 0);
}

pub fn run() {
    println!("dayXX p1: {}", solution("src/inputs/aoc_XX.input"));
}