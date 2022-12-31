use aoc_runner_derive::{aoc, aoc_generator};
use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;
use pathfinding::prelude::Matrix;

const DIRECTIONS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

#[aoc_generator(day8)]
pub fn generate(inp: &str) -> Option<Matrix<u32>> {
    let rows = inp
        .lines()
        .map(|it| it.chars().filter_map(|it| it.to_digit(10)).collect_vec())
        .collect_vec();

    Matrix::from_rows(rows).ok()
}

fn is_tree_visible((y, x): (usize, usize), inp: &Matrix<u32>) -> bool {
    // edges always visible
    if y == 0 || x == 0 || y == inp.rows || x == inp.columns {
        return true;
    }

    let current = inp[(y, x)];
    DIRECTIONS.iter().fold(0, |acc, it| {
        acc + usize::from(inp.in_direction((y, x), *it).all(|p| inp[p] < current))
    }) > 0
}

fn scenic_score(pos: (usize, usize), inp: &Matrix<u32>) -> usize {
    let current = inp[pos];

    DIRECTIONS
        .iter()
        .map(|it| {
            inp.in_direction(pos, *it)
                .fold_while(0, |inner, p| {
                    if inp[p] < current {
                        Continue(inner + 1)
                    } else {
                        Done(inner + 1)
                    }
                })
                .into_inner()
        })
        .product()
}

#[aoc(day8, part1)]
pub fn part1(inp: &Matrix<u32>) -> usize {
    inp.keys()
        .fold(0, |acc, it| acc + usize::from(is_tree_visible(it, inp)))
}

#[aoc(day8, part2)]
pub fn part2(inp: &Matrix<u32>) -> Option<usize> {
    inp.keys().map(|it| scenic_score(it, inp)).max()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "30373\n\
                              25512\n\
                              65332\n\
                              33549\n\
                              35390";

    #[test]
    fn test_sample_p1() {
        let data = generate(TEST_INPUT).expect("Failed generating test input");
        let res = part1(&data);
        assert_eq!(res, 21);
    }

    #[test]
    fn test_score() {
        let data = generate(TEST_INPUT).expect("Failed generating test input");
        let score = scenic_score((1, 2), &data);
        assert_eq!(score, 4);
    }

    #[test]
    fn test_sample_p2() {
        let data = generate(TEST_INPUT).expect("Failed generating test input");
        let res = part2(&data);
        assert_eq!(res, Some(8));
    }
}
