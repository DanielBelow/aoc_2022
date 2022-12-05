use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use parse_display::{Display, FromStr};

#[derive(Display, FromStr, Copy, Clone, Debug)]
#[display("move {num} from {from} to {to}")]
pub struct Operation {
    num: usize,
    from: usize,
    to: usize,
}

#[derive(Clone, Debug)]
pub struct ParsedData {
    stacks: Vec<Vec<char>>,
    operations: Vec<Operation>,
}

#[aoc_generator(day5)]
pub fn generate(inp: &str) -> Option<ParsedData> {
    let stacks = vec![
        vec!['N', 'S', 'D', 'C', 'V', 'Q', 'T'],
        vec!['M', 'F', 'V'],
        vec!['F', 'Q', 'W', 'D', 'P', 'N', 'H', 'M'],
        vec!['D', 'Q', 'R', 'T', 'F'],
        vec!['R', 'F', 'M', 'N', 'Q', 'H', 'V', 'B'],
        vec!['C', 'F', 'G', 'N', 'P', 'W', 'Q'],
        vec!['W', 'F', 'R', 'L', 'C', 'T'],
        vec!['T', 'Z', 'N', 'S'],
        vec!['M', 'S', 'D', 'J', 'R', 'Q', 'H', 'N'],
    ];

    let ops = inp.split("\n\n").nth(1)?;

    let operations = ops.lines().filter_map(|it| it.parse().ok()).collect_vec();

    Some(ParsedData { stacks, operations })
}

#[aoc(day5, part1)]
pub fn part1(inp: &ParsedData) -> String {
    let mut cur_state = inp.stacks.clone();

    for &Operation { num, from, to } in &inp.operations {
        for _ in 0..num {
            let elem = cur_state[from - 1].pop().unwrap();
            cur_state[to - 1].push(elem);
        }
    }

    cur_state.iter().filter_map(|it| it.iter().last()).join("")
}

#[aoc(day5, part2)]
pub fn part2(inp: &ParsedData) -> String {
    let mut cur_state = inp.stacks.clone();

    for &Operation { num, from, to } in &inp.operations {
        let mut to_move = vec![];
        for _ in 0..num {
            let elem = cur_state[from - 1].pop().unwrap();
            to_move.push(elem);
        }

        for e in to_move.iter().rev() {
            cur_state[to - 1].push(*e);
        }
    }

    cur_state.iter().filter_map(|it| it.iter().last()).join("")
}
