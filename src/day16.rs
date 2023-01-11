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
    name: usize,
    flow_rate: i32,
    leads_to: SmallVec<[usize; 10]>,
}

struct NodeIdMap {
    nodes: HashMap<String, usize>,
    next_id: usize,
}

impl NodeIdMap {
    fn new() -> Self {
        NodeIdMap {
            nodes: HashMap::new(),
            next_id: 0,
        }
    }

    fn insert_or_get(&mut self, node_name: &str) -> usize {
        if let Some(node_id) = self.nodes.get(node_name) {
            return *node_id;
        }
        self.nodes.insert(node_name.to_string(), self.next_id);
        self.next_id += 1;
        self.next_id - 1
    }
}

fn match_to_i32(m: &Option<regex::Match>) -> i32 {
    m.unwrap().as_str().parse::<i32>().unwrap()
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

lazy_static::lazy_static! {
    static ref NODE_ID_MAP: Mutex<NodeIdMap> = Mutex::new(NodeIdMap::new());
}

impl From<regex::Captures<'_>> for Valve {
    fn from(capture: regex::Captures) -> Self {
        let mut node_id_map = NODE_ID_MAP.lock().unwrap();
        Valve {
            name: node_id_map.insert_or_get(capture.name("valve").unwrap().as_str()),
            flow_rate: match_to_i32(&capture.name("rate")),
            leads_to: capture
                .name("dest")
                .unwrap()
                .as_str()
                .split(',')
                .map(|d| node_id_map.insert_or_get(d.trim()))
                .collect(),
        }
    }
}

#[derive(Debug, Copy, Clone, Constructor)]
struct Distance {
    to: usize,
    len: usize,
}

lazy_static::lazy_static! {
    static ref BFSCACHE: Mutex<HashMap<(usize, usize), Option<Distance>>> = Mutex::new(HashMap::new());
}

fn bfs(from: usize, to: usize, valves: &[Valve]) -> Option<Distance> {
    if let Some(r) = BFSCACHE.lock().unwrap().get(&(from, to)) {
        return *r;
    }
    let mut q: VecDeque<(usize, usize)> = VecDeque::new();
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
        valves[p.0]
            .leads_to
            .iter()
            .for_each(|lead| q.push_back((*lead, p.1 + 1)));
    }
}

#[derive(Debug, Constructor, PartialEq, Eq)]
struct PathPotential {
    last_node: usize,
    score: usize,
    len: usize,
}

fn evaluate_paths(
    paths: impl Iterator<Item = Distance>,
    valves: &[Valve],
    minutes_left: usize,
) -> SmallVec<[PathPotential; 32]> {
    let max_path = minutes_left;
    paths
        .filter(|p| max_path > p.len)
        .map(|p| {
            PathPotential::new(
                p.to,
                (max_path - p.len) * valves[p.to].flow_rate as usize,
                p.len,
            )
        })
        .filter(|pp| pp.score > 0)
        .collect::<SmallVec<[PathPotential; 32]>>()
}

fn paths_for_node(valves: &[Valve], node: usize) -> impl Iterator<Item = Distance> + '_ {
    valves
        .iter()
        .filter(|valve| valve.flow_rate > 0)
        .map(move |valve| bfs(node, valve.name, valves).unwrap())
}

fn set_flow_rate(valves: &mut [Valve], node_id: usize, new_flow_rate: i32) -> i32 {
    let ret = valves[node_id].flow_rate;
    valves[node_id].flow_rate = new_flow_rate;
    ret
}

fn search_path(valves: &mut Vec<Valve>, cur_node: usize, minutes_left: usize, sum: usize) -> usize {
    let flow_rate_left: i32 = valves.iter().map(|valve| valve.flow_rate).sum();
    if flow_rate_left == 0 {
        return sum;
    }
    let potential = evaluate_paths(paths_for_node(valves, cur_node), valves, minutes_left);
    potential
        .iter()
        .map(|p| {
            let flow_rate_backup = set_flow_rate(valves, p.last_node, 0);
            let r = search_path(valves, p.last_node, minutes_left - p.len, sum + p.score);
            valves[p.last_node].flow_rate = flow_rate_backup;
            r
        })
        .max()
        .unwrap_or(sum)
}

#[derive(Constructor)]
struct Visitor {
    node_id: usize,
    minutes_left: usize,
    sum: usize,
}

fn search_path_p2(
    valves: &mut Vec<Valve>,
    you: &Visitor,
    elephant: &Visitor,
    flow_rate_left: i32,
) -> (usize, usize) {
    if flow_rate_left == 0 {
        return (you.sum, elephant.sum);
    }

    let potential_you = evaluate_paths(
        paths_for_node(valves, you.node_id),
        valves,
        you.minutes_left,
    );
    let potential_elephant = evaluate_paths(
        paths_for_node(valves, elephant.node_id),
        valves,
        elephant.minutes_left,
    );

    let mut sum_max = (you.sum, elephant.sum);
    if potential_you.is_empty() && potential_elephant.is_empty() {
        //
    } else if potential_you.is_empty() {
        //only elephants moves
        potential_elephant.iter().for_each(|pp_elephant| {
            let flow_rate_backup_elephant = set_flow_rate(valves, pp_elephant.last_node, 0);
            let r = search_path_p2(
                valves,
                you,
                &Visitor::new(
                    pp_elephant.last_node,
                    elephant.minutes_left - pp_elephant.len,
                    elephant.sum + pp_elephant.score,
                ),
                flow_rate_left - flow_rate_backup_elephant,
            );
            if r.0 + r.1 > sum_max.0 + sum_max.1 {
                sum_max = r;
            }

            valves[pp_elephant.last_node].flow_rate = flow_rate_backup_elephant;
        });
    } else if potential_elephant.is_empty() {
        //only you moves
        potential_you.iter().for_each(|pp_you| {
            let flow_rate_backup_you = set_flow_rate(valves, pp_you.last_node, 0);
            let r = search_path_p2(
                valves,
                &Visitor::new(
                    pp_you.last_node,
                    you.minutes_left - pp_you.len,
                    you.sum + pp_you.score,
                ),
                elephant,
                flow_rate_left - flow_rate_backup_you,
            );
            if r.0 + r.1 > sum_max.0 + sum_max.1 {
                sum_max = r;
            }

            valves[pp_you.last_node].flow_rate = flow_rate_backup_you;
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
                let flow_rate_backup_you = set_flow_rate(valves, pp_you.last_node, 0);
                let flow_rate_backup_elephant = set_flow_rate(valves, pp_elephant.last_node, 0);
                let r = search_path_p2(
                    valves,
                    &Visitor::new(
                        pp_you.last_node,
                        you.minutes_left - pp_you.len,
                        you.sum + pp_you.score,
                    ),
                    &Visitor::new(
                        pp_elephant.last_node,
                        elephant.minutes_left - pp_elephant.len,
                        elephant.sum + pp_elephant.score,
                    ),
                    flow_rate_left - flow_rate_backup_elephant - flow_rate_backup_you,
                );
                if r.0 + r.1 > sum_max.0 + sum_max.1 {
                    sum_max = r;
                }

                valves[pp_you.last_node].flow_rate = flow_rate_backup_you;
                valves[pp_elephant.last_node].flow_rate = flow_rate_backup_elephant;
            }
        }
    }
    sum_max
}

fn solution(filename: &str) -> (usize, usize) {
    NODE_ID_MAP.lock().unwrap().nodes.clear();
    NODE_ID_MAP.lock().unwrap().next_id = 0;
    BFSCACHE.lock().unwrap().clear();
    let rex =
        Regex::new(r"Valve\s(?P<valve>[A-Z]*)\shas\sflow\srate=(?P<rate>\d*);\stunnels?\sleads?\sto\svalves?\s(?P<dest>([A-Z]*,?\s?)+)").unwrap();
    let mut valves: Vec<Valve> = fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(|l| Valve::from(rex.captures(l).unwrap()))
        .flat_map(|v| vec![v])
        .collect();
    valves.sort_by_key(|v| v.name);

    let p1 = search_path(
        &mut valves,
        NODE_ID_MAP.lock().unwrap().insert_or_get("AA"),
        30,
        0,
    );
    println!("day16 p1: {p1}");

    let mut node_id_map = NODE_ID_MAP.lock().unwrap();
    let flow_rate_left = valves.iter().map(|valve| valve.flow_rate).sum();
    let p2 = search_path_p2(
        &mut valves,
        &Visitor::new(node_id_map.insert_or_get("AA"), 26, 0),
        &Visitor::new(node_id_map.insert_or_get("AA"), 26, 0),
        flow_rate_left,
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
    fn test_run_sample() {
        assert_eq!(solution("src/inputs/aoc_16_sample.input"), (1651, 1707));
    }

    #[test]
    fn test_run() {
        assert_eq!(solution("src/inputs/aoc_16.input"), (1873, 2425));
    }
}
