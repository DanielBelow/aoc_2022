use aoc_runner_derive::{aoc, aoc_generator};
use parse_display::{Display, FromStr};
use std::collections::HashMap;

#[derive(Display, FromStr, Clone, Debug)]
pub enum ChangeDir {
    #[display("..")]
    Up,

    #[display("{0}")]
    Subdir(String),
}

#[derive(Display, FromStr, Clone, Debug)]
pub enum Command {
    #[display("$ cd {0}")]
    CD(ChangeDir),

    #[display("{0} {1}")]
    Entry(usize, String),
}

#[aoc_generator(day7)]
pub fn generate(inp: &str) -> Vec<Command> {
    inp.lines().filter_map(|it| it.parse().ok()).collect()
}

fn collect_command_output(cmds: &[Command]) -> HashMap<String, usize> {
    let mut directory_stack: Vec<&str> = vec![];

    let mut result = HashMap::new();
    result.insert("/".to_string(), 0);

    for cmd in cmds {
        match cmd {
            Command::CD(ChangeDir::Up) => {
                directory_stack.pop();
            }
            Command::CD(ChangeDir::Subdir(dir_name)) => {
                directory_stack.push(dir_name);
            }
            Command::Entry(size, _) => {
                // Just add the size to all parent directories as well
                for idx in 0..directory_stack.len() {
                    let cur_dir_name = directory_stack[..=idx].join("/");
                    *result.entry(cur_dir_name).or_insert(0) += size;
                }
            }
        };
    }

    result
}

#[aoc(day7, part1)]
pub fn part1(cmds: &[Command]) -> usize {
    let dirs = collect_command_output(cmds);
    dirs.values().filter(|it| **it <= 100_000).sum()
}

#[aoc(day7, part2)]
pub fn part2(cmds: &[Command]) -> Option<usize> {
    const TOTAL_SIZE: usize = 70_000_000;
    const EMPTY_NEEDED: usize = 30_000_000;

    let dirs = collect_command_output(cmds);

    let cur_size = *dirs.get("/")?;
    let needed_cleanup = EMPTY_NEEDED - (TOTAL_SIZE - cur_size);

    dirs.values()
        .filter(|it| **it >= needed_cleanup)
        .min()
        .copied()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "$ cd /\n\
                             $ ls\n\
                             dir a\n\
                             14848514 b.txt\n\
                             8504156 c.dat\n\
                             dir d\n\
                             $ cd a\n\
                             $ ls\n\
                             dir e\n\
                             29116 f\n\
                             2557 g\n\
                             62596 h.lst\n\
                             $ cd e\n\
                             $ ls\n\
                             584 i\n\
                             $ cd ..\n\
                             $ cd ..\n\
                             $ cd d\n\
                             $ ls\n\
                             4060174 j\n\
                             8033020 d.log\n\
                             5626152 d.ext\n\
                             7214296 k";

    #[test]
    fn test_sample_p1() {
        let data = generate(TEST_DATA);
        let res = part1(&data);
        assert_eq!(res, 95437);
    }

    #[test]
    fn test_sample_p2() {
        let data = generate(TEST_DATA);
        let res = part2(&data);
        assert_eq!(res, Some(24_933_642));
    }
}
