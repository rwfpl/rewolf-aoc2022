extern crate derive_more;

use derive_more::Constructor;
use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::iter;
use std::ops::Add;

#[derive(Debug)]
enum Rotate {
    None,
    Left,
    Right,
}

impl From<char> for Rotate {
    fn from(c: char) -> Self {
        match c {
            'R' => Rotate::Right,
            'L' => Rotate::Left,
            _ => Rotate::None,
        }
    }
}

#[derive(Debug)]
struct Move {
    steps: usize,
    rotate: Rotate,
}

impl From<regex::Captures<'_>> for Move {
    fn from(capture: regex::Captures) -> Self {
        Move {
            steps: capture
                .name("steps")
                .unwrap()
                .as_str()
                .parse::<usize>()
                .unwrap(),
            rotate: Rotate::from(
                capture
                    .name("rot")
                    .unwrap()
                    .as_str()
                    .chars()
                    .next()
                    .unwrap_or('X'),
            ),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Tile {
    None,
    Floor,
    Wall,
    Teleport(HashMap<Direction, (Pos, Direction)>),
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '#' => Tile::Wall,
            '.' => Tile::Floor,
            _ => Tile::None,
        }
    }
}

#[derive(Debug, Constructor, Clone, Copy, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl From<(i32, i32)> for Direction {
    fn from(t: (i32, i32)) -> Self {
        match t {
            (0, 1) => Direction::Down,
            (0, -1) => Direction::Up,
            (1, 0) => Direction::Right,
            (-1, 0) => Direction::Left,
            _ => panic!("wrong tuple"),
        }
    }
}

impl From<Direction> for (i32, i32) {
    fn from(val: Direction) -> Self {
        match val {
            Direction::Up => (0, -1),
            Direction::Left => (-1, 0),
            Direction::Down => (0, 1),
            Direction::Right => (1, 0),
        }
    }
}

impl From<Direction> for usize {
    fn from(val: Direction) -> Self {
        match val {
            Direction::Right => 0,
            Direction::Down => 1,
            Direction::Left => 2,
            Direction::Up => 3,
        }
    }
}

impl Direction {
    fn rotate(&self, r: &Rotate) -> Self {
        match r {
            Rotate::None => *self,
            Rotate::Right => match self {
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Up,
                Direction::Up => Direction::Right,
                Direction::Right => Direction::Down,
            },
            Rotate::Left => match self {
                Direction::Up => Direction::Left,
                Direction::Left => Direction::Down,
                Direction::Down => Direction::Right,
                Direction::Right => Direction::Up,
            },
        }
    }

    fn opposite(&self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Left => Direction::Right,
            Direction::Down => Direction::Up,
            Direction::Right => Direction::Left,
        }
    }
}

#[derive(Debug)]
struct Game {
    map: Vec<Vec<Tile>>,
    moves: Vec<Move>,
    pos: Pos,
    dir: Direction,
}

impl Game {
    fn new(map: Vec<Vec<Tile>>, moves: Vec<Move>) -> Self {
        Game {
            moves,
            dir: Direction::Right,
            pos: Pos::new(
                map[1]
                    .iter()
                    .enumerate()
                    .find(|(_, tile)| **tile == Tile::Floor)
                    .unwrap()
                    .0,
                1,
            ),
            map,
        }
    }

    #[allow(dead_code)]
    fn print_map(&self) {
        self.map.iter().for_each(|row| {
            row.iter().for_each(|t| {
                print!(
                    "{}",
                    match t {
                        Tile::Floor => ".",
                        Tile::None => " ",
                        Tile::Wall => "#",
                        Tile::Teleport(t) =>
                            if t.len() == 1 {
                                "T"
                            } else {
                                "X"
                            },
                    }
                )
            });
            println!()
        })
    }

    fn is_wall(&self, x: usize, y: usize) -> bool {
        let y = y % self.map.len();
        let x = x % self.map[y].len();
        self.map[y][x] == Tile::Wall
    }

    fn advance(&self, p: &Pos, adv: (i32, i32)) -> (Pos, (i32, i32)) {
        let mut y = p.y;
        let mut x = p.x;
        if adv.0 != 0 && adv.1 != 0 {
            panic!("both values != 0: {p:?}");
        }
        while self.map[y][x] == Tile::None {
            if adv.1 != 0 {
                if adv.1 > 0 {
                    y += 1;
                } else if y == 0 {
                    y = self.map.len() - 1;
                } else {
                    y -= 1
                }
                y %= self.map.len();
            } else {
                if adv.0 > 0 {
                    x += 1;
                } else if x == 0 {
                    x = self.map[y].len() - 1;
                } else {
                    x -= 1
                }
                x %= self.map[y].len();
            }
        }
        if let Tile::Teleport(dest) = &self.map[y][x] {
            let r = dest.get(&Direction::from(adv)).unwrap();
            return (r.0, r.1.into());
        }
        (Pos::new(x, y), adv)
    }

    fn move_xxx(&self, pos: &Pos, steps: usize, dir: (i32, i32)) -> (Pos, (i32, i32)) {
        let mut pos = *pos;
        let mut dir = dir;
        for _ in 0..steps {
            let (new_pos, new_dir) = self.advance(&pos.add(dir), dir);
            if self.is_wall(new_pos.x, new_pos.y) {
                return (pos, dir);
            }
            pos = new_pos;
            dir = new_dir;
        }
        (pos, dir)
    }

    fn mov(&self, pos: &Pos, dir: &Direction, m: &Move) -> (Pos, Direction) {
        let p = self.move_xxx(pos, m.steps, <(i32, i32)>::from(*dir));
        (p.0, Direction::from(p.1).rotate(&m.rotate))
    }

    fn play(&self) -> (Pos, Direction) {
        let mut pos = self.pos;
        let mut dir = self.dir;
        for m in &self.moves {
            (pos, dir) = self.mov(&pos, &dir, m);
        }
        (pos, dir)
    }
}

fn cube_face_side_vector(
    cube_face: Pos,
    cube_size: usize,
    side: Direction,
) -> impl Iterator<Item = Pos> {
    (0..cube_size).map(move |i| match side {
        Direction::Down => Pos::new(
            cube_face.x * cube_size + i + 1,
            (cube_face.y + 1) * cube_size + 1,
        ),
        Direction::Up => Pos::new(cube_face.x * cube_size + i + 1, cube_face.y * cube_size),
        Direction::Left => Pos::new(cube_face.x * cube_size, cube_face.y * cube_size + i + 1),
        Direction::Right => Pos::new(
            (cube_face.x + 1) * cube_size + 1,
            cube_face.y * cube_size + i + 1,
        ),
    })
}

fn cube_face_vector(
    cube_face: Pos,
    cube_size: usize,
    side: Direction,
    reverse: bool,
) -> Box<dyn Iterator<Item = Pos>> {
    let r = (0..cube_size).map(move |i| match side {
        Direction::Down => Pos::new(
            cube_face.x * cube_size + i + 1,
            (cube_face.y + 1) * cube_size,
        ),
        Direction::Up => Pos::new(cube_face.x * cube_size + i + 1, cube_face.y * cube_size + 1),
        Direction::Left => Pos::new(cube_face.x * cube_size + 1, cube_face.y * cube_size + i + 1),
        Direction::Right => Pos::new(
            (cube_face.x + 1) * cube_size,
            cube_face.y * cube_size + i + 1,
        ),
    });
    if reverse {
        Box::new(r.rev())
    } else {
        Box::new(r)
    }
}

#[derive(Debug)]
struct Teleport {
    from_pos: Pos,
    from_side: Direction,
    to_pos: Pos,
    to_side: Direction,
    to_reverse: bool,
}

impl Teleport {
    fn from(
        from_side: Direction,
        from_pos: (usize, usize),
        to_side: Direction,
        to_pos: (usize, usize),
        to_reverse: bool,
    ) -> Teleport {
        Teleport {
            from_pos: Pos::new(from_pos.0, from_pos.1),
            from_side,
            to_pos: Pos::new(to_pos.0, to_pos.1),
            to_side,
            to_reverse,
        }
    }
}

fn apply_teleport(
    game: &mut Game,
    from: impl Iterator<Item = Pos>,
    from_side: &Direction,
    to: impl Iterator<Item = Pos>,
    to_side: &Direction,
) {
    for (from_pos, to_pos) in from.zip(to) {
        if let Tile::Teleport(cp) = &mut game.map[from_pos.y][from_pos.x] {
            cp.insert(*from_side, (to_pos, to_side.opposite()));
        } else {
            game.map[from_pos.y][from_pos.x] =
                Tile::Teleport(HashMap::from([(*from_side, (to_pos, to_side.opposite()))]));
        }
    }
}

fn apply_teleport_in_both_direction(game: &mut Game, teleport: &Teleport, cube_size: usize) {
    // one direction
    apply_teleport(
        game,
        cube_face_side_vector(teleport.from_pos, cube_size, teleport.from_side),
        &teleport.from_side,
        cube_face_vector(
            teleport.to_pos,
            cube_size,
            teleport.to_side,
            teleport.to_reverse,
        ),
        &teleport.to_side,
    );
    // other direction
    apply_teleport(
        game,
        cube_face_side_vector(teleport.to_pos, cube_size, teleport.to_side),
        &teleport.to_side,
        cube_face_vector(
            teleport.from_pos,
            cube_size,
            teleport.from_side,
            teleport.to_reverse,
        ),
        &teleport.from_side,
    );
}

fn apply_teleports(game: &mut Game, teleports: &[Teleport], cube_size: usize) {
    teleports.iter().for_each(|tp| {
        apply_teleport_in_both_direction(game, tp, cube_size);
    });
}

fn solution(filename: &str, p2: bool, cube_size: usize, teleports: &[Teleport]) -> usize {
    let moves_re = Regex::new(r"(?P<steps>\d+)(?P<rot>[R|L]*)+").unwrap();
    let input = fs::read_to_string(filename).unwrap();

    let max_width = input
        .lines()
        .filter(|l| !(l.is_empty() || ('0'..'9').contains(&l.chars().next().unwrap())))
        .map(|l| l.len())
        .max()
        .unwrap();
    let mut game = Game::new(
        iter::once(" ".repeat(max_width).as_str())
            .chain(
                input
                    .lines()
                    .chain(iter::once(" ".repeat(max_width).as_str())),
            )
            .filter(|l| !(l.is_empty() || ('0'..'9').contains(&l.chars().next().unwrap())))
            .map(|l| {
                " ".chars()
                    .chain(l.chars().chain(" ".repeat(max_width + 1 - l.len()).chars()))
                    .map(Tile::from)
                    .collect::<Vec<Tile>>()
            })
            .collect::<Vec<Vec<Tile>>>(),
        moves_re
            .captures_iter(input.as_str())
            .map(Move::from)
            .collect::<Vec<Move>>(),
    );

    if p2 {
        apply_teleports(&mut game, teleports, cube_size);
    }

    let (pos, dir) = game.play();
    1000 * pos.y + 4 * pos.x + usize::from(dir)
}

fn get_teleports() -> Vec<Teleport> {
    vec![
        Teleport::from(Direction::Right, (1, 1), Direction::Down, (2, 0), false),
        Teleport::from(Direction::Up, (0, 2), Direction::Left, (1, 1), false),
        Teleport::from(Direction::Right, (0, 3), Direction::Down, (1, 2), false),
        Teleport::from(Direction::Right, (1, 2), Direction::Right, (2, 0), true),
        Teleport::from(Direction::Left, (0, 2), Direction::Left, (1, 0), true),
        Teleport::from(Direction::Left, (0, 3), Direction::Up, (1, 0), false),
        Teleport::from(Direction::Down, (0, 3), Direction::Up, (2, 0), false),
    ]
}

#[allow(dead_code)]
fn get_sample_teleports() -> Vec<Teleport> {
    vec![
        Teleport::from(Direction::Right, (2, 1), Direction::Up, (3, 2), true),
        Teleport::from(Direction::Right, (2, 0), Direction::Right, (3, 2), true),
        Teleport::from(Direction::Up, (1, 1), Direction::Left, (2, 0), false),
        Teleport::from(Direction::Up, (0, 1), Direction::Up, (2, 0), true),
        Teleport::from(Direction::Left, (2, 2), Direction::Down, (1, 1), true),
        Teleport::from(Direction::Down, (0, 1), Direction::Down, (2, 2), true),
        Teleport::from(Direction::Left, (0, 1), Direction::Down, (3, 2), true),
    ]
}

pub fn run() -> (String, String) {
    (
        solution("src/inputs/aoc_22.input", false, 50, &get_teleports()).to_string(),
        solution("src/inputs/aoc_22.input", true, 50, &get_teleports()).to_string(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_p1() {
        assert_eq!(
            solution(
                "src/inputs/aoc_22_sample.input",
                false,
                4,
                &get_sample_teleports()
            ),
            6032
        );
        assert_eq!(
            solution("src/inputs/aoc_22.input", false, 50, &get_teleports()),
            27492
        );
    }

    #[test]
    fn test_run_p2() {
        assert_eq!(
            solution(
                "src/inputs/aoc_22_sample.input",
                true,
                4,
                &get_sample_teleports()
            ),
            5031
        );
        assert_eq!(
            solution("src/inputs/aoc_22.input", true, 50, &get_teleports()),
            78291
        );
    }

    #[test]
    fn test_cube_face_vector() {
        assert_eq!(
            cube_face_vector(Pos::new(0, 0), 2, Direction::Down, false).collect::<Vec<Pos>>(),
            vec![Pos::new(1, 2), Pos::new(2, 2)]
        );
        assert_eq!(
            cube_face_vector(Pos::new(0, 0), 2, Direction::Up, false).collect::<Vec<Pos>>(),
            vec![Pos::new(1, 1), Pos::new(2, 1)]
        );
        assert_eq!(
            cube_face_vector(Pos::new(0, 0), 2, Direction::Left, false).collect::<Vec<Pos>>(),
            vec![Pos::new(1, 1), Pos::new(1, 2)]
        );
        assert_eq!(
            cube_face_vector(Pos::new(0, 0), 2, Direction::Right, false).collect::<Vec<Pos>>(),
            vec![Pos::new(2, 1), Pos::new(2, 2)]
        );
    }

    #[test]
    fn test_cube_face_side_vector() {
        assert_eq!(
            cube_face_side_vector(Pos::new(0, 0), 2, Direction::Down).collect::<Vec<Pos>>(),
            vec![Pos::new(1, 3), Pos::new(2, 3)]
        );
        assert_eq!(
            cube_face_side_vector(Pos::new(0, 0), 2, Direction::Up).collect::<Vec<Pos>>(),
            vec![Pos::new(1, 0), Pos::new(2, 0)]
        );
        assert_eq!(
            cube_face_side_vector(Pos::new(0, 0), 2, Direction::Left).collect::<Vec<Pos>>(),
            vec![Pos::new(0, 1), Pos::new(0, 2)]
        );
        assert_eq!(
            cube_face_side_vector(Pos::new(0, 0), 2, Direction::Right).collect::<Vec<Pos>>(),
            vec![Pos::new(3, 1), Pos::new(3, 2)]
        );
    }
}
