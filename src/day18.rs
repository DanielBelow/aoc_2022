use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use parse_display_derive::{Display, FromStr};
use std::collections::{HashMap, HashSet};

#[derive(Display, FromStr, PartialEq, Eq, Hash, Copy, Clone, Debug)]
#[display("{x},{y},{z}")]
pub struct Cube {
    x: isize,
    y: isize,
    z: isize,
}

impl Cube {
    const fn distance_to(&self, other: &Self) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y) + self.z.abs_diff(other.z)
    }

    fn adjacent_sides(&self) -> Vec<Self> {
        vec![
            Self {
                x: self.x + 1,
                y: self.y,
                z: self.z,
            },
            Self {
                x: self.x - 1,
                y: self.y,
                z: self.z,
            },
            Self {
                x: self.x,
                y: self.y + 1,
                z: self.z,
            },
            Self {
                x: self.x,
                y: self.y - 1,
                z: self.z,
            },
            Self {
                x: self.x,
                y: self.y,
                z: self.z + 1,
            },
            Self {
                x: self.x,
                y: self.y,
                z: self.z - 1,
            },
        ]
    }
}

#[aoc_generator(day18)]
pub fn generate(inp: &str) -> Vec<Cube> {
    inp.lines().filter_map(|it| it.parse().ok()).collect()
}

#[aoc(day18, part1)]
pub fn part1(cubes: &[Cube]) -> usize {
    let mut counts = cubes.iter().map(|it| (*it, 6)).collect::<HashMap<_, _>>();

    for cube in cubes {
        for other in cubes {
            if cube == other {
                continue;
            }

            if cube.distance_to(other) == 1 {
                if let Some(it) = counts.get_mut(cube) {
                    *it -= 1;
                }
            }
        }
    }

    counts.values().sum()
}

fn find_path_to_origin(
    cubes: &[Cube],
    reaches_origin: &HashSet<Cube>,
    side: &Cube,
) -> Option<(Vec<Cube>, usize)> {
    let origin = Cube { x: 0, y: 0, z: 0 };

    pathfinding::prelude::astar(
        side,
        |it| {
            let sides = it.adjacent_sides();
            sides
                .iter()
                .filter(|it| !cubes.contains(it))
                .map(|it| (*it, 1))
                .collect_vec()
        },
        |it| it.distance_to(&origin),
        |it| *it == origin || reaches_origin.contains(it),
    )
}

#[aoc(day18, part2)]
pub fn part2(cubes: &[Cube]) -> usize {
    let mut result = part1(cubes);

    let mut air_pocket: HashSet<Cube> = HashSet::new();
    let mut reaches_origin: HashSet<Cube> = HashSet::new();

    for side in cubes
        .iter()
        .flat_map(Cube::adjacent_sides)
        .filter(|it| !cubes.contains(it))
    {
        if air_pocket.contains(&side) {
            // Skip pathfinding if 'side' is known to be an air-pocket
            result -= 1;
            continue;
        }

        // Try to find a path to the origin [0, 0, 0] and remember found nodes
        if let Some((path, _)) = find_path_to_origin(cubes, &reaches_origin, &side) {
            reaches_origin.extend(path.iter());
        } else {
            // No path to the origin -> this is an air-pocket
            air_pocket.insert(side);
            result -= 1;

            // Find all reachable nodes inside the pocket and cache them
            let reachable_nodes = pathfinding::prelude::bfs_reach(side, |it| {
                let sides = it.adjacent_sides();
                sides
                    .iter()
                    .filter(|it| !cubes.contains(it))
                    .copied()
                    .collect_vec()
            });

            for node in reachable_nodes {
                air_pocket.insert(node);
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "2,2,2\n\
                              1,2,2\n\
                              3,2,2\n\
                              2,1,2\n\
                              2,3,2\n\
                              2,2,1\n\
                              2,2,3\n\
                              2,2,4\n\
                              2,2,6\n\
                              1,2,5\n\
                              3,2,5\n\
                              2,1,5\n\
                              2,3,5";

    #[test]
    fn test_sample_p1() {
        let data = generate(TEST_INPUT);
        let res = part1(&data);
        assert_eq!(res, 64);
    }

    #[test]
    fn test_sample_p2() {
        let data = generate(TEST_INPUT);
        let res = part2(&data);
        assert_eq!(res, 58);
    }
}
