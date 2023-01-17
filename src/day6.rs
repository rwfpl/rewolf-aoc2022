use std::{collections::HashSet, fs};

fn is_packet_start(s: &str) -> bool {
    HashSet::<u8>::from_iter(s.bytes()).len() == s.len()
}

fn solution(distinct: usize) -> Option<usize> {
    let input = fs::read_to_string("src/inputs/aoc_6.input").unwrap();
    (0..input.len() - distinct)
        .filter(|i| is_packet_start(input.get(*i..*i + distinct).unwrap()))
        .map(|i| i + distinct)
        .next()
}

pub fn run() -> (String, String) {
    (
        solution(4).unwrap().to_string(),
        solution(14).unwrap().to_string(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        assert_eq!(solution(4), Some(1275));
        assert_eq!(solution(14), Some(3605))
    }
}
