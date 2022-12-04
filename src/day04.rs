use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use parse_display::{Display, FromStr};
use std::collections::HashSet;

#[derive(Display, FromStr, Debug)]
#[display("{al}-{ah},{bl}-{bh}")]
pub struct ParsedData {
    al: i64,
    ah: i64,
    bl: i64,
    bh: i64,
}

pub struct SectionAssignment {
    a: HashSet<i64>,
    b: HashSet<i64>,
}

#[aoc_generator(day4)]
pub fn generate(inp: &str) -> Vec<SectionAssignment> {
    inp.lines()
        .filter_map(|it| it.parse::<ParsedData>().ok())
        .map(|ParsedData { al, ah, bl, bh }| SectionAssignment {
            a: (al..=ah).collect(),
            b: (bl..=bh).collect(),
        })
        .collect_vec()
}

#[aoc(day4, part1)]
pub fn part1(inp: &[SectionAssignment]) -> usize {
    inp.iter()
        .filter(|SectionAssignment { a, b }| a.is_subset(b) || b.is_subset(a))
        .count()
}

#[aoc(day4, part2)]
pub fn part2(inp: &[SectionAssignment]) -> usize {
    inp.iter()
        .filter(|SectionAssignment { a, b }| {
            a.intersection(b).count() != 0 || b.intersection(a).count() != 0
        })
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
}
