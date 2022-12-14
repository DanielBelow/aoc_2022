use aoc_runner_derive::{aoc, aoc_generator};
use parse_display_derive::{Display, FromStr};

#[derive(Display, FromStr, PartialEq, Eq, Copy, Clone, Debug)]
#[display("{x},{y}")]
pub struct Pair {
    x: usize,
    y: usize,
}

#[derive(Clone, Debug)]
pub struct Chain {
    pairs: Vec<Pair>,
}

#[aoc_generator(day14)]
pub fn generate(inp: &str) -> Vec<Chain> {
    inp.lines().fold(vec![], |mut acc, it| {
        let pairs = it.split(" -> ").fold(vec![], |mut acc, it| {
            if let Ok(pair) = it.parse::<Pair>() {
                acc.push(pair);
            }
            acc
        });

        acc.push(Chain { pairs });
        acc
    })
}

fn make_grid(chains: &[Chain], with_floor: bool) -> Vec<Vec<char>> {
    let max_x = chains
        .iter()
        .flat_map(|it| it.pairs.clone())
        .max_by_key(|it| it.x)
        .unwrap()
        .x;

    let max_y = chains
        .iter()
        .flat_map(|it| it.pairs.clone())
        .max_by_key(|it| it.y)
        .unwrap()
        .y;

    let mut grid = vec![vec!['.'; max_x + 1]; max_y + 1];

    if with_floor {
        grid.push(vec!['.'; max_x + 1]);
        grid.push(vec!['#'; max_x + 1]);
    }

    for chain in chains {
        for pair in chain.pairs.windows(2) {
            if let [lhs, rhs] = pair {
                if lhs.x == rhs.x {
                    let range = if lhs.y < rhs.y {
                        lhs.y..=rhs.y
                    } else {
                        rhs.y..=lhs.y
                    };

                    grid[range].iter_mut().for_each(|it| it[lhs.x] = '#');
                } else {
                    assert_eq!(lhs.y, rhs.y);

                    let row = &mut grid[lhs.y];

                    let range = if lhs.x < rhs.x {
                        lhs.x..=rhs.x
                    } else {
                        rhs.x..=lhs.x
                    };

                    row[range].iter_mut().for_each(|it| *it = '#');
                }
            }
        }
    }

    grid
}

enum PositionState {
    Resting,
    Moved(Pair),
    FellOff,
    GrowLeft,
    GrowRight,
}

// Return new position or none if can't move
fn try_move(sand: Pair, grid: &[Vec<char>], with_floor: bool) -> PositionState {
    let Pair { x, y } = sand;

    if y + 1 >= grid.len() {
        return if with_floor {
            PositionState::Resting
        } else {
            PositionState::FellOff
        };
    }

    // down
    if grid[y + 1][x] == '.' {
        return PositionState::Moved(Pair { x, y: y + 1 });
    }

    if x == 0 {
        return if with_floor {
            PositionState::GrowLeft
        } else {
            PositionState::FellOff
        };
    }

    // down+left
    if grid[y + 1][x - 1] == '.' {
        return PositionState::Moved(Pair { x: x - 1, y: y + 1 });
    }

    if x + 1 >= grid[y].len() {
        return if with_floor {
            PositionState::GrowRight
        } else {
            PositionState::FellOff
        };
    }

    // down+right
    if grid[y + 1][x + 1] == '.' {
        return PositionState::Moved(Pair { x: x + 1, y: y + 1 });
    }

    // rest
    PositionState::Resting
}

#[aoc(day14, part1)]
pub fn part1(chains: &[Chain]) -> usize {
    let sand_source = Pair { x: 500, y: 0 };
    let mut grid = make_grid(chains, false);

    loop {
        let mut new_sand = sand_source;

        loop {
            let state = try_move(new_sand, &grid, false);
            match state {
                PositionState::Resting => {
                    grid[new_sand.y][new_sand.x] = 'O';
                    break;
                }
                PositionState::Moved(new_pos) => {
                    new_sand = new_pos;
                }
                PositionState::FellOff => {
                    return grid.iter().flatten().filter(|it| **it == 'O').count();
                }
                PositionState::GrowLeft | PositionState::GrowRight => {
                    unreachable!("Not available in part1!")
                }
            };
        }
    }
}

#[aoc(day14, part2)]
pub fn part2(chains: &[Chain]) -> usize {
    let mut sand_source = Pair { x: 500, y: 0 };
    let mut grid = make_grid(chains, true);

    loop {
        let mut new_sand = sand_source;

        if grid[sand_source.y][sand_source.x] == 'O' {
            return grid.iter().flatten().filter(|it| **it == 'O').count();
        }

        loop {
            let state = try_move(new_sand, &grid, true);
            match state {
                PositionState::Resting => {
                    grid[new_sand.y][new_sand.x] = 'O';
                    break;
                }
                PositionState::Moved(new_pos) => {
                    new_sand = new_pos;
                }
                PositionState::GrowLeft => {
                    let grid_len = grid.len();
                    for (idx, row) in grid.iter_mut().enumerate() {
                        let c = if idx == grid_len - 1 { '#' } else { '.' };
                        row.insert(0, c);
                    }

                    sand_source.x += 1;
                }
                PositionState::GrowRight => {
                    let grid_len = grid.len();
                    for (idx, row) in grid.iter_mut().enumerate() {
                        let c = if idx == grid_len - 1 { '#' } else { '.' };
                        row.push(c);
                    }
                }
                PositionState::FellOff => unreachable!("Can not fall off in part2!"),
            };
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "498,4 -> 498,6 -> 496,6\n\
                              503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn test_sample_p1() {
        let data = generate(TEST_INPUT);
        let res = part1(&data);
        assert_eq!(res, 24);
    }

    #[test]
    fn test_sample_p2() {
        let data = generate(TEST_INPUT);
        let res = part2(&data);
        assert_eq!(res, 93);
    }
}
