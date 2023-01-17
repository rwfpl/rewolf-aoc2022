extern crate derive_more;

use derive_more::Constructor;
use rayon::prelude::*;
use regex::Regex;
use std::fs;

#[derive(Debug, Constructor)]
struct Robot {
    ore_cost: i32,
    clay_cost: i32,
    obsidian_cost: i32,
}

#[derive(Debug)]
struct Blueprint {
    id: i32,
    ore_robot: Robot,
    clay_robot: Robot,
    obsidian_robot: Robot,
    geode_robot: Robot,
}

fn match_to_i32(m: &Option<regex::Match>) -> i32 {
    m.unwrap().as_str().parse::<i32>().unwrap()
}

impl From<regex::Captures<'_>> for Blueprint {
    fn from(capture: regex::Captures) -> Self {
        Blueprint {
            id: match_to_i32(&capture.name("id")),
            ore_robot: Robot::new(match_to_i32(&capture.name("ore_robot_cost_ore")), 0, 0),
            clay_robot: Robot::new(match_to_i32(&capture.name("clay_robot_cost_ore")), 0, 0),
            obsidian_robot: Robot::new(
                match_to_i32(&capture.name("obsidian_robot_cost_ore")),
                match_to_i32(&capture.name("obsidian_robot_cost_clay")),
                0,
            ),
            geode_robot: Robot::new(
                match_to_i32(&capture.name("geode_robot_cost_ore")),
                0,
                match_to_i32(&capture.name("geode_robot_cost_obsidian")),
            ),
        }
    }
}

#[derive(Debug)]
struct Mine<'a> {
    bp: &'a Blueprint,
    resources: Resources,
    ore_robots: i32,
    clay_robots: i32,
    obsidian_robots: i32,
    geode_robots: i32,
}

#[derive(Debug, Clone, Copy)]
struct Resources {
    ore: i32,
    clay: i32,
    obsidian: i32,
    geode: i32,
}

impl Resources {
    fn new() -> Self {
        Self {
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0,
        }
    }
}

impl<'a> Mine<'a> {
    fn new(bp: &'a Blueprint) -> Self {
        Self {
            bp,
            resources: Resources::new(),
            ore_robots: 1,
            clay_robots: 0,
            obsidian_robots: 0,
            geode_robots: 0,
        }
    }

    fn can_build_ore_robot(&self) -> bool {
        self.bp.ore_robot.ore_cost <= self.resources.ore
    }

    fn build_ore_robot(&mut self) {
        self.resources.ore -= self.bp.ore_robot.ore_cost;
        self.ore_robots += 1;
    }

    fn can_build_clay_robot(&self) -> bool {
        self.bp.clay_robot.ore_cost <= self.resources.ore
    }

    fn build_clay_robot(&mut self) {
        self.resources.ore -= self.bp.clay_robot.ore_cost;
        self.clay_robots += 1;
    }

    fn can_build_obsidian_robot(&self) -> bool {
        let robot = &self.bp.obsidian_robot;
        robot.ore_cost <= self.resources.ore && robot.clay_cost <= self.resources.clay
    }

    fn build_obsidian_robot(&mut self) {
        let robot = &self.bp.obsidian_robot;
        self.resources.ore -= robot.ore_cost;
        self.resources.clay -= robot.clay_cost;
        self.obsidian_robots += 1;
    }

    fn can_build_geode_robot(&self) -> bool {
        let robot = &self.bp.geode_robot;
        robot.ore_cost <= self.resources.ore && robot.obsidian_cost <= self.resources.obsidian
    }

    fn build_geode_robot(&mut self) {
        let robot = &self.bp.geode_robot;
        self.resources.ore -= robot.ore_cost;
        self.resources.obsidian -= robot.obsidian_cost;
        self.geode_robots += 1;
    }

    fn run(&mut self, minutes: i32) -> i32 {
        if minutes == 0 {
            return self.resources.geode;
        }
        // figure out which robot to build
        let new_geode = self.can_build_geode_robot();
        let new_obsidian = self.can_build_obsidian_robot();
        let new_clay = self.can_build_clay_robot();
        let new_ore = self.can_build_ore_robot();

        // collect resources
        self.resources.ore += self.ore_robots;
        self.resources.clay += self.clay_robots;
        self.resources.obsidian += self.obsidian_robots;
        self.resources.geode += self.geode_robots;

        // finish building robots
        let mut max: i32 = self.resources.geode;
        let rb = self.resources;
        if new_geode {
            // we always want to build geode robot to maximize geodes
            self.build_geode_robot();
            let m = self.run(minutes - 1);
            if m > max {
                max = m;
            }
            self.geode_robots -= 1;
            self.resources = rb;
        } else {
            if new_obsidian {
                self.build_obsidian_robot();
                let m = self.run(minutes - 1);
                if m > max {
                    max = m;
                }
                self.obsidian_robots -= 1;
                self.resources = rb;
            }
            if new_clay && self.resources.clay < 24 {
                self.build_clay_robot();
                let m = self.run(minutes - 1);
                if m > max {
                    max = m;
                }
                self.clay_robots -= 1;
                self.resources = rb;
            }
            if new_ore && self.resources.ore < 9 {
                self.build_ore_robot();
                let m = self.run(minutes - 1);
                if m > max {
                    max = m;
                }
                self.ore_robots -= 1;
                self.resources = rb;
            }
            let m = self.run(minutes - 1);
            if m > max {
                max = m;
            }
            self.resources = rb;
        }
        max
    }
}

fn load_blueprints(filename: &str) -> Vec<Blueprint> {
    let bprex = Regex::new(r"Blueprint\s(?P<id>\d+):\sEach\sore\srobot\scosts\s(?P<ore_robot_cost_ore>\d+)\sore\.\sEach\sclay\srobot\scosts\s(?P<clay_robot_cost_ore>\d+)\sore\.\sEach\sobsidian\srobot\scosts\s(?P<obsidian_robot_cost_ore>\d+)\sore\sand\s(?P<obsidian_robot_cost_clay>\d+)\sclay\.\sEach\sgeode\srobot\scosts\s(?P<geode_robot_cost_ore>\d+)\sore\sand\s(?P<geode_robot_cost_obsidian>\d+)\sobsidian\.").unwrap();
    fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(|l| Blueprint::from(bprex.captures(l).unwrap()))
        .collect()
}

fn solution_p1(filename: &str) -> i32 {
    load_blueprints(filename)
        .into_par_iter()
        .map(|bp| Mine::new(&bp).run(24) * bp.id)
        .sum::<i32>()
}

fn solution_p2(filename: &str) -> i32 {
    load_blueprints(filename)
        .into_iter()
        .take(3)
        .collect::<Vec<Blueprint>>()
        .into_par_iter()
        .map(|bp| Mine::new(&bp).run(32))
        .product::<i32>()
}

pub fn run() -> (String, String) {
    (
        solution_p1("src/inputs/aoc_19.input").to_string(),
        solution_p2("src/inputs/aoc_19.input").to_string(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        let blueprints = load_blueprints("src/inputs/aoc_19_sample.input");
        assert_eq!(blueprints.len(), 2);
        assert_eq!(Mine::new(&blueprints[0]).run(24), 9);
        assert_eq!(Mine::new(&blueprints[1]).run(24), 12);
        assert_eq!(Mine::new(&blueprints[0]).run(32), 56);
        assert_eq!(Mine::new(&blueprints[1]).run(32), 62);
        assert_eq!(solution_p1("src/inputs/aoc_19_sample.input"), 33);
        assert_eq!(solution_p1("src/inputs/aoc_19.input"), 1199);
        assert_eq!(solution_p2("src/inputs/aoc_19.input"), 3510);
    }
}
