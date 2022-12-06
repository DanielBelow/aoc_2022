use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day6)]
pub fn generate(inp: &str) -> Vec<char> {
    inp.chars().collect()
}

fn find_unique_window(data: &[char], size: usize) -> Option<usize> {
    data.windows(size)
        .find_position(|it| it.iter().all_unique())
        .map(|(idx, _)| idx + size)
}

#[aoc(day6, part1)]
pub fn part1(inp: &[char]) -> Option<usize> {
    find_unique_window(inp, 4)
}

#[aoc(day6, part2)]
pub fn part2(inp: &[char]) -> Option<usize> {
    find_unique_window(inp, 14)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_p1() {
        let inp = vec![
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5),
            ("nppdvjthqldpwncqszvftbrmjlhg", 6),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11),
        ];

        for (s, expected) in inp {
            let data = generate(s);
            let res = part1(&data);
            assert_eq!(res, Some(expected));
        }
    }

    #[test]
    fn test_sample_p2() {
        let inp = vec![
            ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 19),
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 23),
            ("nppdvjthqldpwncqszvftbrmjlhg", 23),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 29),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 26),
        ];

        for (s, expected) in inp {
            let data = generate(s);
            let res = part2(&data);
            assert_eq!(res, Some(expected));
        }
    }
}
