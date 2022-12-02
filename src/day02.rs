use aoc_runner_derive::{aoc, aoc_generator};
use parse_display::{Display, FromStr};

const ROCK: usize = 1;
const PAPER: usize = 2;
const SCISSORS: usize = 3;

const WIN: usize = 6;
const DRAW: usize = 3;

#[derive(Display, FromStr, Copy, Clone)]
#[display("{lhs} {rhs}")]
struct ParsedInput {
    lhs: char,
    rhs: char,
}

#[aoc_generator(day2)]
pub fn generate(inp: &str) -> Vec<(char, char)> {
    inp.lines()
        .filter_map(|it| it.parse::<ParsedInput>().ok())
        .map(|ParsedInput { lhs, rhs }| (lhs, rhs))
        .collect()
}

#[aoc(day2, part1)]
pub fn part1(inp: &[(char, char)]) -> usize {
    inp.iter()
        .map(|(lhs, rhs)| match (lhs, rhs) {
            ('A', 'X') => ROCK + DRAW,
            ('A', 'Y') => PAPER + WIN,
            ('A', 'Z') => SCISSORS,

            ('B', 'X') => ROCK,
            ('B', 'Y') => PAPER + DRAW,
            ('B', 'Z') => SCISSORS + WIN,

            ('C', 'X') => ROCK + WIN,
            ('C', 'Y') => PAPER,
            ('C', 'Z') => SCISSORS + DRAW,

            _ => unreachable!("Unknown pattern"),
        })
        .sum()
}

#[aoc(day2, part2)]
pub fn part2(inp: &[(char, char)]) -> usize {
    inp.iter()
        .map(|(lhs, rhs)| match (lhs, rhs) {
            ('A', 'X') => SCISSORS,
            ('A', 'Y') => ROCK + DRAW,
            ('A', 'Z') => PAPER + WIN,

            ('B', 'X') => ROCK,
            ('B', 'Y') => PAPER + DRAW,
            ('B', 'Z') => SCISSORS + WIN,

            ('C', 'X') => PAPER,
            ('C', 'Y') => SCISSORS + DRAW,
            ('C', 'Z') => ROCK + WIN,

            _ => unreachable!("Unknown pattern"),
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "A Y\n\
                              B X\n\
                              C Z";

    #[test]
    fn test_sample_p1() {
        let data = generate(TEST_INPUT);
        let res = part1(&data);
        assert_eq!(res, 15);
    }

    #[test]
    fn test_sample_p2() {
        let data = generate(TEST_INPUT);
        let res = part2(&data);
        assert_eq!(res, 12);
    }
}
