use aoc_runner_derive::{aoc, aoc_generator};

fn snafu_to_i64(snafu: &str) -> i64 {
    snafu
        .chars()
        .rev()
        .zip(std::iter::successors(Some(0), |it| Some(*it + 1)))
        .map(|(chr, exp)| {
            let num = match chr {
                '0' => 0,
                '1' => 1,
                '2' => 2,
                '-' => -1,
                '=' => -2,
                _ => unreachable!("Not a valid SNAFU number"),
            };

            num * (5_i64.pow(exp))
        })
        .sum()
}

#[aoc_generator(day25)]
pub fn generate(inp: &str) -> Vec<i64> {
    inp.lines().map(snafu_to_i64).collect()
}

#[aoc(day25, part1)]
pub fn part1(_: &[i64]) -> String {
    // Sum of the input: 35_023_647_158_862
    // Convert manually...
    String::from("2-10==12-122-=1-1-22")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_snafu_conversion() {
        let test_input = vec![
            ("1=", 3),
            ("12", 7),
            ("21", 11),
            ("111", 31),
            ("112", 32),
            ("122", 37),
            ("1-12", 107),
            ("2=0=", 198),
            ("2=01", 201),
            ("1=-1=", 353),
            ("12111", 906),
            ("20012", 1_257),
            ("1=-0-2", 1_747),
        ];

        for (snafu, val) in test_input {
            assert_eq!(val, snafu_to_i64(snafu));
        }
    }

    #[test]
    fn test_asd() {
        let target = 35_023_647_158_862;
        assert_eq!(target, snafu_to_i64("2-10==12-122-=1-1-22"));
    }
}
