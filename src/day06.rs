use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::hash_map::Entry;
use std::collections::HashMap;

#[aoc_generator(day6)]
pub fn generate(inp: &str) -> Vec<char> {
    inp.chars().collect()
}

fn find_unique_window(data: &[char], size: usize) -> usize {
    // Keep track of chars and their first occurrence
    let mut seen = HashMap::new();

    let mut idx = 0;
    'outer: loop {
        // Clear previous iteration
        seen.clear();

        for (collision_idx, c) in data[idx..idx + size].iter().enumerate() {
            match seen.entry(c) {
                // If we have seen this char already, move our idx until 1 after the first occurrence
                // e.g. for "abcc" we don't have to check index 1, since the first collision
                // was at indices 2 and 3
                Entry::Occupied(e) => {
                    idx += *e.get() + 1;
                    continue 'outer;
                }
                // Insert & keep iterating
                Entry::Vacant(n) => {
                    n.insert(collision_idx);
                }
            }
        }

        return idx + size;
    }
}

#[aoc(day6, part1)]
pub fn part1(inp: &[char]) -> usize {
    find_unique_window(inp, 4)
}

#[aoc(day6, part2)]
pub fn part2(inp: &[char]) -> usize {
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
            assert_eq!(res, expected);
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
            assert_eq!(res, expected);
        }
    }
}
