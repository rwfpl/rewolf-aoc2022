use std::collections::HashSet;
use std::fs;

fn get_tree_score(map: &[Vec<u8>], tree: (usize, usize)) -> usize {
    let mut score: usize = 1;
    let tree_size = map[tree.0][tree.1];
    let mut visible = 0;

    for i in tree.1 + 1..map[tree.0].len() {
        visible += 1;
        if map[tree.0][i] >= tree_size {
            break;
        }
    }
    score *= visible;

    visible = 0;
    for i in (0..tree.1).rev() {
        visible += 1;
        if map[tree.0][i] >= tree_size {
            break;
        }
    }
    score *= visible;

    visible = 0;
    for ct in map.iter().skip(tree.0 + 1) {
        visible += 1;
        if ct[tree.1] >= tree_size {
            break;
        }
    }
    score *= visible;

    visible = 0;
    for i in (0..tree.0).rev() {
        visible += 1;
        if map[i][tree.1] >= tree_size {
            break;
        }
    }
    score *= visible;

    score
}

fn get_max_tree_score(map: &Vec<Vec<u8>>) -> usize {
    (0..map[0].len())
        .map(|i| {
            (0..map.len())
                .map(|j| get_tree_score(map, (i, j)))
                .max()
                .unwrap()
        })
        .max()
        .unwrap()
}

fn get_visible_trees(map: &Vec<Vec<u8>>) -> usize {
    // rows
    let left_right = map.iter().enumerate().flat_map(|(i, row)| {
        // left side
        let left = row
            .iter()
            .enumerate()
            .fold((-1, Vec::new()), |mut acc, (j, t)| {
                if *t as i32 > acc.0 {
                    acc.1.push((i, j));
                    (*t as i32, acc.1)
                } else {
                    acc
                }
            })
            .1;
        // right side
        row.iter()
            .rev()
            .enumerate()
            .fold((-1, left), |mut acc, (j, t)| {
                if *t as i32 > acc.0 {
                    acc.1.push((i, row.len() - j - 1));
                    (*t as i32, acc.1)
                } else {
                    acc
                }
            })
            .1
    });

    // columns
    let top_bottom = (0..map[0].len()).flat_map(|i| {
        // top
        let top = (0..map.len())
            .fold((-1, Vec::new()), |mut acc, j| {
                if map[j][i] as i32 > acc.0 {
                    acc.1.push((j, i));
                    (map[j][i] as i32, acc.1)
                } else {
                    acc
                }
            })
            .1;
        // bottom
        (0..map.len())
            .rev()
            .fold((-1, top), |mut acc, j| {
                if map[j][i] as i32 > acc.0 {
                    acc.1.push((j, i));
                    (map[j][i] as i32, acc.1)
                } else {
                    acc
                }
            })
            .1
    });
    HashSet::<(usize, usize)>::from_iter(left_right.chain(top_bottom)).len()
}

fn solution(filename: &str) -> (usize, usize) {
    let map = fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(|line| line.bytes().map(|t| t - 0x30).collect::<Vec<u8>>())
        .collect::<Vec<Vec<u8>>>();
    (get_visible_trees(&map), get_max_tree_score(&map))
}

pub fn run() {
    let (p1, p2) = solution("src/inputs/aoc_8.input");
    println!("day8 p1: {p1}");
    println!("day8 p2: {p2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        assert_eq!(solution("src/inputs/aoc_8_sample.input"), (21, 8));
        assert_eq!(solution("src/inputs/aoc_8.input"), (1538, 496125));
    }
}
