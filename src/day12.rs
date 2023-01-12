extern crate derive_more;
use std::{
    collections::{HashSet, VecDeque},
    fs,
};

use derive_more::Constructor;
#[derive(Debug, Default, Constructor, PartialEq, Eq, Hash, Clone, Copy)]
struct Pos {
    row: usize,
    column: usize,
}

#[derive(Debug, Default)]
struct Map {
    map: Vec<Vec<u8>>,
    start: Pos,
    end: Pos,
}

impl Map {
    fn from_str(s: &str) -> Self {
        let mut start = Default::default();
        let mut end = Default::default();
        let map = s
            .lines()
            .enumerate()
            .map(|(row, line)| {
                line.bytes()
                    .enumerate()
                    .map(|(column, t)| match t {
                        b'S' => {
                            start = Pos::new(row, column);
                            b'a'
                        }
                        b'E' => {
                            end = Pos::new(row, column);
                            b'z'
                        }
                        _ => t,
                    })
                    .collect()
            })
            .collect();
        Map { map, start, end }
    }

    fn can_move(&self, p: &Pos, dest_elevation: u8) -> bool {
        self.map[p.row][p.column] >= dest_elevation
            || self.map[p.row][p.column] + 1 == dest_elevation
    }

    fn can_move_up(&self, pos: &Pos) -> bool {
        if pos.row > 0 {
            self.can_move(pos, self.map[pos.row - 1][pos.column])
        } else {
            false
        }
    }

    fn can_move_down(&self, pos: &Pos) -> bool {
        if pos.row < self.map.len() - 1 {
            self.can_move(pos, self.map[pos.row + 1][pos.column])
        } else {
            false
        }
    }

    fn can_move_left(&self, pos: &Pos) -> bool {
        if pos.column > 0 {
            self.can_move(pos, self.map[pos.row][pos.column - 1])
        } else {
            false
        }
    }

    fn can_move_right(&self, pos: &Pos) -> bool {
        if pos.column < self.map[0].len() - 1 {
            self.can_move(pos, self.map[pos.row][pos.column + 1])
        } else {
            false
        }
    }

    fn bfs(&self, start: &Pos) -> Option<usize> {
        let mut visited: HashSet<Pos> = HashSet::new();
        let mut q: VecDeque<(Pos, usize)> = VecDeque::new();
        q.push_back((*start, 0));

        loop {
            if q.is_empty() {
                return None;
            }
            let p = q.pop_front().unwrap();
            if !visited.insert(p.0) {
                continue;
            }
            if p.0 == self.end {
                return Some(p.1);
            }
            if self.can_move_right(&p.0) {
                q.push_back((Pos::new(p.0.row, p.0.column + 1), p.1 + 1));
            }
            if self.can_move_down(&p.0) {
                q.push_back((Pos::new(p.0.row + 1, p.0.column), p.1 + 1));
            }
            if self.can_move_up(&p.0) {
                q.push_back((Pos::new(p.0.row - 1, p.0.column), p.1 + 1));
            }
            if self.can_move_left(&p.0) {
                q.push_back((Pos::new(p.0.row, p.0.column - 1), p.1 + 1));
            }
        }
    }
}

fn solution(filename: &str) -> (usize, usize) {
    let map: Map = Map::from_str(&fs::read_to_string(filename).unwrap());
    (
        map.bfs(&map.start).unwrap(),
        map.map
            .iter()
            .enumerate()
            .map(|(i, row)| {
                row.iter()
                    .enumerate()
                    .filter(|(_, t)| t == &&b'a')
                    .map(|(j, _)| map.bfs(&Pos::new(i, j)).unwrap_or(usize::MAX))
                    .min()
                    .unwrap()
            })
            .min()
            .unwrap(),
    )
}

pub fn run() {
    let (p1, p2) = solution("src/inputs/aoc_12.input");
    println!("part12 p1: {p1}");
    println!("part12 p2: {p2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        assert_eq!(solution("src/inputs/aoc_12_sample.input"), (31, 29));
        assert_eq!(solution("src/inputs/aoc_12.input"), (437, 430));
    }
}
