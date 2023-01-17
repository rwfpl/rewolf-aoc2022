use std::fs;

fn get_score_p1(input: &str) -> i32 {
    match input {
        "A X" => 1 + 3,
        "B X" => 1,
        "C X" => 1 + 6,
        "A Y" => 2 + 6,
        "B Y" => 2 + 3,
        "C Y" => 2,
        "A Z" => 3,
        "B Z" => 3 + 6,
        "C Z" => 3 + 3,
        _ => 0,
    }
}

fn win(a: char) -> i32 {
    match a {
        'A' => 2,
        'B' => 3,
        'C' => 1,
        _ => 0,
    }
}

fn draw(a: char) -> i32 {
    match a {
        'A' => 1,
        'B' => 2,
        'C' => 3,
        _ => 0,
    }
}

fn lose(a: char) -> i32 {
    match a {
        'A' => 3,
        'B' => 1,
        'C' => 2,
        _ => 0,
    }
}

fn get_score_p2(input: &str) -> i32 {
    match input.chars().nth(2) {
        Some('X') => lose(input.chars().next().unwrap()),
        Some('Y') => 3 + draw(input.chars().next().unwrap()),
        Some('Z') => 6 + win(input.chars().next().unwrap()),
        _ => 0,
    }
}

fn solution<F>(get_score: F) -> i32
where
    F: Fn(&str) -> i32,
{
    fs::read_to_string("src/inputs/aoc_2.input")
        .unwrap()
        .lines()
        .map(get_score)
        .sum()
}

pub fn run() -> (String, String) {
    (
        solution(get_score_p1).to_string(),
        solution(get_score_p2).to_string(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        assert_eq!(solution(get_score_p1), 13446);
        assert_eq!(solution(get_score_p2), 13509);
    }
}
