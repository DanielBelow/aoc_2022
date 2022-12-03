use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::collections::HashSet;

#[aoc_generator(day3)]
pub fn generate(inp: &str) -> Vec<String> {
    inp.lines().map(ToString::to_string).collect()
}

fn sum_common_chars(common: &[char]) -> usize {
    common.iter().fold(0, |acc, &it| {
        acc + if it.is_lowercase() {
            (1 + (it as u8) - b'a') as usize
        } else {
            (27 + (it as u8) - b'A') as usize
        }
    })
}

#[aoc(day3, part1)]
pub fn part1(inp: &[String]) -> usize {
    inp.iter().fold(0, |acc, it| {
        let (r1, r2) = it.split_at(it.len() / 2);
        let s1 = r1.chars().collect::<HashSet<_>>();
        let s2 = r2.chars().collect::<HashSet<_>>();

        let shared = s1.intersection(&s2).copied().collect_vec();

        acc + sum_common_chars(&shared)
    })
}

#[aoc(day3, part2)]
pub fn part2(inp: &[String]) -> usize {
    inp.chunks_exact(3).fold(0, |acc, it| {
        if let [l1, l2, l3] = it {
            let s1 = l1.chars().collect::<HashSet<_>>();
            let s2 = l2.chars().collect::<HashSet<_>>();
            let s3 = l3.chars().collect::<HashSet<_>>();

            let shared_in_group = s1
                .intersection(&s2)
                .copied()
                .collect::<HashSet<_>>()
                .intersection(&s3)
                .copied()
                .collect_vec();

            acc + sum_common_chars(&shared_in_group)
        } else {
            acc
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "vJrwpWtwJgWrhcsFMMfFFhFp\n\
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\n\
PmmdzqPrVvPwwTWBwg\n\
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\n\
ttgJtRGJQctTZtZT\n\
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn test_sample_p1() {
        let data = generate(TEST_DATA);
        let res = part1(&data);
        assert_eq!(res, 157);
    }

    #[test]
    fn test_sample_p2() {
        let data = generate(TEST_DATA);
        let res = part2(&data);
        assert_eq!(res, 70);
    }
}
