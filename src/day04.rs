use aoc_runner_derive::{aoc, aoc_generator};
use parse_display::{Display, FromStr};

#[derive(Display, FromStr, Copy, Clone, Debug)]
#[display("{lo}-{hi}")]
pub struct Range {
    lo: i64,
    hi: i64,
}

impl Range {
    fn fully_contains(&self, other: &Self) -> bool {
        other.lo >= self.lo && other.hi <= self.hi
    }

    fn overlaps(&self, other: &Self) -> bool {
        self.lo <= other.hi && self.hi >= other.lo
    }
}

#[derive(Display, FromStr, Copy, Clone, Debug)]
#[display("{lhs},{rhs}")]
pub struct RangePair {
    lhs: Range,
    rhs: Range,
}

#[aoc_generator(day4)]
pub fn generate(inp: &str) -> Vec<RangePair> {
    inp.lines().filter_map(|it| it.parse().ok()).collect()
}

#[aoc(day4, part1)]
pub fn part1(inp: &[RangePair]) -> usize {
    inp.iter()
        .filter(|RangePair { lhs, rhs }| lhs.fully_contains(rhs) || rhs.fully_contains(lhs))
        .count()
}

#[aoc(day4, part2)]
pub fn part2(inp: &[RangePair]) -> usize {
    inp.iter()
        .filter(|RangePair { lhs, rhs }| lhs.overlaps(rhs))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_p1() {
        let inp = "2-4,6-8\n\
                          2-3,4-5\n\
                          5-7,7-9\n\
                          2-8,3-7\n\
                          6-6,4-6\n\
                          2-6,4-8";
        let data = generate(inp);
        let res = part1(&data);
        assert_eq!(res, 2);
    }

    #[test]
    fn test_sample_p2() {
        let inp = "2-4,6-8\n\
                          2-3,4-5\n\
                          5-7,7-9\n\
                          2-8,3-7\n\
                          6-6,4-6\n\
                          2-6,4-8";
        let data = generate(inp);
        let res = part2(&data);
        assert_eq!(res, 4);
    }
}
