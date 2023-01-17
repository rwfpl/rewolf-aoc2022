extern crate derive_more;
extern crate num;
use derive_more::Constructor;
use num::Integer;
use smallvec::*;
use std::collections::VecDeque;
use std::{collections::HashSet, fs, ops::Add};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl From<Direction> for char {
    fn from(d: Direction) -> char {
        match d {
            Direction::Down => 'v',
            Direction::Left => '<',
            Direction::Right => '>',
            Direction::Up => '^',
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Tile {
    Empty,
    Wall,
    Blizzard(SmallVec<[Direction; 4]>),
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '.' => Tile::Empty,
            '#' => Tile::Wall,
            '^' => Tile::Blizzard(smallvec![Direction::Up; 1]),
            '<' => Tile::Blizzard(smallvec![Direction::Left; 1]),
            '>' => Tile::Blizzard(smallvec![Direction::Right; 1]),
            'v' => Tile::Blizzard(smallvec![Direction::Down; 1]),
            _ => panic!("Invalid tile: {c}"),
        }
    }
}

impl From<&Tile> for char {
    fn from(t: &Tile) -> char {
        match t {
            Tile::Wall => '#',
            Tile::Empty => '.',
            Tile::Blizzard(v) => {
                if v.len() == 1 {
                    char::from(v[0])
                } else {
                    v.len().to_string().chars().next().unwrap()
                }
            }
        }
    }
}

#[derive(Debug, Constructor, Copy, Clone, PartialEq, Eq, Hash)]
struct Pos {
    x: usize,
    y: usize,
}

impl Add<(i32, i32)> for Pos {
    type Output = Option<Self>;

    fn add(self, other: (i32, i32)) -> Option<Self> {
        let x = self.x as i32 + other.0;
        let y = self.y as i32 + other.1;
        if x < 0 || y < 0 {
            None
        } else {
            Some(Pos::new(x as usize, y as usize))
        }
    }
}

#[derive(Debug, Constructor, Clone, PartialEq, Eq)]
struct Map {
    grid: Vec<Tile>,
    width: usize,
    height: usize,
}

impl From<&str> for Map {
    fn from(s: &str) -> Self {
        Self {
            grid: s
                .lines()
                .flat_map(|l| l.chars().map(Tile::from).collect::<Vec<Tile>>())
                .collect::<Vec<Tile>>(),
            width: s.lines().next().unwrap().len(),
            height: s.lines().count(),
        }
    }
}

impl Map {
    fn index_to_pos(&self, index: usize) -> Pos {
        Pos {
            x: index % self.width,
            y: index / self.width,
        }
    }

    fn pos_to_index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    fn get_tile(&self, x: usize, y: usize) -> &Tile {
        &self.grid[self.pos_to_index(x, y)]
    }

    fn get_tile_mut(&mut self, x: usize, y: usize) -> &mut Tile {
        let index = self.pos_to_index(x, y);
        self.grid.get_mut(index).unwrap()
    }

    fn reset(&mut self) {
        for tile in &mut self.grid {
            if tile != &Tile::Wall {
                *tile = Tile::Empty;
            }
        }
    }

    fn get_entrance(&self) -> Pos {
        self.index_to_pos(
            self.grid
                .iter()
                .enumerate()
                .find(|(_, t)| t == &&Tile::Empty)
                .unwrap()
                .0,
        )
    }

    fn get_exit(&self) -> Pos {
        self.index_to_pos(
            self.grid
                .iter()
                .enumerate()
                .rev()
                .find(|(_, t)| t == &&Tile::Empty)
                .unwrap()
                .0,
        )
    }

    #[allow(dead_code)]
    fn print(&self) {
        self.grid.iter().enumerate().for_each(|(i, t)| {
            if i % self.width == 0 {
                println!();
            }
            print!("{}", char::from(t));
        });
        println!();
    }

    fn blizzard_can_move(&self, p: &Pos, d: &Direction) -> Pos {
        match self.get_tile(p.x, p.y) {
            Tile::Empty => *p,
            Tile::Blizzard(_) => *p,
            Tile::Wall => match d {
                Direction::Down => Pos::new(p.x, 1),
                Direction::Up => Pos::new(p.x, self.height - 2),
                Direction::Left => Pos::new(self.width - 2, p.y),
                Direction::Right => Pos::new(1, p.y),
            },
        }
    }

    fn advance_blizzard(&self, p: &Pos, d: &Direction) -> Pos {
        match d {
            Direction::Down => self.blizzard_can_move(&p.add((0, 1)).unwrap(), d),
            Direction::Left => self.blizzard_can_move(&p.add((-1, 0)).unwrap(), d),
            Direction::Right => self.blizzard_can_move(&p.add((1, 0)).unwrap(), d),
            Direction::Up => self.blizzard_can_move(&p.add((0, -1)).unwrap(), d),
        }
    }

    fn get_possible_palyer_moves(&self, p: &Pos) -> SmallVec<[Pos; 5]> {
        [(0, 0), (-1, 0), (0, -1), (1, 0), (0, 1)]
            .iter()
            .map(|v| p.add(*v))
            .filter(|np| {
                np.is_some_and(|np| {
                    np.y != self.height && self.get_tile(np.x, np.y) == &Tile::Empty
                })
            })
            .map(|np| np.unwrap())
            .collect()
    }
}

#[derive(Debug)]
struct Game {
    lcm: i32,
    maps: Vec<Map>,
    round: i32,
}

#[derive(Debug, Constructor, PartialEq, Eq)]
struct GameState {
    round: i32,
    player: Pos,
}

impl Game {
    fn new(map: Map) -> Self {
        let lcm = (map.height - 2).lcm(&(&map.width - 2)) as i32;
        let mut reset_map = map.clone();
        reset_map.reset();

        let mut maps: Vec<Map> = Vec::with_capacity(lcm as usize);
        let mut map = map;
        for _ in 0..lcm {
            let next_map = Self::next_blizzard_map(&map, &reset_map);
            maps.push(map);
            map = next_map;
        }

        Game {
            lcm,
            maps,
            round: 0,
        }
    }

    fn next_blizzard_map(map: &Map, reset_map: &Map) -> Map {
        let mut r = reset_map.clone();
        map.grid.iter().enumerate().for_each(|(index, t)| {
            if let Tile::Blizzard(dirs) = t {
                dirs.iter().for_each(|d| {
                    let p = map.index_to_pos(index);
                    let new_pos = map.advance_blizzard(&p, d);
                    if let Tile::Blizzard(b) = r.get_tile_mut(new_pos.x, new_pos.y) {
                        b.push(*d);
                    } else {
                        *r.get_tile_mut(new_pos.x, new_pos.y) = Tile::Blizzard(smallvec![*d; 1]);
                    }
                });
            }
        });
        r
    }

    fn wait_until_move_is_possible(&mut self, player: &Pos) -> Option<(SmallVec<[Pos; 5]>, i32)> {
        let mut rounds = 0;
        loop {
            let moves =
                self.maps[(self.round % self.lcm) as usize].get_possible_palyer_moves(player);
            rounds += 1;
            if !moves.is_empty() {
                return Some((moves, rounds));
            }
            if let Tile::Blizzard(_) =
                &self.maps[(self.round % self.lcm) as usize].get_tile(player.x, player.y)
            {
                //println!("blizzard moved to the player field");
                return None;
            }
            self.round += 1;
        }
    }

    fn play_bfs(&mut self, player: &Pos, to: &Pos) -> i32 {
        let mut visited: HashSet<(Pos, i32)> = HashSet::with_capacity(256_000);
        let mut next_moves: VecDeque<GameState> = VecDeque::with_capacity(4096);
        next_moves.push_back(GameState::new(self.round, *player));
        loop {
            if next_moves.is_empty() {
                panic!("couldn't find path between {player:?} and {to:?}");
            }
            let gs = next_moves.pop_front().unwrap();
            self.round = gs.round;
            if gs.player == *to {
                return gs.round - 1;
            }
            if !visited.insert((gs.player, gs.round % self.lcm)) {
                continue;
            }

            if let Some((moves, wait_rounds)) = self.wait_until_move_is_possible(&gs.player) {
                next_moves.extend(
                    moves
                        .into_iter()
                        .map(|mv| GameState::new(gs.round + wait_rounds, mv)),
                );
                next_moves.make_contiguous().sort_by_key(|gs| gs.round);
            }
        }
    }
}

fn solution(filename: &str) -> (i32, i32) {
    let mut game = Game::new(Map::from(fs::read_to_string(filename).unwrap().as_str()));
    let from = game.maps[0].get_entrance();
    let to = game.maps[0].get_exit();
    let p1 = game.play_bfs(&from, &to);
    game.play_bfs(&to, &from);
    (p1, game.play_bfs(&from, &to))
}

pub fn run() -> (String, String) {
    let (p1, p2) = solution("src/inputs/aoc_24.input");
    (p1.to_string(), p2.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blizzard_moves() {
        let m = Map::from("#####\n#...#\n#...#\n#...#\n#####");
        assert_eq!(
            m.advance_blizzard(&Pos::new(2, 2), &Direction::Down),
            Pos::new(2, 3)
        );
        assert_eq!(
            m.advance_blizzard(&Pos::new(2, 3), &Direction::Down),
            Pos::new(2, 1)
        );

        assert_eq!(
            m.advance_blizzard(&Pos::new(2, 2), &Direction::Up),
            Pos::new(2, 1)
        );
        assert_eq!(
            m.advance_blizzard(&Pos::new(2, 1), &Direction::Up),
            Pos::new(2, 3)
        );

        assert_eq!(
            m.advance_blizzard(&Pos::new(2, 2), &Direction::Left),
            Pos::new(1, 2)
        );
        assert_eq!(
            m.advance_blizzard(&Pos::new(1, 2), &Direction::Left),
            Pos::new(3, 2)
        );

        assert_eq!(
            m.advance_blizzard(&Pos::new(2, 2), &Direction::Right),
            Pos::new(3, 2)
        );
        assert_eq!(
            m.advance_blizzard(&Pos::new(3, 2), &Direction::Right),
            Pos::new(1, 2)
        );
    }

    #[test]
    fn test_run() {
        assert_eq!(solution("src/inputs/aoc_24_sample.input"), (18, 54));
        assert_eq!(solution("src/inputs/aoc_24.input"), (314, 896));
    }
}
