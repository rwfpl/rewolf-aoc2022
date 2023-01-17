extern crate derive_more;
use derive_more::Constructor;
use std::cmp;
use std::fs;

#[derive(Debug, Constructor, Clone, Copy)]
struct Shape<const X: usize, const Y: usize> {
    p: [[bool; Y]; X],
}

#[derive(Debug, Clone, Copy)]
enum Shapes {
    S14(Shape<1, 4>),
    S33(Shape<3, 3>),
    S41(Shape<4, 1>),
    S22(Shape<2, 2>),
}

impl Shapes {
    fn height(&self) -> usize {
        match self {
            Shapes::S14(_) => 1,
            Shapes::S22(_) => 2,
            Shapes::S33(_) => 3,
            Shapes::S41(_) => 4,
        }
    }

    fn get_row(&self, n: usize) -> &[bool] {
        match self {
            Shapes::S14(s) => &s.p[0 - n],
            Shapes::S22(s) => &s.p[1 - n],
            Shapes::S33(s) => &s.p[2 - n],
            Shapes::S41(s) => &s.p[3 - n],
        }
    }

    #[allow(dead_code)]
    fn print_shape(&self) {
        for i in (0..self.height()).rev() {
            let row = self.get_row(i);
            for c in row {
                print!("{}", if *c { '@' } else { ' ' });
            }
            println!();
        }
    }
}

static SHAPES: [Shapes; 5] = [
    Shapes::S14(Shape {
        p: [[true, true, true, true]],
    }),
    Shapes::S33(Shape {
        p: [
            [false, true, false],
            [true, true, true],
            [false, true, false],
        ],
    }),
    Shapes::S33(Shape {
        p: [
            [false, false, true],
            [false, false, true],
            [true, true, true],
        ],
    }),
    Shapes::S41(Shape {
        p: [[true], [true], [true], [true]],
    }),
    Shapes::S22(Shape {
        p: [[true, true], [true, true]],
    }),
];

#[derive(Debug, Clone, Copy)]
enum Move {
    Left,
    Right,
}

#[derive(Debug, Constructor, Clone, Copy)]
struct Pos {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct TheGame {
    grid: Vec<Vec<bool>>,
    moves: Vec<Move>,
    current_move: usize,
    grid_width: usize,
}

impl TheGame {
    fn new(moves: &[Move], grid_width: usize) -> Self {
        Self {
            grid: Vec::new(),
            moves: moves.to_owned(),
            current_move: 0,
            grid_width,
        }
    }

    #[allow(dead_code)]
    fn print_grid(&self) {
        for r in self.grid.iter().rev() {
            println!(
                "|{}|",
                r.iter()
                    .map(|c| if *c { '@' } else { ' ' })
                    .collect::<String>()
            );
        }
        println!("{}", "-".repeat(self.grid_width + 2));
    }

    fn can_move_to(&self, x: usize, y: usize) -> bool {
        if y >= self.grid.len() {
            // We are above the grid, all moves are ok.
            true
        } else {
            // Check the actual position in the grid.
            !self.grid[y][x]
        }
    }

    fn can_move_row(&self, row: &[bool], x: usize, y: usize, m: &Move) -> bool {
        for (i, c) in row.iter().enumerate() {
            if !*c {
                continue;
            }
            match m {
                Move::Left => {
                    if x == 0 || !self.can_move_to(x - 1 + i, y) {
                        return false;
                    }
                }
                Move::Right => {
                    if x + row.len() >= self.grid_width || !self.can_move_to(x + i + 1, y) {
                        return false;
                    }
                }
            }
        }
        true
    }

    fn apply_jet(&self, s: &Shapes, cp: &Pos, m: &Move) -> Option<Pos> {
        for i in 0..s.height() {
            if !self.can_move_row(s.get_row(i), cp.x, cp.y + i, m) {
                return None;
            }
        }
        match m {
            Move::Left => Some(Pos::new(cp.x - 1, cp.y)),
            Move::Right => Some(Pos::new(cp.x + 1, cp.y)),
        }
    }

    fn fall(&self, s: &Shapes, cp: &Pos) -> Option<Pos> {
        if cp.y == 0 {
            None
        } else if cp.y > self.grid.len() {
            Some(Pos::new(cp.x, cp.y - 1))
        } else {
            for j in 0..s.height() {
                if cp.y - 1 + j >= self.grid.len() {
                    continue;
                }
                for (i, c) in s.get_row(j).iter().enumerate() {
                    if *c && self.grid[cp.y - 1 + j][cp.x + i] {
                        return None;
                    }
                }
            }
            Some(Pos::new(cp.x, cp.y - 1))
        }
    }

    fn add_shape_to_the_grid(&mut self, s: &Shapes, p: &Pos) {
        let delta = s.height() as i32 + p.y as i32 - self.grid.len() as i32;
        if delta > 0 {
            (0..delta).for_each(|_| self.grid.push(vec![false; self.grid_width]));
        }
        for i in 0..s.height() {
            for (j, c) in s.get_row(i).iter().enumerate() {
                if *c {
                    self.grid[p.y + i][p.x + j] = true;
                }
            }
        }
    }

    fn play_shape(&mut self, s: &Shapes) {
        let mut cp: Pos = Pos::new(2, self.grid.len() + 3);
        loop {
            cp = self
                .apply_jet(s, &cp, &self.moves[self.current_move % self.moves.len()])
                .unwrap_or(cp);
            self.current_move += 1;
            if let Some(p) = self.fall(s, &cp) {
                cp = p;
            } else {
                self.add_shape_to_the_grid(s, &cp);
                //s.print_shape();
                //self.print_grid();
                break;
            }
        }
    }
}

fn solution(filename: &str, interval: usize, n: i64) -> usize {
    let moves = fs::read_to_string(filename)
        .unwrap()
        .trim()
        .chars()
        .map(|m| if m == '<' { Move::Left } else { Move::Right })
        .collect::<Vec<Move>>();

    let mut the_game = TheGame::new(&moves, 7);

    let mut prev_h = 0;
    let iterations = moves.len() * SHAPES.len() * interval;
    let mut next_heights = 0;
    for i in 0..cmp::min(n as usize, 2 * iterations + 1) {
        if i != 0 && i % iterations == 0 {
            next_heights = the_game.grid.len() - prev_h;
            prev_h = the_game.grid.len();
        }
        the_game.play_shape(&SHAPES[i % SHAPES.len()]);
    }
    let iterations_left = n - (2 * iterations + 1) as i64;
    if iterations_left > 0 {
        for i in 0..iterations_left as usize % iterations {
            the_game.play_shape(&SHAPES[(i + 2 * iterations + 1) % SHAPES.len()]);
        }
        the_game.grid.len() + next_heights * (iterations_left as usize / iterations)
    } else {
        the_game.grid.len()
    }
}

pub fn run() -> (String, String) {
    (
        solution("src/inputs/aoc_17.input", 341, 2022).to_string(),
        solution("src/inputs/aoc_17.input", 341, 1_000_000_000_000).to_string(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        assert_eq!(solution("src/inputs/aoc_17_sample.input", 7, 2022), 3068);
        assert_eq!(
            solution("src/inputs/aoc_17_sample.input", 7, 1_000_000_000_000),
            1_514_285_714_288
        );
        assert_eq!(solution("src/inputs/aoc_17.input", 341, 2022), 3153);
        assert_eq!(
            solution("src/inputs/aoc_17.input", 341, 1_000_000_000_000),
            1_553_665_689_155
        );
    }
}
