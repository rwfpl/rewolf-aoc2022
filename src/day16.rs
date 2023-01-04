extern crate derive_more;
extern crate lazy_static;
extern crate smallvec;

use derive_more::Constructor;
use regex::Regex;
use smallvec::SmallVec;
use std::collections::{HashMap, VecDeque};
use std::fs;
use std::sync::Mutex;

#[derive(Debug, Constructor, PartialEq, Eq, Hash)]
struct Valve {
    name: i32,
    flow_rate: i32,
    leads_to: SmallVec<[i32; 10]>,
}

fn match_to_i32(m: &Option<regex::Match>) -> i32 {
    m.unwrap().as_str().parse::<i32>().unwrap()
}

fn node_name_to_i32(name: &str) -> i32 {
    *name.as_bytes().first().unwrap() as i32 | (*name.as_bytes().get(1).unwrap() as i32) << 8
}

#[allow(dead_code)]
fn i32_to_node_name(i: i32) -> String {
    [
        char::from_u32((i & 0xFF) as u32).unwrap(),
        char::from_u32(((i >> 8) & 0xFF) as u32).unwrap(),
    ]
    .iter()
    .collect()
}

impl From<regex::Captures<'_>> for Valve {
    fn from(capture: regex::Captures) -> Self {
        Valve {
            name: node_name_to_i32(capture.name("valve").unwrap().as_str()),
            flow_rate: match_to_i32(&capture.name("rate")),
            leads_to: capture
                .name("dest")
                .unwrap()
                .as_str()
                .split(',')
                .map(|d| node_name_to_i32(d.trim()))
                .collect(),
        }
    }
}

#[derive(Debug, Copy, Clone, Constructor)]
struct Distance {
    to: i32,
    len: usize,
}

lazy_static::lazy_static! {
    static ref BFSCACHE: Mutex<HashMap<(i32, i32), Option<Distance>>> = Mutex::new(HashMap::new());
}

fn bfs(from: i32, to: i32, valves: &HashMap<i32, Valve>) -> Option<Distance> {
    if let Some(r) = BFSCACHE.lock().unwrap().get(&(from, to)) {
        return *r;
    }
    let mut q: VecDeque<(i32, usize)> = VecDeque::new();
    q.push_back((from, 1));
    loop {
        if q.is_empty() {
            BFSCACHE.lock().unwrap().insert((from, to), None);
            return None;
        }
        let p = q.pop_front().unwrap();
        if p.0 == to {
            BFSCACHE
                .lock()
                .unwrap()
                .insert((from, to), Some(Distance::new(p.0, p.1)));
            return Some(Distance::new(p.0, p.1));
        }
        valves
            .get(&p.0)
            .unwrap()
            .leads_to
            .iter()
            .for_each(|lead| q.push_back((*lead, p.1 + 1)));
    }
}

#[derive(Debug, Constructor, PartialEq, Eq)]
struct PathPotential {
    last_node: i32,
    score: usize,
    len: usize,
}

fn evaluate_paths(
    paths: impl Iterator<Item = Distance>,
    valves: &HashMap<i32, Valve>,
    minutes_left: usize,
) -> SmallVec<[PathPotential; 32]> {
    let max_path = minutes_left;
    paths
        .map(|p| {
            let score = if max_path > p.len {
                (max_path - p.len) * valves.get(&p.to).unwrap().flow_rate as usize
            } else {
                0
            };
            PathPotential::new(p.to, score, p.len)
        })
        .filter(|pp| pp.score > 0)
        .collect::<SmallVec<[PathPotential; 32]>>()
}

fn paths_for_node(valves: &HashMap<i32, Valve>, node: i32) -> impl Iterator<Item = Distance> + '_ {
    valves
        .iter()
        .filter(|(_, valve)| valve.flow_rate > 0)
        .map(move |(_, valve)| bfs(node, valve.name, valves).unwrap())
}

fn search_path(
    valves: &mut HashMap<i32, Valve>,
    cur_node: i32,
    minutes_left: usize,
    sum: usize,
) -> usize {
    let flow_rate_left: i32 = valves.iter().map(|(_, valve)| valve.flow_rate).sum();
    if flow_rate_left == 0 {
        return sum;
    }
    let potential = evaluate_paths(paths_for_node(valves, cur_node), valves, minutes_left);
    potential
        .iter()
        .map(|p| {
            let flow_rate_backup = valves.get(&p.last_node).unwrap().flow_rate;
            valves.get_mut(&p.last_node).unwrap().flow_rate = 0;
            let r = search_path(valves, p.last_node, minutes_left - p.len, sum + p.score);
            valves.get_mut(&p.last_node).unwrap().flow_rate = flow_rate_backup;
            r
        })
        .max()
        .unwrap_or(sum)
}

fn search_path_p2(
    valves: &mut HashMap<i32, Valve>,
    you: i32,
    elephant: i32,
    minutes_left_you: usize,
    minutes_left_elephant: usize,
    sum_you: usize,
    sum_elephant: usize,
) -> (usize, usize) {
    let flow_rate_left: i32 = valves.iter().map(|(_, valve)| valve.flow_rate).sum();
    if flow_rate_left == 0 {
        return (sum_you, sum_elephant);
    }

    let potential_you = evaluate_paths(paths_for_node(valves, you), valves, minutes_left_you);
    let potential_elephant = evaluate_paths(
        paths_for_node(valves, elephant),
        valves,
        minutes_left_elephant,
    );

    let mut sum_max = (sum_you, sum_elephant);
    if potential_you.is_empty() && potential_elephant.is_empty() {
        //
    } else if potential_you.is_empty() {
        //only elephants moves
        potential_elephant.iter().for_each(|pp_elephant| {
            let flow_rate_backup_elephant = valves.get(&pp_elephant.last_node).unwrap().flow_rate;
            valves.get_mut(&pp_elephant.last_node).unwrap().flow_rate = 0;

            let r = search_path_p2(
                valves,
                you,
                pp_elephant.last_node,
                minutes_left_you,
                minutes_left_elephant - pp_elephant.len,
                sum_you,
                sum_elephant + pp_elephant.score,
            );
            if r.0 + r.1 > sum_max.0 + sum_max.1 {
                sum_max = r;
            }

            valves.get_mut(&pp_elephant.last_node).unwrap().flow_rate = flow_rate_backup_elephant;
        });
    } else if potential_elephant.is_empty() {
        //only you moves
        potential_you.iter().for_each(|pp_you| {
            let flow_rate_backup_you = valves.get(&pp_you.last_node).unwrap().flow_rate;
            valves.get_mut(&pp_you.last_node).unwrap().flow_rate = 0;

            let r = search_path_p2(
                valves,
                pp_you.last_node,
                elephant,
                minutes_left_you - pp_you.len,
                minutes_left_elephant,
                sum_you + pp_you.score,
                sum_elephant,
            );
            if r.0 + r.1 > sum_max.0 + sum_max.1 {
                sum_max = r;
            }

            valves.get_mut(&pp_you.last_node).unwrap().flow_rate = flow_rate_backup_you;
        });
    } else {
        for pp_you in &potential_you {
            for pp_elephant in &potential_elephant {
                if pp_elephant == pp_you {
                    //println!("SAME PATH");
                    continue;
                }
                if pp_elephant.last_node == pp_you.last_node {
                    //println!("SAME DEST");
                    continue;
                }
                let flow_rate_backup_you = valves.get(&pp_you.last_node).unwrap().flow_rate;
                valves.get_mut(&pp_you.last_node).unwrap().flow_rate = 0;

                let flow_rate_backup_elephant =
                    valves.get(&pp_elephant.last_node).unwrap().flow_rate;
                valves.get_mut(&pp_elephant.last_node).unwrap().flow_rate = 0;

                let r = search_path_p2(
                    valves,
                    pp_you.last_node,
                    pp_elephant.last_node,
                    minutes_left_you - pp_you.len,
                    minutes_left_elephant - pp_elephant.len,
                    sum_you + pp_you.score,
                    sum_elephant + pp_elephant.score,
                );
                if r.0 + r.1 > sum_max.0 + sum_max.1 {
                    sum_max = r;
                }

                valves.get_mut(&pp_you.last_node).unwrap().flow_rate = flow_rate_backup_you;
                valves.get_mut(&pp_elephant.last_node).unwrap().flow_rate =
                    flow_rate_backup_elephant;
            }
        }
    }
    sum_max
}

fn solution(filename: &str) -> (usize, usize) {
    let rex =
        Regex::new(r"Valve\s(?P<valve>[A-Z]*)\shas\sflow\srate=(?P<rate>\d*);\stunnels?\sleads?\sto\svalves?\s(?P<dest>([A-Z]*,?\s?)+)").unwrap();
    let mut valves: HashMap<i32, Valve> = fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(|l| Valve::from(rex.captures(l).unwrap()))
        .flat_map(|v| vec![v])
        .map(|v| (v.name, v))
        .collect();

    let p1 = search_path(&mut valves, node_name_to_i32("AA"), 30, 0);
    println!("day16 p1: {p1}");

    let p2 = search_path_p2(
        &mut valves,
        node_name_to_i32("AA"),
        node_name_to_i32("AA"),
        26,
        26,
        0,
        0,
    );
    println!("day16 p2: {}", p2.0 + p2.1);
    (p1, p2.0 + p2.1)
}

pub fn run() {
    solution("src/inputs/aoc_16.input");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        assert_eq!(solution("src/inputs/aoc_16_sample.input"), (1651, 1707));
        assert_eq!(solution("src/inputs/aoc_16.input"), (1873, 2425));
    }
}
