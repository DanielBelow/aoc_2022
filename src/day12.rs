use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use pathfinding::matrix::Matrix;

#[aoc_generator(day12)]
pub fn generate(inp: &str) -> Option<Matrix<char>> {
    let rows = inp.lines().map(|it| it.chars().collect_vec()).collect_vec();
    Matrix::from_rows(rows).ok()
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

fn successors((row, col): (usize, usize), grid: &Matrix<char>) -> Vec<((usize, usize), usize)> {
    let current_elevation = char_to_elevation(grid[(row, col)]);
    let is_one_step_up = |next| next <= current_elevation || next == current_elevation + 1;

    grid.neighbours((row, col), false)
        .fold(vec![], |mut acc, it| {
            if is_one_step_up(char_to_elevation(grid[it])) {
                acc.push((it, 1));
            }

            acc
        })
}

fn find_shortest_path_from(start: (usize, usize), grid: &Matrix<char>) -> Option<usize> {
    let (_, cost) = pathfinding::prelude::dijkstra(
        &start,
        |it| successors(*it, grid),
        |pos| grid[*pos] == 'E',
    )?;

    Some(cost)
}

#[aoc(day12, part1)]
pub fn part1(inp: &Matrix<char>) -> Option<usize> {
    let start_pos = inp.keys().find(|it| inp[*it] == 'S')?;
    find_shortest_path_from(start_pos, inp)
}

#[aoc(day12, part2)]
pub fn part2(inp: &Matrix<char>) -> Option<usize> {
    inp.keys()
        .filter_map(|it| {
            if inp[it] == 'a' || inp[it] == 'S' {
                find_shortest_path_from(it, inp)
            } else {
                None
            }
        })
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
        let data = generate(TEST_INPUT).unwrap();
        let res = part1(&data);
        assert_eq!(res.unwrap(), 31);
    }

    #[test]
    fn test_sample_p2() {
        let data = generate(TEST_INPUT).unwrap();
        let res = part2(&data);
        assert_eq!(res.unwrap(), 29);
    }
}
