use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day20)]
pub fn generate(inp: &str) -> Vec<i64> {
    inp.lines().filter_map(|it| it.parse().ok()).collect()
}

fn shift_elements(indices: &mut Vec<usize>, inp: &mut [i64]) {
    for (idx, it) in inp.iter().enumerate() {
        let start_idx = indices
            .iter()
            .position(|it| *it == idx)
            .expect("Index has to exist");
        let end_idx = (start_idx as i64 + *it).rem_euclid(inp.len() as i64 - 1) as usize;
        indices.remove(start_idx);
        indices.insert(end_idx, idx);
    }
}

fn sum_elements(nums: &[i64], indices: &[usize]) -> i64 {
    let zero_pos = nums
        .iter()
        .position(|it| *it == 0)
        .expect("Zero has to exist");

    let mapped_zero = indices
        .iter()
        .position(|it| *it == zero_pos)
        .expect("Index has to exist");

    let a = nums[indices[(mapped_zero + 1000) % nums.len()]];
    let b = nums[indices[(mapped_zero + 2000) % nums.len()]];
    let c = nums[indices[(mapped_zero + 3000) % nums.len()]];

    a + b + c
}

#[aoc(day20, part1)]
pub fn part1(inp: &[i64]) -> i64 {
    let mut inp = inp.to_vec();
    let mut indices = (0..inp.len()).collect_vec();

    shift_elements(&mut indices, &mut inp);

    sum_elements(&inp, &indices)
}

#[aoc(day20, part2)]
pub fn part2(inp: &[i64]) -> i64 {
    let mut inp = inp.iter().map(|it| it * 811_589_153).collect_vec();
    let mut indices = (0..inp.len()).collect_vec();

    for _ in 0..10 {
        shift_elements(&mut indices, &mut inp);
    }

    sum_elements(&inp, &indices)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "1\n\
                            2\n\
                            -3\n\
                            3\n\
                            -2\n\
                            0\n\
                            4";

    #[test]
    fn test_sample_p1() {
        let data = generate(TEST_DATA);
        let res = part1(&data);
        assert_eq!(res, 3);
    }

    #[test]
    fn test_sample_p2() {
        let data = generate(TEST_DATA);
        let res = part2(&data);
        assert_eq!(res, 1_623_178_306);
    }
}
