use std::fs;
use std::iter;

fn get_signal(cycle: i32, regx: i32) -> i32 {
    if ((cycle - 20) % 40) == 0 {
        cycle * regx
    } else {
        0
    }
}

fn crt_to_string(crt: &[Vec<char>]) -> String {
    crt.iter()
        .map(|row| iter::once(&'\n').chain(row.iter()).collect::<String>())
        .collect::<String>()
}

fn cycle_to_pos(cycle: i32) -> (usize, usize) {
    let cycle = cycle as usize;
    ((cycle - 1) / 40, (cycle - 1) % 40)
}

fn update_pixel(crt: &mut [Vec<char>], cycle: i32, regx: i32) {
    let pos = cycle_to_pos(cycle);
    if pos.1 as i32 >= regx - 1 && pos.1 as i32 <= regx + 1 {
        crt[pos.0][pos.1] = '#';
    }
}

fn solution(filename: &str) -> (i32, String) {
    let mut cycle: i32 = 1;
    let mut regx: i32 = 1;
    let mut crt: Vec<Vec<char>> = vec![vec!['.'; 40]; 6];
    let sum: i32 = fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(|line| match line {
            "noop" => {
                let signal = get_signal(cycle, regx);
                update_pixel(&mut crt, cycle, regx);
                cycle += 1;
                signal
            }
            addx if addx.starts_with("addx ") => {
                let signal1 = get_signal(cycle, regx);
                update_pixel(&mut crt, cycle, regx);
                cycle += 1;
                let signal2 = get_signal(cycle, regx);
                update_pixel(&mut crt, cycle, regx);
                cycle += 1;
                regx += line[5..].parse::<i32>().unwrap();
                signal1 + signal2
            }
            _ => {
                panic!("error: {line}")
            }
        })
        .sum();
    (sum, crt_to_string(&crt))
}

pub fn run() {
    let (p1, p2) = solution("src/inputs/aoc_10.input");
    println!("day10 p1: {p1}");
    println!("day10 p1: {p2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        assert_eq!(
            solution("src/inputs/aoc_10_sample.input"),
            (
                13140,
                "
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."
                    .to_owned()
            )
        );
        assert_eq!(
            solution("src/inputs/aoc_10.input"),
            (
                16480,
                "
###..#....####.####.#..#.#....###..###..
#..#.#....#....#....#..#.#....#..#.#..#.
#..#.#....###..###..#..#.#....#..#.###..
###..#....#....#....#..#.#....###..#..#.
#....#....#....#....#..#.#....#....#..#.
#....####.####.#.....##..####.#....###.."
                    .to_owned()
            )
        );
    }
}
