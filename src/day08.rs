use aoc_runner_derive::{aoc, aoc_generator};
use itertools::FoldWhile::{Continue, Done};
use itertools::{iproduct, Itertools};

#[aoc_generator(day8)]
pub fn generate(inp: &str) -> Vec<Vec<u32>> {
    let mut result = vec![];
    for line in inp.lines() {
        let row = line.chars().fold(vec![], |mut acc, it| {
            acc.push(it.to_digit(10).unwrap());
            acc
        });

        result.push(row);
    }

    result
}

fn is_tree_visible(y: usize, x: usize, inp: &[Vec<u32>]) -> bool {
    // edges always visible
    if y == 0 || x == 0 || y == inp.len() || x == inp[0].len() {
        return true;
    }

    let current = inp[y][x];

    let mut result = 0;

    // up
    let before_y = &inp[..y];
    result += usize::from(before_y.iter().all(|it| it[x] < current));

    // down
    let after_y = &inp[y + 1..];
    result += usize::from(after_y.iter().all(|it| it[x] < current));

    // left
    let before_x = &inp[y][..x];
    result += usize::from(before_x.iter().all(|it| *it < current));

    // right
    let after_x = &inp[y][x + 1..];
    result += usize::from(after_x.iter().all(|it| *it < current));

    result > 0
}

fn scenic_score(y: usize, x: usize, inp: &[Vec<u32>]) -> usize {
    let current = inp[y][x];

    let mut scores = vec![];

    // up
    let before_y = &inp[..y];
    let up = before_y
        .iter()
        .rev()
        .fold_while(0, |acc, it| {
            if it[x] < current {
                Continue(acc + 1)
            } else {
                Done(acc + 1)
            }
        })
        .into_inner();

    scores.push(up);

    // down
    let after_y = &inp[y + 1..];
    let down = after_y
        .iter()
        .fold_while(0, |acc, it| {
            if it[x] < current {
                Continue(acc + 1)
            } else {
                Done(acc + 1)
            }
        })
        .into_inner();
    scores.push(down);

    // left
    let before_x = &inp[y][..x];
    let left = before_x
        .iter()
        .rev()
        .fold_while(0, |acc, it| {
            if *it < current {
                Continue(acc + 1)
            } else {
                Done(acc + 1)
            }
        })
        .into_inner();
    scores.push(left);

    // right
    let after_x = &inp[y][x + 1..];
    let right = after_x
        .iter()
        .fold_while(0, |acc, it| {
            if *it < current {
                Continue(acc + 1)
            } else {
                Done(acc + 1)
            }
        })
        .into_inner();
    scores.push(right);

    scores.iter().product()
}

#[aoc(day8, part1)]
pub fn part1(inp: &[Vec<u32>]) -> usize {
    iproduct!(0..inp.len(), 0..inp[0].len()).fold(0, |acc, (row, col)| {
        acc + usize::from(is_tree_visible(row, col, inp))
    })
}

#[aoc(day8, part2)]
pub fn part2(inp: &[Vec<u32>]) -> Option<usize> {
    iproduct!(0..inp.len(), 0..inp[0].len())
        .map(|(row, col)| scenic_score(row, col, inp))
        .max()
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
        let data = generate(TEST_INPUT);
        let res = part1(&data);
        assert_eq!(res, 21);
    }

    #[test]
    fn test_score() {
        let data = generate(TEST_INPUT);
        let score = scenic_score(1, 2, &data);
        assert_eq!(score, 4);
    }

    #[test]
    fn test_sample_p2() {
        let data = generate(TEST_INPUT);
        let res = part2(&data);
        assert_eq!(res, Some(8));
    }
}
