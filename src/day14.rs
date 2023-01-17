extern crate derive_more;
use derive_more::Constructor;
use itertools::Itertools;
use std::collections::HashSet;
use std::fmt;
use std::fs;

#[derive(Debug, PartialEq, Eq, Hash, Constructor)]
struct Point {
    x: usize,
    y: usize,
}

impl From<(usize, usize)> for Point {
    fn from((x, y): (usize, usize)) -> Self {
        Self { x, y }
    }
}

impl From<&str> for Point {
    fn from(s: &str) -> Self {
        Self::from(
            s.split(',')
                .map(|v| v.parse::<usize>().unwrap())
                .tuples::<(usize, usize)>()
                .next()
                .unwrap(),
        )
    }
}

fn get_points_from_tuple_vector((s, e): (&Point, &Point)) -> Vec<Point> {
    let x_d = e.x as i32 - s.x as i32;
    let y_d = e.y as i32 - s.y as i32;
    if x_d != 0 {
        (0..=x_d.abs())
            .map(|i| Point::new((s.x as i32 + i * x_d.signum()) as usize, s.y))
            .collect()
    } else if y_d != 0 {
        (0..=y_d.abs())
            .map(|i| Point::new(s.x, (s.y as i32 + i * y_d.signum()) as usize))
            .collect()
    } else {
        panic!("empty vector");
    }
}

#[derive(Debug)]
struct Path {
    points: Vec<Point>,
}

impl From<&str> for Path {
    fn from(s: &str) -> Self {
        Self {
            points: s.split("->").map(|s| Point::from(s.trim())).collect(),
        }
    }
}

impl Path {
    fn max_x(&self) -> usize {
        self.points.iter().map(|p| p.x).max().unwrap()
    }

    fn max_y(&self) -> usize {
        self.points.iter().map(|p| p.y).max().unwrap()
    }

    fn get_all_points(&self) -> impl Iterator<Item = Point> + '_ {
        self.points
            .iter()
            .zip(self.points[1..].iter())
            .flat_map(get_points_from_tuple_vector)
    }
}

#[derive(Debug)]
struct Paths {
    paths: Vec<Path>,
}

impl From<&str> for Paths {
    fn from(s: &str) -> Self {
        Self {
            paths: s.lines().map(Path::from).collect(),
        }
    }
}

impl Paths {
    fn max_x(&self) -> usize {
        self.paths.iter().map(|p| p.max_x()).max().unwrap()
    }

    fn max_y(&self) -> usize {
        self.paths.iter().map(|p| p.max_y()).max().unwrap()
    }
}

struct Map {
    grid: Vec<Vec<char>>,
    sand_source: Point,
    abyss: bool,
    width: usize,
    height: usize,
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.grid
                .iter()
                .map(|row| row.iter().collect::<String>() + "\n")
                .collect::<String>()
        )
    }
}

impl Map {
    fn from_paths(paths: &Paths, sand_source: &Point, abyss: bool) -> Map {
        let width = if abyss {
            paths.max_x() + 1
        } else {
            2 * (paths.max_x() + 1)
        };
        let height = if abyss {
            paths.max_y() + 1
        } else {
            paths.max_y() + 2
        };

        let points: HashSet<Point> =
            HashSet::from_iter(paths.paths.iter().flat_map(|p| p.get_all_points()));

        Map {
            grid: (0..height)
                .into_iter()
                .map(|y| {
                    (0..width)
                        .into_iter()
                        .map(|x| {
                            if points.contains(&Point::new(x, y)) {
                                'X'
                            } else {
                                '.'
                            }
                        })
                        .collect::<Vec<char>>()
                })
                .collect(),
            sand_source: Point::new(sand_source.x, sand_source.y),
            abyss,
            width,
            height,
        }
    }

    fn is_blocked(&self, x: usize, y: usize) -> bool {
        self.is_wall(x) || self.is_floor(y) || self.grid[y][x] == 'X' || self.grid[y][x] == 'o'
    }

    fn is_floor(&self, y: usize) -> bool {
        y >= self.height
    }

    fn is_wall(&self, x: usize) -> bool {
        x >= self.width
    }

    fn add_sand(&mut self) -> bool {
        let mut cur = Point::new(self.sand_source.x, self.sand_source.y);
        loop {
            if self.abyss && (self.is_floor(cur.y + 1) || self.is_wall(cur.x + 1)) {
                return false;
            }
            if !self.is_blocked(cur.x, cur.y + 1) {
                // down
                cur.y += 1;
            } else if !self.is_blocked(cur.x - 1, cur.y + 1) {
                // left
                cur.y += 1;
                cur.x -= 1;
            } else if !self.is_blocked(cur.x + 1, cur.y + 1) {
                // right
                cur.y += 1;
                cur.x += 1;
            } else {
                self.grid[cur.y][cur.x] = 'o';
                return !(cur == self.sand_source);
            }
        }
    }
}

fn solution(filename: &str, abyss: bool) -> i32 {
    let mut map = Map::from_paths(
        &Paths::from(fs::read_to_string(filename).unwrap().as_str()),
        &Point::new(500, 0),
        abyss,
    );

    let mut i = 0;
    while map.add_sand() {
        i += 1;
    }

    if !abyss {
        i += 1;
    }
    i
}

pub fn run() -> (String, String) {
    (
        solution("src/inputs/aoc_14.input", true).to_string(),
        solution("src/inputs/aoc_14.input", false).to_string(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        assert_eq!(solution("src/inputs/aoc_14_sample.input", true), 24);
        assert_eq!(solution("src/inputs/aoc_14_sample.input", false), 93);
        assert_eq!(solution("src/inputs/aoc_14.input", true), 873);
        assert_eq!(solution("src/inputs/aoc_14.input", false), 24813);
    }
}
