use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use parse_display::{Display, FromStr};
use std::collections::{HashMap, HashSet};

type TargetsWithCost = Vec<(String, usize)>;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct Valve {
    id: String,
    flow_rate: usize,
    targets: Vec<String>,
}

#[derive(Display, FromStr)]
#[display("Valve {id} has flow rate={flow_rate}")]
pub struct ParsedData {
    id: String,
    flow_rate: usize,
}

#[aoc_generator(day16)]
pub fn generate(inp: &str) -> Option<Vec<Valve>> {
    let mut result = vec![];

    for line in inp.lines() {
        let (lhs, rhs) = line.split_once("; ")?;
        let parsed_data = lhs.parse::<ParsedData>().ok()?;

        let mut tunnel_targets = vec![];
        let rhs = if let Some(multiple) = rhs.strip_prefix("tunnels lead to valves ") {
            multiple
        } else {
            rhs.strip_prefix("tunnel leads to valve ")?
        };

        for tunnel in rhs.split(", ") {
            tunnel_targets.push(tunnel.to_string());
        }

        result.push(Valve {
            id: parsed_data.id,
            flow_rate: parsed_data.flow_rate,
            targets: tunnel_targets,
        });
    }

    Some(result)
}

fn find_best_path(
    opened: &HashSet<&str>,
    cur_node: &str,
    cur_pressure: usize,
    rem_time: usize,
    valves: &HashMap<String, (usize, TargetsWithCost)>,
    elephant: Option<bool>,
    max_pressure: &mut usize,
) {
    *max_pressure = (*max_pressure).max(cur_pressure);

    if rem_time == 0 {
        return;
    }

    let (flow, targets) = valves.get(cur_node).unwrap();
    if opened.contains(cur_node) {
        for (target, cost) in targets {
            if opened.contains(target.as_str()) || *cost > rem_time {
                continue;
            }

            find_best_path(
                opened,
                target,
                cur_pressure,
                rem_time - *cost,
                valves,
                elephant,
                max_pressure,
            );
        }
    } else {
        let mut opened = opened.clone();
        opened.insert(cur_node);

        let pressure = cur_pressure + *flow * rem_time;
        let opened = opened.union(&HashSet::from([cur_node])).copied().collect();
        find_best_path(
            &opened,
            cur_node,
            pressure,
            rem_time - 1,
            valves,
            elephant,
            max_pressure,
        );

        if elephant == Some(false) {
            find_best_path(
                &opened,
                "AA",
                pressure,
                25,
                valves,
                Some(true),
                max_pressure,
            );
        }
    }
}

fn find_reachable(
    current: &str,
    valves: &HashMap<String, (usize, Vec<String>)>,
) -> Vec<(String, usize)> {
    let successors = pathfinding::prelude::dijkstra_all(&current.to_string(), |it| {
        let (_, targets) = valves.get(it).unwrap();
        targets.iter().map(|it| (it.clone(), 1)).collect_vec()
    });

    let mut result = vec![];
    for (target, (_, cost)) in successors {
        if valves.get(&target).map_or(0, |(flow, _)| *flow) > 0 {
            result.push((target, cost));
        }
    }

    result
}

fn get_valve_map(valves: &[Valve]) -> HashMap<String, (usize, Vec<(String, usize)>)> {
    let valve_map = valves
        .iter()
        .map(|it| (it.id.clone(), (it.flow_rate, it.targets.clone())))
        .collect::<HashMap<_, _>>();

    valve_map
        .iter()
        .map(|(key, (flow, _))| {
            let reachable_nodes = find_reachable(key, &valve_map);
            (key.clone(), (*flow, reachable_nodes))
        })
        .collect()
}

#[aoc(day16, part1)]
pub fn part1(valves: &[Valve]) -> usize {
    let valve_map = get_valve_map(valves);

    let mut pressure = 0;
    find_best_path(
        &HashSet::from(["AA"]),
        "AA",
        0,
        29,
        &valve_map,
        None,
        &mut pressure,
    );

    pressure
}

#[aoc(day16, part2)]
pub fn part2(valves: &[Valve]) -> usize {
    let valve_map = get_valve_map(valves);

    let mut pressure = 0;
    find_best_path(
        &HashSet::from(["AA"]),
        "AA",
        0,
        25,
        &valve_map,
        Some(false),
        &mut pressure,
    );

    pressure
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB\n\
                              Valve BB has flow rate=13; tunnels lead to valves CC, AA\n\
                              Valve CC has flow rate=2; tunnels lead to valves DD, BB\n\
                              Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE\n\
                              Valve EE has flow rate=3; tunnels lead to valves FF, DD\n\
                              Valve FF has flow rate=0; tunnels lead to valves EE, GG\n\
                              Valve GG has flow rate=0; tunnels lead to valves FF, HH\n\
                              Valve HH has flow rate=22; tunnel leads to valve GG\n\
                              Valve II has flow rate=0; tunnels lead to valves AA, JJ\n\
                              Valve JJ has flow rate=21; tunnel leads to valve II";

    #[test]
    fn test_sample_p1() {
        let data = generate(TEST_INPUT).unwrap();
        let res = part1(&data);
        assert_eq!(res, 1651);
    }

    #[test]
    fn test_sample_p2() {
        let data = generate(TEST_INPUT).unwrap();
        let res = part2(&data);
        assert_eq!(res, 1707);
    }
}
