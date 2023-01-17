use itertools::Itertools;
use std::{collections::HashSet, fs};

fn is_boundary(cube: &(usize, usize, usize), pond: &Vec<Vec<Vec<bool>>>) -> bool {
    cube.0 == pond.len() - 1
        || cube.0 == 0
        || cube.1 == pond[0].len() - 1
        || cube.1 == 0
        || cube.2 == pond[0][0].len() - 1
        || cube.2 == 0
}

fn has_surface_access(
    cube: &(usize, usize, usize),
    pond: &Vec<Vec<Vec<bool>>>,
    visited: &mut HashSet<(usize, usize, usize)>,
) -> bool {
    if visited.contains(cube) {
        return false;
    }
    visited.insert(*cube);
    if pond[cube.0][cube.1][cube.2] {
        return false;
    }
    if is_boundary(cube, pond) {
        return true;
    }
    if has_surface_access(&(cube.0 + 1, cube.1, cube.2), pond, visited) {
        return true;
    }
    if has_surface_access(&(cube.0 - 1, cube.1, cube.2), pond, visited) {
        return true;
    }
    if has_surface_access(&(cube.0, cube.1 + 1, cube.2), pond, visited) {
        return true;
    }
    if has_surface_access(&(cube.0, cube.1 - 1, cube.2), pond, visited) {
        return true;
    }
    if has_surface_access(&(cube.0, cube.1, cube.2 + 1), pond, visited) {
        return true;
    }
    if has_surface_access(&(cube.0, cube.1, cube.2 - 1), pond, visited) {
        return true;
    }
    false
}

fn is_empty_cube(
    cube: &(usize, usize, usize),
    pond: &Vec<Vec<Vec<bool>>>,
    needs_surface_access: bool,
) -> usize {
    if !pond[cube.0][cube.1][cube.2] {
        if needs_surface_access {
            let mut visited: HashSet<(usize, usize, usize)> = HashSet::new();
            if has_surface_access(&(cube.0, cube.1, cube.2), pond, &mut visited) {
                1
            } else {
                0
            }
        } else {
            1
        }
    } else {
        0
    }
}

fn get_not_connected_sides_for_cube(
    cube: &(usize, usize, usize),
    pond: &Vec<Vec<Vec<bool>>>,
    needs_surface_access: bool,
) -> usize {
    let mut r: usize = 0;

    if cube.0 < pond.len() - 1 {
        r += is_empty_cube(&(cube.0 + 1, cube.1, cube.2), pond, needs_surface_access);
    } else {
        r += 1;
    }
    if cube.0 > 0 {
        r += is_empty_cube(&(cube.0 - 1, cube.1, cube.2), pond, needs_surface_access);
    } else {
        r += 1;
    }
    if cube.1 < pond[0].len() - 1 {
        r += is_empty_cube(&(cube.0, cube.1 + 1, cube.2), pond, needs_surface_access);
    } else {
        r += 1;
    }
    if cube.1 > 0 {
        r += is_empty_cube(&(cube.0, cube.1 - 1, cube.2), pond, needs_surface_access);
    } else {
        r += 1;
    }
    if cube.2 < pond[0][0].len() - 1 {
        r += is_empty_cube(&(cube.0, cube.1, cube.2 + 1), pond, needs_surface_access);
    } else {
        r += 1;
    }
    if cube.2 > 0 {
        r += is_empty_cube(&(cube.0, cube.1, cube.2 - 1), pond, needs_surface_access);
    } else {
        r += 1;
    }
    r
}

fn get_not_connected_sides(pond: &Vec<Vec<Vec<bool>>>, surface_access: bool) -> usize {
    pond.iter()
        .enumerate()
        .map(|(x, ry)| {
            ry.iter()
                .enumerate()
                .map(|(y, rz)| {
                    rz.iter()
                        .enumerate()
                        .map(|(z, b)| {
                            if *b {
                                get_not_connected_sides_for_cube(&(x, y, z), pond, surface_access)
                            } else {
                                0
                            }
                        })
                        .sum::<usize>()
                })
                .sum::<usize>()
        })
        .sum()
}

fn solution(filename: &str) -> (usize, usize) {
    let cubes: Vec<(usize, usize, usize)> = fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(|l| {
            l.split(',')
                .map(|v| v.parse::<usize>().unwrap())
                .tuples()
                .next()
                .unwrap()
        })
        .collect();

    let max_x = cubes.iter().map(|c| c.0).max().unwrap() + 1;
    let max_y = cubes.iter().map(|c| c.1).max().unwrap() + 1;
    let max_z = cubes.iter().map(|c| c.2).max().unwrap() + 1;

    let mut pond: Vec<Vec<Vec<bool>>> = vec![vec![vec![false; max_z]; max_y]; max_x];
    cubes.iter().for_each(|c| pond[c.0][c.1][c.2] = true);

    (
        get_not_connected_sides(&pond, false),
        get_not_connected_sides(&pond, true),
    )
}

pub fn run() -> (String, String) {
    let (p1, p2) = solution("src/inputs/aoc_18.input");
    (p1.to_string(), p2.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        assert_eq!(solution("src/inputs/aoc_18_sample.input"), (64, 58));
        assert_eq!(solution("src/inputs/aoc_18.input"), (4314, 2444));
    }
}
