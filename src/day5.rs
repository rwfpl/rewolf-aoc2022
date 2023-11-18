use regex::Regex;
use std::fs;

// Let's cheat a little and and assume there is always 9 stacks at most.
const MAX_CRATES: usize = 9;

fn part1(n: usize, src: usize, dst: usize, stacks: &mut [Vec<char>; MAX_CRATES]) {
    (0..n).for_each(|_| {
        let x = stacks[src].pop().unwrap();
        stacks[dst].push(x);
    });
}

fn part2(n: usize, src: usize, dst: usize, stacks: &mut [Vec<char>; MAX_CRATES]) {
    let x: Vec<char> = stacks[src].drain((stacks[src].len() - n)..).collect();
    stacks[dst].extend(x);
}

fn solution<F>(stack_operation: F) -> String
where
    F: Fn(usize, usize, usize, &mut [Vec<char>; MAX_CRATES]),
{
    // Positions of crates for each stacks.
    let positions: Vec<usize> = (0..MAX_CRATES).map(|x| 4 * x + 1).collect();
    let move_re = Regex::new(r"^move\s(?P<n>\d*)\sfrom\s(?P<src>\d*)\sto\s(?P<dst>\d*)$").unwrap();
    let input = fs::read_to_string("src/inputs/aoc_5.input").unwrap();
    let mut stacks: [Vec<char>; MAX_CRATES] = Default::default();

    input
        .lines()
        .filter(|line| line.contains('['))
        .for_each(|line| {
            positions.iter().for_each(|pos| {
                let x = line.chars().nth(*pos).unwrap();
                if x != ' ' && x.is_ascii_uppercase() {
                    stacks[(*pos - 1) / 4].push(x);
                }
            });
        });
    stacks.iter_mut().for_each(|stack| stack.reverse());

    input
        .lines()
        .filter(|line| line.starts_with("move"))
        .for_each(|line| {
            let mv = move_re.captures(line).unwrap();
            let n = mv.name("n").unwrap().as_str().parse::<usize>().unwrap();
            let src = mv.name("src").unwrap().as_str().parse::<usize>().unwrap() - 1;
            let dst = mv.name("dst").unwrap().as_str().parse::<usize>().unwrap() - 1;
            stack_operation(n, src, dst, &mut stacks);
        });

    stacks
        .iter()
        .map(|stack| stack.last().unwrap())
        .collect::<String>()
}

pub fn run() -> (String, String) {
    (solution(part1), solution(part2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        assert_eq!(solution(part1), "BSDMQFLSP");
        assert_eq!(solution(part2), "PGSQBFLDP");
    }
}
