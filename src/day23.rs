extern crate derive_more;
use derive_more::Constructor;
use std::{collections::HashMap, fs, ops::Add};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Elf,
    Empty,
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '#' => Tile::Elf,
            '.' => Tile::Empty,
            _ => panic!("Invalid tile character"),
        }
    }
}

#[derive(Debug, Constructor, Copy, Clone, PartialEq, Eq, Hash)]
struct Pos {
    x: usize,
    y: usize,
}

impl Add<(i32, i32)> for Pos {
    type Output = Self;

    fn add(self, other: (i32, i32)) -> Self {
        if self.x as i32 + other.0 < 0 || self.y as i32 + other.1 < 0 {
            panic!("Pos cannot be negative.");
        }
        Pos::new(
            (self.x as i32 + other.0) as usize,
            (self.y as i32 + other.1) as usize,
        )
    }
}

#[derive(Debug)]
struct Game {
    map: Vec<Vec<Tile>>,
}

#[derive(Debug)]
enum Direction {
    North,
    South,
    West,
    East,
}

#[allow(dead_code)]
fn print_map(map: &[Vec<Tile>]) {
    map.iter().for_each(|row| {
        row.iter().for_each(|t| {
            print!(
                "{}",
                match t {
                    Tile::Elf => '#',
                    Tile::Empty => '.',
                }
            );
        });
        println!()
    });
}

impl Game {
    fn new(input: Vec<Vec<Tile>>) -> Self {
        let map_width = input[0].len();
        let map_height = input.len();
        Game {
            // Pad original map with Empty tiles. New map is 3x bigger.
            map: (0..map_height)
                .map(|_| vec![Tile::Empty; 3 * map_width])
                .chain(
                    input
                        .into_iter()
                        .map(|iv| {
                            (0..map_width)
                                .map(|_| Tile::Empty)
                                .chain(iv)
                                .chain((0..map_width).map(|_| Tile::Empty))
                                .collect::<Vec<Tile>>()
                        })
                        .collect::<Vec<Vec<Tile>>>(),
                )
                .chain((0..map_height).map(|_| vec![Tile::Empty; 3 * map_width]))
                .collect(),
        }
    }

    fn map(&self, pos: &Pos) -> Tile {
        self.map[pos.y][pos.x]
    }

    fn are_empty(&self, pos: &Pos, fields: &[(i32, i32)]) -> bool {
        fields
            .iter()
            .map(|v| self.map(&pos.add(*v)) == Tile::Empty)
            .all(|r| r)
    }

    fn no_other_elves_around(&self, pos: &Pos) -> bool {
        self.are_empty(
            pos,
            &[
                (-1, 1),
                (0, 1),
                (1, 1),
                (-1, 0),
                (1, 0),
                (-1, -1),
                (0, -1),
                (1, -1),
            ],
        )
    }

    fn check_any(&self, pos: &Pos, fields: &[(i32, i32)], add_pos: (i32, i32)) -> Option<Pos> {
        if self.are_empty(pos, fields) {
            Some(pos.add(add_pos))
        } else {
            None
        }
    }

    fn check_north(&self, pos: &Pos) -> Option<Pos> {
        self.check_any(pos, &[(-1, -1), (0, -1), (1, -1)], (0, -1))
    }

    fn check_south(&self, pos: &Pos) -> Option<Pos> {
        self.check_any(pos, &[(-1, 1), (0, 1), (1, 1)], (0, 1))
    }

    fn check_west(&self, pos: &Pos) -> Option<Pos> {
        self.check_any(pos, &[(-1, 1), (-1, 0), (-1, -1)], (-1, 0))
    }

    fn check_east(&self, pos: &Pos) -> Option<Pos> {
        self.check_any(pos, &[(1, 1), (1, 0), (1, -1)], (1, 0))
    }

    fn check_direction(&self, d: &Direction, p: &Pos) -> Option<Pos> {
        match d {
            Direction::East => self.check_east(p),
            Direction::West => self.check_west(p),
            Direction::North => self.check_north(p),
            Direction::South => self.check_south(p),
        }
    }

    fn get_number_of_empty_tiles(&self) -> i32 {
        macro_rules! scan_map {
            ($min_max:ident, $min_max_const:expr, $column:literal) => {
                self.map
                    .iter()
                    .enumerate()
                    .map(|(y, row)| {
                        row.iter()
                            .enumerate()
                            .filter(|(_, t)| **t == Tile::Elf)
                            .map(|(x, _)| if $column { y } else { x })
                            .$min_max()
                            .unwrap_or($min_max_const)
                    })
                    .$min_max()
                    .unwrap()
            };
        }

        let left = scan_map!(min, usize::MAX, false);
        let right = scan_map!(max, 0, false);
        let top = scan_map!(min, usize::MAX, true);
        let bottom = scan_map!(max, 0, true);

        self.map
            .iter()
            .enumerate()
            .filter(|(y, _)| (top..=bottom).contains(y))
            .map(|(_, row)| {
                row.iter()
                    .enumerate()
                    .filter(|(x, t)| (left..=right).contains(x) && t == &&Tile::Empty)
                    .count()
            })
            .sum::<usize>() as i32
    }

    fn get_elves(&self) -> impl Iterator<Item = Pos> + '_ {
        self.map.iter().enumerate().flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, t)| **t == Tile::Elf)
                .map(|(x, _)| Pos::new(x, y))
                .collect::<Vec<Pos>>()
        })
    }

    fn play(&mut self, number_of_rounds: usize) -> (i32, usize) {
        let direction = [
            Direction::North,
            Direction::South,
            Direction::West,
            Direction::East,
        ];
        let mut round = 0;
        let mut p1 = 0;
        loop {
            let elves = self.get_elves();

            let mut proposed_moves: HashMap<Pos, Pos> = HashMap::with_capacity(2048);
            let mut proposed_pos_count: HashMap<Pos, i32> = HashMap::with_capacity(2048);
            elves
                .filter(|elf| !self.no_other_elves_around(elf))
                .for_each(|elf| {
                    for dir_i in 0..direction.len() {
                        if let Some(new_pos) = self
                            .check_direction(&direction[(round + dir_i) % direction.len()], &elf)
                        {
                            proposed_moves.insert(elf, new_pos);
                            proposed_pos_count
                                .entry(new_pos)
                                .and_modify(|cnt| *cnt += 1)
                                .or_insert(1);
                            break;
                        }
                    }
                });
            // execute moves
            let moves = proposed_moves
                .iter()
                .filter(|(_, dst)| proposed_pos_count.get(dst).unwrap() <= &1)
                .map(|(src, dst)| {
                    self.map[src.y][src.x] = Tile::Empty;
                    self.map[dst.y][dst.x] = Tile::Elf;
                })
                .count();

            round += 1;

            if round == number_of_rounds {
                p1 = self.get_number_of_empty_tiles();
            }
            if moves == 0 {
                break;
            }
        }
        (p1, round)
    }
}

fn read_input(filename: &str) -> Vec<Vec<Tile>> {
    input_from_string(&fs::read_to_string(filename).unwrap())
}

fn input_from_string(s: &str) -> Vec<Vec<Tile>> {
    s.lines()
        .map(|l| l.chars().map(Tile::from).collect())
        .collect()
}

fn solution(filename: &str) -> (i32, usize) {
    let mut game = Game::new(read_input(filename));
    game.play(10)
}

pub fn run() -> (String, String) {
    let (p1, p2) = solution("src/inputs/aoc_23.input");
    (p1.to_string(), p2.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_elves() {
        let game = Game::new(read_input("src/inputs/aoc_23_sample.input"));
        let elves = game.get_elves();
        elves.for_each(|elf| {
            assert_eq!(game.map[elf.y][elf.x], Tile::Elf);
        });
    }

    #[test]
    fn test_no_other_elves_around() {
        let game = Game::new(input_from_string("...\n.#.\n..."));
        let elves = game.get_elves().collect::<Vec<Pos>>();
        assert_eq!(elves.len(), 1);
        assert!(game.no_other_elves_around(&elves[0]));

        let game = Game::new(input_from_string(".#.\n.#.\n..."));
        let elves = game.get_elves().collect::<Vec<Pos>>();
        assert_eq!(elves.len(), 2);
        assert!(!game.no_other_elves_around(&elves[0]));
        assert!(!game.no_other_elves_around(&elves[1]));
    }

    #[test]
    fn test_check_north() {
        let game = Game::new(input_from_string("...\n###\n###"));
        let elves = game.get_elves().collect::<Vec<Pos>>();
        assert_eq!(elves.len(), 6);
        assert_eq!(game.check_north(&elves[1]), Some(elves[1].add((0, -1))));

        let game = Game::new(input_from_string(".#.\n.#.\n..."));
        let elves = game.get_elves().collect::<Vec<Pos>>();
        assert_eq!(elves.len(), 2);
        assert!(game.check_north(&elves[1]).is_none());
    }

    #[test]
    fn test_check_south() {
        let game = Game::new(input_from_string("###\n###\n..."));
        let elves = game.get_elves().collect::<Vec<Pos>>();
        assert_eq!(elves.len(), 6);
        assert_eq!(game.check_south(&elves[4]), Some(elves[4].add((0, 1))));

        let game = Game::new(input_from_string("...\n.#.\n.#."));
        let elves = game.get_elves().collect::<Vec<Pos>>();
        assert_eq!(elves.len(), 2);
        assert!(game.check_south(&elves[0]).is_none());
    }

    #[test]
    fn test_check_west() {
        let game = Game::new(input_from_string(".##\n.##\n.##"));
        let elves = game.get_elves().collect::<Vec<Pos>>();
        assert_eq!(elves.len(), 6);
        assert_eq!(game.check_west(&elves[2]), Some(elves[2].add((-1, 0))));

        let game = Game::new(input_from_string("...\n##.\n..."));
        let elves = game.get_elves().collect::<Vec<Pos>>();
        assert_eq!(elves.len(), 2);
        assert!(game.check_west(&elves[1]).is_none());
    }

    #[test]
    fn test_check_east() {
        let game = Game::new(input_from_string("##.\n##.\n##."));
        let elves = game.get_elves().collect::<Vec<Pos>>();
        assert_eq!(elves.len(), 6);
        assert_eq!(game.check_east(&elves[3]), Some(elves[3].add((1, 0))));

        let game = Game::new(input_from_string("...\n.#.\n..#"));
        let elves = game.get_elves().collect::<Vec<Pos>>();
        assert_eq!(elves.len(), 2);
        assert!(game.check_east(&elves[0]).is_none());
    }

    #[test]
    fn test_run() {
        assert_eq!(solution("src/inputs/aoc_23_sample.input"), (110, 20));
        assert_eq!(solution("src/inputs/aoc_23.input"), (4056, 999));
    }
}
