use std::collections::HashSet;
use std::fs;

fn get_distance(head: (i32, i32), tail: (i32, i32)) -> (i32, i32) {
    (head.0 - tail.0, head.1 - tail.1)
}

fn update_knots(head: (i32, i32), tail: &mut (i32, i32)) {
    let distance = get_distance(head, *tail);
    let dst_abs = (distance.0.abs(), distance.1.abs());
    match dst_abs {
        (0, 2) => tail.1 += distance.1 / 2,
        (2, 0) => tail.0 += distance.0 / 2,
        (1, 2) => {
            tail.0 += distance.0;
            tail.1 += distance.1 / 2;
        }
        (2, 1) => {
            tail.0 += distance.0 / 2;
            tail.1 += distance.1;
        }
        (2, 2) => {
            tail.0 += distance.0 / 2;
            tail.1 += distance.1 / 2;
        }
        (0, 1) | (1, 0) | (0, 0) | (1, 1) => {}
        _ => panic!("err: {distance:?} {head:?} {tail:?}"),
    }
}

fn solution(filename: &str, n: usize) -> usize {
    let mut knots = vec![(0, 0); n + 1];
    let visited = fs::read_to_string(filename)
        .unwrap()
        .lines()
        .flat_map(|line| {
            let mv = line[2..].parse::<i32>().unwrap();
            (0..mv)
                .map(|_| {
                    match line.as_bytes().first().unwrap() {
                        b'R' => knots[0].0 += 1,
                        b'L' => knots[0].0 -= 1,
                        b'U' => knots[0].1 += 1,
                        b'D' => knots[0].1 -= 1,
                        _ => {}
                    }
                    (0..knots.len() - 1).for_each(|i| {
                        update_knots(knots[i], &mut knots[i + 1]);
                    });
                    knots[n]
                })
                .collect::<HashSet<(i32, i32)>>()
        })
        .collect::<HashSet<(i32, i32)>>();
    visited.len()
}

pub fn run() -> (String, String) {
    (
        solution("src/inputs/aoc_9.input", 1).to_string(),
        solution("src/inputs/aoc_9.input", 9).to_string(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        assert_eq!(solution("src/inputs/aoc_9_sample.input", 1), 13);
        assert_eq!(solution("src/inputs/aoc_9_sample2.input", 9), 36);
        assert_eq!(solution("src/inputs/aoc_9.input", 1), 6339);
        assert_eq!(solution("src/inputs/aoc_9.input", 9), 2541);
    }
}
