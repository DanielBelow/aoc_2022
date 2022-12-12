use aoc_runner_derive::{aoc, aoc_generator};
use itertools::iproduct;

#[aoc_generator(day12)]
pub fn generate(inp: &str) -> Vec<Vec<char>> {
    inp.lines().fold(vec![], |mut acc, it| {
        acc.push(it.chars().collect());
        acc
    })
}

fn char_to_elevation(c: char) -> u8 {
    if c == 'S' {
        0
    } else if c == 'E' {
        b'z' - b'a'
    } else {
        c as u8 - b'a'
    }
}

fn successors((row, col): (usize, usize), grid: &[Vec<char>]) -> Vec<((usize, usize), usize)> {
    let mut result = vec![];

    let current_elevation = char_to_elevation(grid[row][col]);
    let is_one_step_up = |next| next <= current_elevation || next == current_elevation + 1;

    // up
    if row > 0 {
        let next = char_to_elevation(grid[row - 1][col]);
        if is_one_step_up(next) {
            result.push(((row - 1, col), 1));
        }
    }
    // down
    if row < grid.len() - 1 {
        let next = char_to_elevation(grid[row + 1][col]);
        if is_one_step_up(next) {
            result.push(((row + 1, col), 1));
        }
    }

    // left
    if col > 0 {
        let next = char_to_elevation(grid[row][col - 1]);
        if is_one_step_up(next) {
            result.push(((row, col - 1), 1));
        }
    }
    // right
    if col < grid[row].len() - 1 {
        let next = char_to_elevation(grid[row][col + 1]);
        if is_one_step_up(next) {
            result.push(((row, col + 1), 1));
        }
    }

    result
}

fn find_shortest_path_from(start: (usize, usize), grid: &[Vec<char>]) -> Option<usize> {
    let (_, cost) = pathfinding::prelude::dijkstra(
        &start,
        |it| successors(*it, grid),
        |(row, col)| grid[*row][*col] == 'E',
    )?;

    Some(cost)
}

#[aoc(day12, part1)]
pub fn part1(inp: &[Vec<char>]) -> Option<usize> {
    let mut start_pos = None;
    for (row, col) in iproduct!(0..inp.len(), 0..inp[0].len()) {
        if inp[row][col] == 'S' {
            start_pos = Some((row, col));
            break;
        }
    }

    find_shortest_path_from(start_pos?, inp)
}

#[aoc(day12, part2)]
pub fn part2(inp: &[Vec<char>]) -> Option<usize> {
    let mut start_squares = vec![];
    for (row, col) in iproduct!(0..inp.len(), 0..inp[0].len()) {
        if inp[row][col] == 'a' || inp[row][col] == 'S' {
            start_squares.push((row, col));
        }
    }

    start_squares
        .iter()
        .filter_map(|it| find_shortest_path_from(*it, inp))
        .min()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "Sabqponm\n\
                             abcryxxl\n\
                             accszExk\n\
                             acctuvwj\n\
                             abdefghi";

    #[test]
    fn test_sample_p1() {
        let data = generate(TEST_INPUT);
        let res = part1(&data);
        assert_eq!(res.unwrap(), 31);
    }

    #[test]
    fn test_sample_p2() {
        let data = generate(TEST_INPUT);
        let res = part2(&data);
        assert_eq!(res.unwrap(), 29);
    }
}
