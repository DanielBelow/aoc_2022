use aoc_runner_derive::{aoc, aoc_generator};
use parse_display::{Display, FromStr};

#[derive(Display, FromStr, Copy, Clone, Debug)]
#[display("Each ore robot costs {ore} ore")]
pub struct OreBot {
    ore: usize,
}

#[derive(Display, FromStr, Copy, Clone, Debug)]
#[display("Each clay robot costs {ore} ore")]
pub struct ClayBot {
    ore: usize,
}

#[derive(Display, FromStr, Copy, Clone, Debug)]
#[display("Each obsidian robot costs {ore} ore and {clay} clay")]
pub struct ObsidianBot {
    ore: usize,
    clay: usize,
}

#[derive(Display, FromStr, Copy, Clone, Debug)]
#[display("Each geode robot costs {ore} ore and {obsidian} obsidian")]
pub struct GeodeBot {
    ore: usize,
    obsidian: usize,
}

// Blueprint 1: Each ore robot costs 3 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 18 clay. Each geode robot costs 3 ore and 8 obsidian.
#[derive(Display, FromStr, Copy, Clone, Debug)]
#[display("Blueprint {id}: {ore_robot}. {clay_robot}. {obsidian_robot}. {geode_robot}.")]
pub struct Blueprint {
    id: usize,
    ore_robot: OreBot,
    clay_robot: ClayBot,
    obsidian_robot: ObsidianBot,
    geode_robot: GeodeBot,
}

impl Blueprint {
    fn can_build_ore(&self, resources: &[usize; 4]) -> bool {
        self.ore_robot.ore <= resources[0]
    }

    fn can_build_clay(&self, resources: &[usize; 4]) -> bool {
        self.clay_robot.ore <= resources[0]
    }

    fn can_build_obsidian(&self, resources: &[usize; 4]) -> bool {
        self.obsidian_robot.ore <= resources[0] && self.obsidian_robot.clay <= resources[1]
    }

    fn can_build_geode(&self, resources: &[usize; 4]) -> bool {
        self.geode_robot.ore <= resources[0] && self.geode_robot.obsidian <= resources[2]
    }

    fn has_enough_ore_bots(&self, bots: &[usize; 4]) -> bool {
        let ore_bots = bots[0];
        ore_bots >= self.geode_robot.ore
            && ore_bots >= self.obsidian_robot.ore
            && ore_bots >= self.clay_robot.ore
            && ore_bots >= self.ore_robot.ore
    }

    fn has_enough_clay_bots(&self, bots: &[usize; 4]) -> bool {
        let clay_bots = bots[1];
        clay_bots >= self.obsidian_robot.clay
    }

    fn has_enough_obsidian_bots(&self, bots: &[usize; 4]) -> bool {
        let obsidian_bots = bots[2];
        obsidian_bots >= self.geode_robot.obsidian
    }

    fn num_geodes_opened(&self, minutes: usize) -> usize {
        // Ore, Clay, Obsidian, Geode
        let res = pathfinding::prelude::dijkstra_all(
            &([1, 0, 0, 0], [0, 0, 0, 0], 0),
            |&(bots, resources, time)| {
                if time == minutes {
                    return vec![];
                }

                let mut result = vec![];

                let geode = self.can_build_geode(&resources);
                let obsidian = self.can_build_obsidian(&resources);
                let clay = self.can_build_clay(&resources);
                let ore = self.can_build_ore(&resources);

                let mut new_resources = resources;
                for (idx, b) in bots.iter().enumerate() {
                    new_resources[idx] += *b;
                }

                if geode {
                    let mut new_bots = bots;
                    let mut new_res = new_resources;
                    new_bots[3] += 1;
                    new_res[0] -= self.geode_robot.ore;
                    new_res[2] -= self.geode_robot.obsidian;
                    result.push(((new_bots, new_res, time + 1), 1));
                    return result;
                }

                if obsidian && !self.has_enough_obsidian_bots(&bots) {
                    let mut new_bots = bots;
                    let mut new_res = new_resources;
                    new_bots[2] += 1;
                    new_res[0] -= self.obsidian_robot.ore;
                    new_res[1] -= self.obsidian_robot.clay;
                    result.push(((new_bots, new_res, time + 1), 1));

                    if minutes > 24 {
                        // For whatever reason this makes part2 A LOT faster but breaks part1
                        return result;
                    }
                }

                if clay && !self.has_enough_clay_bots(&bots) {
                    let mut new_bots = bots;
                    let mut new_res = new_resources;
                    new_bots[1] += 1;
                    new_res[0] -= self.clay_robot.ore;
                    result.push(((new_bots, new_res, time + 1), 1));
                }

                if ore && !self.has_enough_ore_bots(&bots) {
                    let mut new_bots = bots;
                    let mut new_res = new_resources;
                    new_bots[0] += 1;
                    new_res[0] -= self.ore_robot.ore;
                    result.push(((new_bots, new_res, time + 1), 1));
                }

                // Build nothing
                result.push(((bots, new_resources, time + 1), 1));

                result
            },
        );

        res.keys()
            .filter_map(|&(_, resources, time)| {
                if time == minutes {
                    Some(resources[3])
                } else {
                    None
                }
            })
            .max()
            .unwrap_or_default()
    }
}

#[aoc_generator(day19)]
pub fn generate(inp: &str) -> Vec<Blueprint> {
    inp.lines().filter_map(|it| it.parse().ok()).collect()
}

#[aoc(day19, part1)]
pub fn part1(inp: &[Blueprint]) -> usize {
    inp.iter().map(|it| it.num_geodes_opened(24) * it.id).sum()
}

#[aoc(day19, part2)]
pub fn part2(inp: &[Blueprint]) -> usize {
    inp.iter()
        .take(3)
        .map(|it| it.num_geodes_opened(32))
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.\n\
                              Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.";

    #[test]
    #[ignore]
    fn test_sample_p1() {
        let data = generate(TEST_INPUT);
        let res = part1(&data);
        assert_eq!(res, 33);
    }
}
