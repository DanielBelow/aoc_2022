use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day01)]
pub fn generate(inp: &str) -> Vec<Vec<usize>> {
    inp.split("\n\n").fold(vec![], |mut acc, block| {
        let calories_per_elf = block.lines().filter_map(|l| l.parse().ok()).collect();
        acc.push(calories_per_elf);
        acc
    })
}

#[aoc(day01, part1)]
pub fn part1(inp: &[Vec<usize>]) -> Option<usize> {
    inp.iter().map(|it| it.iter().sum()).max()
}

#[aoc(day01, part2)]
pub fn part2(inp: &[Vec<usize>]) -> usize {
    inp.iter()
        .map(|it| it.iter().sum::<usize>())
        .sorted()
        .rev()
        .take(3)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "1000\n\
                             2000\n\
                             3000\n\
                             \n\
                             4000\n\
                             \n\
                             5000\n\
                             6000\n\
                             \n\
                             7000\n\
                             8000\n\
                             9000\n\
                             \n\
                             10000";

    #[test]
    fn test_sample_p1() {
        let data = generate(TEST_DATA);
        let result = part1(&data);
        assert_eq!(result, Some(24_000));
    }

    #[test]
    fn test_sample_p2() {
        let data = generate(TEST_DATA);
        let result = part2(&data);
        assert_eq!(result, 45_000);
    }
}
