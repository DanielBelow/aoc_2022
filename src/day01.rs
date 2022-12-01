use aoc_runner_derive::{aoc, aoc_generator};

fn sum_calories_per_elf(data: &str) -> usize {
    data.lines().filter_map(|it| it.parse::<usize>().ok()).sum()
}

#[aoc_generator(day01)]
pub fn generate(inp: &str) -> Vec<usize> {
    let mut result = inp.split("\n\n").fold(vec![], |mut acc, block: &str| {
        let calories_per_elf = sum_calories_per_elf(block);
        acc.push(calories_per_elf);
        acc
    });

    result.sort_by(|a, b| b.cmp(a));
    result
}

#[aoc(day01, part1)]
pub fn part1(inp: &[usize]) -> Option<usize> {
    inp.iter().max().copied()
}

#[aoc(day01, part2)]
pub fn part2(inp: &[usize]) -> usize {
    inp.iter().take(3).sum()
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
