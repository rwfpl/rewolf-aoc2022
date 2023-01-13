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
        if self.x as i32 + other.0 < 0 || self.y as i32 + other.1 < 0 {
            None
        } else {
            Some(Pos::new(
                (self.x as i32 + other.0) as usize,
                (self.y as i32 + other.1) as usize,
            ))
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

    fn get_possible_palyer_moves(&self, p: &Pos) -> SmallVec<[Pos; 8]> {
        [(-1, 0), (0, -1), (1, 0), (0, 1)]
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
    map: Map,
    scratch_map: Map,
    reset_map: Map,
}

#[derive(Debug, Constructor, PartialEq, Eq)]
struct GameState {
    map: Map,
    round: i32,
    player: Pos,
}

impl Game {
    fn new(map: Map) -> Self {
        let mut scratch_map = map.clone();
        scratch_map.reset();
        Game { scratch_map: scratch_map.clone(), map, reset_map:scratch_map }
    }

    fn advance_blizzards(map: &mut Map, scratch_map: &mut Map, reset_map: &Map) {
        map.grid.iter().enumerate().for_each(|(index, t)| {
            if let Tile::Blizzard(dirs) = t {
                dirs.iter().for_each(|d| {
                    let p = map.index_to_pos(index);
                    let new_pos = map.advance_blizzard(&p, d);
                    if let Tile::Blizzard(b) = scratch_map.get_tile_mut(new_pos.x, new_pos.y)
                    {
                        b.push(*d);
                    } else {
                        *scratch_map.get_tile_mut(new_pos.x, new_pos.y) =
                            Tile::Blizzard(smallvec![*d; 1]);
                    }
                });
            }
        });
        std::mem::swap(map, scratch_map);
        scratch_map.clone_from(reset_map);
    }

    fn wait_until_move_is_possible(
        map: &mut Map,
        scratch_map: &mut Map,
        reset_map: &Map,
        player: &Pos,
    ) -> Option<(SmallVec<[Pos; 8]>, i32)> {
        let mut rounds = 0;
        let mut moves;
        loop {
            Self::advance_blizzards(map, scratch_map, reset_map);
            moves = map.get_possible_palyer_moves(player);
            rounds += 1;
            if !moves.is_empty() {
                if let Tile::Empty = map.get_tile(player.x, player.y) {
                    // add current position to simulate no move
                    moves.push(*player);
                }
                break;
            }
            if let Tile::Blizzard(_) = &map.get_tile(player.x, player.y) {
                //println!("blizzard moved to the player field");
                return None;
            }
        }
        Some((moves, rounds))
    }

    fn play_bfs(&mut self, player: &Pos, to: &Pos) -> i32 {
        let lcm = (self.map.height - 2).lcm(&(&self.map.width - 2)) as i32;
        let mut visited: HashSet<(Pos, i32)> = HashSet::new();
        let mut next_moves: VecDeque<GameState> = VecDeque::new();
        next_moves.push_back(GameState::new(self.map.clone(), 0, *player));
        loop {
            if next_moves.is_empty() {
                panic!("couldn't find path between {player:?} and {to:?}");
            }
            let mut gs = next_moves.pop_front().unwrap();
            if gs.player == *to {
                self.map = gs.map;
                return gs.round;
            }
            if visited.contains(&(gs.player, gs.round % lcm)) {
                continue;
            }
            visited.insert((gs.player, gs.round % lcm));

            if let Some((moves, wait_rounds)) =
                Self::wait_until_move_is_possible(&mut gs.map, &mut self.scratch_map, &self.reset_map, &gs.player)
            {
                next_moves.extend(
                    moves
                        .into_iter()
                        .map(|mv| GameState::new(gs.map.clone(), gs.round + wait_rounds, mv)),
                );
                next_moves.make_contiguous().sort_by_key(|gs| gs.round);
            }
        }
    }
}

fn solution(filename: &str) -> (i32, i32) {
    let mut game = Game::new(Map::from(fs::read_to_string(filename).unwrap().as_str()));
    let from = game.map.get_entrance();
    let to = game.map.get_exit();
    let p1 = game.play_bfs(&from, &to);
    (
        p1,
        p1 + game.play_bfs(&to, &from) + game.play_bfs(&from, &to),
    )
}

pub fn run() {
    let (p1, p2) = solution("src/inputs/aoc_24.input");
    println!("day24 p1: {p1}");
    println!("day24 p2: {p2}");
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
