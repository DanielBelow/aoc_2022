use aoc_runner_derive::{aoc, aoc_generator};
use itertools::iproduct;

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum Tile {
    Clear,
    Blizzard(Vec<Direction>),
    Wall,
}

fn to_tile(c: char) -> Tile {
    match c {
        '#' => Tile::Wall,
        '.' => Tile::Clear,
        '^' => Tile::Blizzard(vec![Direction::Up]),
        'v' => Tile::Blizzard(vec![Direction::Down]),
        '<' => Tile::Blizzard(vec![Direction::Left]),
        '>' => Tile::Blizzard(vec![Direction::Right]),
        _ => unreachable!(),
    }
}

#[aoc_generator(day24)]
pub fn generate(inp: &str) -> Vec<Vec<Tile>> {
    inp.lines().fold(vec![], |mut acc, line| {
        let row = line.chars().fold(vec![], |mut acc, it| {
            acc.push(to_tile(it));
            acc
        });

        acc.push(row);
        acc
    })
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct State {
    x: usize,
    y: usize,
    grid: Vec<Vec<Tile>>,
}

fn add_blizzard(x: usize, y: usize, direction: Direction, grid: &mut [Vec<Tile>]) {
    if let Tile::Blizzard(ref mut dirs) = grid[y][x] {
        dirs.push(direction);
    } else {
        grid[y][x] = Tile::Blizzard(vec![direction]);
    }
}

fn successors(state: &State) -> Vec<(State, usize)> {
    let height = state.grid.len();
    let width = state.grid[0].len();

    let mut new_grid = vec![vec![Tile::Clear; width]; height];

    // Update grid
    for (y, x) in iproduct!(0..height, 0..width) {
        match &state.grid[y][x] {
            Tile::Clear => {}
            Tile::Blizzard(dirs) => {
                for dir in dirs {
                    match dir {
                        Direction::Up => {
                            let next_y = if y == 1 { height - 2 } else { y - 1 };
                            add_blizzard(x, next_y, Direction::Up, &mut new_grid);
                        }
                        Direction::Down => {
                            let next_y = if y == height - 2 { 1 } else { y + 1 };
                            add_blizzard(x, next_y, Direction::Down, &mut new_grid);
                        }
                        Direction::Left => {
                            let next_x = if x == 1 { width - 2 } else { x - 1 };
                            add_blizzard(next_x, y, Direction::Left, &mut new_grid);
                        }
                        Direction::Right => {
                            let next_x = if x == width - 2 { 1 } else { x + 1 };
                            add_blizzard(next_x, y, Direction::Right, &mut new_grid);
                        }
                    }
                }
            }
            Tile::Wall => new_grid[y][x] = Tile::Wall,
        };
    }

    let mut result = vec![];

    // Down
    if state.y < height - 1 && new_grid[state.y + 1][state.x] == Tile::Clear {
        result.push((
            State {
                x: state.x,
                y: state.y + 1,
                grid: new_grid.clone(),
            },
            1,
        ));
    }

    // Right
    if state.x < width - 1 && new_grid[state.y][state.x + 1] == Tile::Clear {
        result.push((
            State {
                x: state.x + 1,
                y: state.y,
                grid: new_grid.clone(),
            },
            1,
        ));
    }

    // Up
    if state.y > 0 && new_grid[state.y - 1][state.x] == Tile::Clear {
        result.push((
            State {
                x: state.x,
                y: state.y - 1,
                grid: new_grid.clone(),
            },
            1,
        ));
    }

    // Left
    if state.x > 0 && new_grid[state.y][state.x - 1] == Tile::Clear {
        result.push((
            State {
                x: state.x - 1,
                y: state.y,
                grid: new_grid.clone(),
            },
            1,
        ));
    }

    if new_grid[state.y][state.x] == Tile::Clear {
        result.push((
            State {
                x: state.x,
                y: state.y,
                grid: new_grid.clone(),
            },
            1,
        ));
    }

    result
}

fn heuristic(state: &State, gx: usize, gy: usize) -> usize {
    state.x.abs_diff(gx) + state.y.abs_diff(gy)
}

fn find_path_to(gx: usize, gy: usize, grid: &[Vec<Tile>]) -> Option<usize> {
    let start_state = State {
        x: 1,
        y: 0,
        grid: grid.to_vec(),
    };

    let (path, _) = pathfinding::prelude::astar(
        &start_state,
        successors,
        |it| heuristic(it, gx, gy),
        |it| it.x == gx && it.y == gy,
    )?;

    Some(path.len() - 1)
}

fn find_roundtrips_to(gx: usize, gy: usize, grid: &[Vec<Tile>]) -> Option<usize> {
    let start_state = State {
        x: 1,
        y: 0,
        grid: grid.to_vec(),
    };

    let mut cur_time = 0;

    let (path, _) = pathfinding::prelude::astar(
        &start_state,
        successors,
        |it| heuristic(it, gx, gy),
        |it| it.x == gx && it.y == gy,
    )?;

    cur_time += path.len() - 1;

    let last_state = path.last()?;
    let (path, _) = pathfinding::prelude::astar(
        last_state,
        successors,
        |it| heuristic(it, gx, gy),
        |it| it.x == 1 && it.y == 0,
    )?;

    cur_time += path.len() - 1;

    let last_state = path.last()?;
    let (path, _) = pathfinding::prelude::astar(
        last_state,
        successors,
        |it| heuristic(it, gx, gy),
        |it| it.x == gx && it.y == gy,
    )?;

    cur_time += path.len() - 1;

    Some(cur_time)
}

#[aoc(day24, part1)]
pub fn part1(grid: &[Vec<Tile>]) -> Option<usize> {
    find_path_to(100, 36, grid)
}

#[aoc(day24, part2)]
pub fn part2(grid: &[Vec<Tile>]) -> Option<usize> {
    find_roundtrips_to(100, 36, grid)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "#.######\n\
                             #>>.<^<#\n\
                             #.<..<<#\n\
                             #>v.><>#\n\
                             #<^v^^>#\n\
                             ######.#";

    #[test]
    fn test_sample_p1() {
        let data = generate(TEST_DATA);
        let res = find_path_to(6, 5, &data);
        assert_eq!(res, Some(18));
    }

    #[test]
    fn test_sample_p2() {
        let data = generate(TEST_DATA);
        let res = find_roundtrips_to(6, 5, &data);
        assert_eq!(res, Some(54));
    }
}
