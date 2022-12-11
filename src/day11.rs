use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[derive(Clone)]
pub struct Monkey {
    items: Vec<usize>,
    operation: fn(usize) -> usize,
    test: fn(usize) -> bool,
    if_true: usize,
    if_false: usize,
}

fn get_monkeys() -> Vec<Monkey> {
    vec![
        Monkey {
            items: vec![64, 89, 65, 95],
            operation: |it| it * 7,
            test: |it| it % 3 == 0,
            if_true: 4,
            if_false: 1,
        },
        Monkey {
            items: vec![76, 66, 74, 87, 70, 56, 51, 66],
            operation: |it| it + 5,
            test: |it| it % 13 == 0,
            if_true: 7,
            if_false: 3,
        },
        Monkey {
            items: vec![91, 60, 63],
            operation: |it| it * it,
            test: |it| it % 2 == 0,
            if_true: 6,
            if_false: 5,
        },
        Monkey {
            items: vec![92, 61, 79, 97, 79],
            operation: |it| it + 6,
            test: |it| it % 11 == 0,
            if_true: 2,
            if_false: 6,
        },
        Monkey {
            items: vec![93, 54],
            operation: |it| it * 11,
            test: |it| it % 5 == 0,
            if_true: 1,
            if_false: 7,
        },
        Monkey {
            items: vec![60, 79, 92, 69, 88, 82, 70],
            operation: |it| it + 8,
            test: |it| it % 17 == 0,
            if_true: 4,
            if_false: 0,
        },
        Monkey {
            items: vec![64, 57, 73, 89, 55, 53],
            operation: |it| it + 1,
            test: |it| it % 19 == 0,
            if_true: 0,
            if_false: 5,
        },
        Monkey {
            items: vec![62],
            operation: |it| it + 4,
            test: |it| it % 7 == 0,
            if_true: 3,
            if_false: 2,
        },
    ]
}

#[aoc_generator(day11)]
pub fn generate(_: &str) -> Vec<Monkey> {
    get_monkeys()
}

fn run_for_n_steps<F>(n: usize, monkeys: &[Monkey], relief: F) -> usize
where
    F: Fn(usize) -> usize,
{
    let mut monkeys = monkeys.to_vec();
    let num_monkeys = monkeys.len();

    let mut inspects = vec![0; num_monkeys];

    for _ in 0..n {
        for idx in 0..num_monkeys {
            // modify items
            monkeys[idx].items = monkeys[idx]
                .items
                .iter()
                .map(|it| (monkeys[idx].operation)(*it))
                .map(&relief)
                .inspect(|_| inspects[idx] += 1)
                .collect();

            // throw items
            let items = monkeys[idx].items.clone();
            let true_case = monkeys[idx].if_true;
            let false_case = monkeys[idx].if_false;
            for it in items {
                if (monkeys[idx].test)(it) {
                    monkeys[true_case].items.push(it);
                } else {
                    monkeys[false_case].items.push(it);
                }
            }

            monkeys[idx].items.clear();
        }
    }

    inspects.iter().sorted().rev().take(2).product()
}

#[aoc(day11, part1)]
pub fn part1(monkeys: &[Monkey]) -> usize {
    run_for_n_steps(20, monkeys, |it| it / 3)
}

#[aoc(day11, part2)]
pub fn part2(monkeys: &[Monkey]) -> usize {
    let mod_by = 2 * 3 * 5 * 7 * 11 * 13 * 17 * 19;
    run_for_n_steps(10_000, monkeys, |it| it % mod_by)
}
