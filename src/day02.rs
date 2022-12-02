use aoc_runner_derive::{aoc, aoc_generator};
use parse_display::{Display, FromStr};

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum RockPaperScissor {
    Rock,
    Paper,
    Scissor,
}

impl From<char> for RockPaperScissor {
    fn from(value: char) -> Self {
        match value {
            'A' | 'X' => RockPaperScissor::Rock,
            'B' | 'Y' => RockPaperScissor::Paper,
            'C' | 'Z' => RockPaperScissor::Scissor,
            _ => unreachable!(),
        }
    }
}

impl RockPaperScissor {
    const LOSS: usize = 0;
    const DRAW: usize = 3;
    const WIN: usize = 6;

    fn value(self) -> usize {
        match self {
            RockPaperScissor::Rock => 1,
            RockPaperScissor::Paper => 2,
            RockPaperScissor::Scissor => 3,
        }
    }

    fn fight(self, other: RockPaperScissor) -> usize {
        match (self, other) {
            (RockPaperScissor::Rock, RockPaperScissor::Rock)
            | (RockPaperScissor::Paper, RockPaperScissor::Paper)
            | (RockPaperScissor::Scissor, RockPaperScissor::Scissor) => Self::DRAW,
            (RockPaperScissor::Rock, RockPaperScissor::Paper)
            | (RockPaperScissor::Scissor, RockPaperScissor::Rock)
            | (RockPaperScissor::Paper, RockPaperScissor::Scissor) => Self::LOSS,
            (RockPaperScissor::Rock, RockPaperScissor::Scissor)
            | (RockPaperScissor::Paper, RockPaperScissor::Rock)
            | (RockPaperScissor::Scissor, RockPaperScissor::Paper) => Self::WIN,
        }
    }

    fn score_against(self, other: RockPaperScissor) -> usize {
        self.value() + self.fight(other)
    }

    fn choose(self, wdl: char) -> RockPaperScissor {
        // X: Lose
        // Y: Draw
        // Z: Win
        match (self, wdl) {
            (RockPaperScissor::Rock, 'X')
            | (RockPaperScissor::Paper, 'Z')
            | (RockPaperScissor::Scissor, 'Y') => RockPaperScissor::Scissor,

            (RockPaperScissor::Rock, 'Y')
            | (RockPaperScissor::Paper, 'X')
            | (RockPaperScissor::Scissor, 'Z') => RockPaperScissor::Rock,

            (RockPaperScissor::Rock, 'Z')
            | (RockPaperScissor::Paper, 'Y')
            | (RockPaperScissor::Scissor, 'X') => RockPaperScissor::Paper,

            _ => unreachable!(),
        }
    }
}

#[derive(Display, FromStr, Copy, Clone)]
#[display("{lhs} {rhs}")]
struct ParsedInput {
    lhs: char,
    rhs: char,
}

#[aoc_generator(day2)]
pub fn generate(inp: &str) -> Vec<(RockPaperScissor, char)> {
    inp.lines()
        .filter_map(|it| it.parse::<ParsedInput>().ok())
        .map(|it| (RockPaperScissor::from(it.lhs), it.rhs))
        .collect()
}

#[aoc(day2, part1)]
pub fn part1(inp: &[(RockPaperScissor, char)]) -> usize {
    inp.iter()
        .map(|(lhs, rhs)| (lhs, RockPaperScissor::from(*rhs)))
        .fold(0, |acc, (lhs, rhs)| acc + rhs.score_against(*lhs))
}

#[aoc(day2, part2)]
pub fn part2(inp: &[(RockPaperScissor, char)]) -> usize {
    inp.iter()
        .map(|(lhs, rhs)| (lhs, lhs.choose(*rhs)))
        .fold(0, |acc, (lhs, rhs)| acc + rhs.score_against(*lhs))
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
