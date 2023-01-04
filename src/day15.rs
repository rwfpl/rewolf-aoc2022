extern crate derive_more;

use derive_more::Constructor;
use itertools::sorted;
use regex::Regex;
use std::collections::HashSet;
use std::fs;

#[derive(Debug, PartialEq, Eq, Hash, Constructor, Clone)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct SensorBeacon {
    sensor: Point,
    beacon: Point,
}

fn match_to_i32(m: &Option<regex::Match>) -> i32 {
    m.unwrap().as_str().parse::<i32>().unwrap()
}

impl From<&regex::Captures<'_>> for SensorBeacon {
    fn from(capture: &regex::Captures) -> Self {
        SensorBeacon {
            sensor: Point::new(
                match_to_i32(&capture.name("sx")),
                match_to_i32(&capture.name("sy")),
            ),
            beacon: Point::new(
                match_to_i32(&capture.name("bx")),
                match_to_i32(&capture.name("by")),
            ),
        }
    }
}

impl SensorBeacon {
    fn get_distance(&self) -> i32 {
        (self.sensor.x - self.beacon.x).abs() + (self.sensor.y - self.beacon.y).abs()
    }

    fn get_coverage(&self, flt: i32) -> HashSet<Point> {
        let distance = self.get_distance();
        if (self.sensor.y - distance..=self.sensor.y + distance).contains(&flt) {
            let flt_distance = flt - self.sensor.y;
            let delta = distance - flt_distance.abs();
            (0..2 * delta + 1)
                .map(|i| Point::new(self.sensor.x - delta + i, flt))
                .collect::<HashSet<Point>>()
        } else {
            HashSet::new()
        }
    }

    fn get_range_for_row(&self, flt: i32) -> (i32, i32) {
        let distance = self.get_distance();
        if (self.sensor.y - distance..=self.sensor.y + distance).contains(&flt) {
            let flt_distance = flt - self.sensor.y;
            let delta = distance - flt_distance.abs();
            (self.sensor.x - delta, self.sensor.x + delta)
        } else {
            (0, 0)
        }
    }
}

fn get_row_blind_spots(row: i32, sb: &[SensorBeacon]) -> HashSet<(i32, i32)> {
    let ranges = sb
        .iter()
        .map(|sb| sb.get_range_for_row(row))
        .filter(|r| r != &(0, 0));

    let mut blind: HashSet<(i32, i32)> = HashSet::new();
    let mut i = 0;
    sorted(ranges).for_each(|r| {
        if (r.0..=r.1).contains(&i) {
            i = r.1 + 1;
        } else if i < r.0 {
            blind.insert((i, r.0 - 1));
            i = r.0;
        }
    });
    blind
}

fn get_row_coverage(row: i32, sb: &[SensorBeacon], beacons: &HashSet<&Point>) -> usize {
    let coverage: HashSet<Point> = sb.iter().flat_map(|sb| sb.get_coverage(row)).collect();
    coverage.len() - beacons.iter().filter(|b| b.y == row).count()
}

fn solution(filename: &str, p1_row: i32) -> (usize, i64) {
    let rex =
        Regex::new(r"Sensor\sat\sx=(?P<sx>-?\d*),\sy=(?P<sy>-?\d*):\sclosest\sbeacon\sis\sat\sx=(?P<bx>-?\d*),\sy=(?P<by>-?\d*)").unwrap();
    let sb: Vec<SensorBeacon> = fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(|l| SensorBeacon::from(&rex.captures(l).unwrap()))
        .collect();
    let p1 = get_row_coverage(p1_row, &sb, &sb.iter().map(|sb| &sb.beacon).collect());

    for i in 0..4_000_000 {
        let bs = get_row_blind_spots(i, &sb);
        if !bs.is_empty() {
            return (
                p1,
                bs.iter().next().unwrap().0 as i64 * 4_000_000 + i as i64,
            );
        }
    }
    panic!("couldn't find p2 solution");
}

pub fn run() {
    let (p1, p2) = solution("src/inputs/aoc_15.input", 2_000_000);
    println!("day15 p1: {p1}");
    println!("day15 p1: {p2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        assert_eq!(
            solution("src/inputs/aoc_15_sample.input", 10),
            (26, 56000011)
        );
        assert_eq!(
            solution("src/inputs/aoc_15.input", 2_000_000),
            (5525990, 11756174628223)
        );
    }
}
