use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::{HashMap, VecDeque};

type Grid = VecDeque<Vec<char>>;
type CacheKey = (usize, usize, Grid);

const GRID_WIDTH: usize = 7;

fn get_rock_patterns() -> [Rock; 5] {
    [
        // ####
        Rock {
            coords: vec![(0, 0), (0, 1), (0, 2), (0, 3)],
            height: 1,
        },
        // .#.
        // ###
        // .#.
        Rock {
            coords: vec![(0, 1), (1, 0), (1, 1), (1, 2), (2, 1)],
            height: 3,
        },
        // ..#
        // ..#
        // ###
        Rock {
            coords: vec![(0, 2), (1, 2), (2, 0), (2, 1), (2, 2)],
            height: 3,
        },
        // #
        // #
        // #
        // #
        Rock {
            coords: vec![(0, 0), (1, 0), (2, 0), (3, 0)],
            height: 4,
        },
        // ##
        // ##
        Rock {
            coords: vec![(0, 0), (0, 1), (1, 0), (1, 1)],
            height: 2,
        },
    ]
}

#[derive(Clone, Debug)]
struct Rock {
    coords: Vec<(usize, usize)>,
    height: usize,
}

impl Rock {
    fn has_collision(&self, dx: usize, dy: usize, grid: &Grid) -> bool {
        self.coords.iter().any(|&(ry, rx)| {
            rx + dx >= GRID_WIDTH || ry + dy >= grid.len() || grid[ry + dy][rx + dx] == '#'
        })
    }
}

struct GameState<const NUM_ROCKS: usize> {
    grid: Grid,
    rocks: [Rock; NUM_ROCKS],
    movements: Vec<char>,
    cache: Option<HashMap<CacheKey, (usize, usize)>>,
    rock_idx: usize,
    mvmt_idx: usize,
}

impl<const NUM_ROCKS: usize> GameState<NUM_ROCKS> {
    fn new(jets: &[char], rocks: [Rock; NUM_ROCKS]) -> Self {
        Self {
            grid: Grid::new(),
            rocks,
            movements: jets.to_vec(),
            cache: None,
            rock_idx: 0,
            mvmt_idx: 0,
        }
    }

    fn enable_caching(&mut self) {
        self.cache = Some(HashMap::new());
    }

    fn disable_caching(&mut self) {
        self.cache = None;
    }

    fn fast_forward(&mut self, cycle_length: usize, num_cycles: usize) {
        self.rock_idx += cycle_length * num_cycles;
    }

    fn spawn_rock(&mut self, idx: usize) -> Rock {
        let rock = self.rocks[idx].clone();

        // New rock spawns 3 above highest rock
        self.grid.push_front(vec!['.'; GRID_WIDTH]);
        self.grid.push_front(vec!['.'; GRID_WIDTH]);
        self.grid.push_front(vec!['.'; GRID_WIDTH]);

        for _ in 0..rock.height {
            self.grid.push_front(vec!['.'; GRID_WIDTH]);
        }

        rock
    }

    fn simulate(&mut self) -> Option<(usize, usize)> {
        let num_movements = self.movements.len();

        let adjusted_rock_idx = self.rock_idx % NUM_ROCKS;
        let adjusted_mvmt_idx = self.mvmt_idx % self.movements.len();

        // Cache current state

        if let Some(ref mut cache) = self.cache {
            let top_thirty = self.grid.iter().take(30).cloned().collect::<Grid>();
            let cache_key = (adjusted_rock_idx, adjusted_mvmt_idx, top_thirty);
            if let Some((old_rock_idx, old_height)) = cache.get(&cache_key) {
                return Some((self.rock_idx - old_rock_idx, self.grid.len() - old_height));
            } else if self.grid.len() >= 30 {
                cache.insert(cache_key, (self.rock_idx, self.grid.len()));
            }
        }

        let rock = self.spawn_rock(adjusted_rock_idx);

        // Current x,y offset of the rock (applies to each coord of rock patterns)
        let (mut dx, mut dy) = (2usize, 0usize);

        loop {
            // Apply jet
            let ndx = if self.movements[self.mvmt_idx % num_movements] == '<' {
                dx.saturating_sub(1)
            } else {
                (dx + 1).clamp(0, GRID_WIDTH - 1)
            };
            self.mvmt_idx += 1;

            // Check collision
            if !rock.has_collision(ndx, dy, &self.grid) {
                dx = ndx;
            }

            // Fall
            let ndy = dy + 1;

            // Check collision
            if rock.has_collision(dx, ndy, &self.grid) {
                break;
            }

            dy = ndy;
        }

        // Land the rock
        for (ry, rx) in rock.coords {
            self.grid[ry + dy][rx + dx] = '#';
        }

        // Drop empty lines, so we can always insert 3 empty lines in the next round
        self.grid.retain(|it| it.iter().any(|c| *c != '.'));

        self.rock_idx += 1;

        None
    }
}

#[aoc_generator(day17)]
pub fn generate(inp: &str) -> Vec<char> {
    inp.chars().collect()
}

#[aoc(day17, part1)]
pub fn part1(jets: &[char]) -> usize {
    let mut state = GameState::new(jets, get_rock_patterns());

    for _ in 0..2022usize {
        let _ = state.simulate();
    }

    state.grid.len()
}

#[aoc(day17, part2)]
pub fn part2(jets: &[char]) -> Option<usize> {
    let mut state = GameState::new(jets, get_rock_patterns());
    state.enable_caching();

    let num_iterations = 1_000_000_000_000usize;
    let (cycle_length, cycle_height) = (0..num_iterations).find_map(|_| state.simulate())?;

    let num_cycles = (num_iterations - state.rock_idx) / cycle_length;
    state.fast_forward(cycle_length, num_cycles);

    state.disable_caching();

    for _ in 0..num_iterations - state.rock_idx {
        let _ = state.simulate();
    }

    Some(cycle_height * num_cycles + state.grid.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_p1() {
        let data = generate(">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>");
        let res = part1(&data);
        assert_eq!(res, 3068);
    }

    #[test]
    fn test_sample_p2() {
        let data = generate(">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>");
        let res = part2(&data);
        assert_eq!(res, Some(1_514_285_714_288));
    }
}
