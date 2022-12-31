use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;
use std::ops::AddAssign;

#[derive(Copy, Clone, Debug)]
pub enum Direction {
    Up,

    Down,

    Left,

    Right,
}

impl Direction {
    fn evaluate(self, knots: &mut [Pos]) {
        match self {
            Self::Up => {
                knots[0].row -= 1;
            }
            Self::Down => {
                knots[0].row += 1;
            }
            Self::Left => {
                knots[0].col -= 1;
            }
            Self::Right => {
                knots[0].col += 1;
            }
        }
    }
}

#[aoc_generator(day9)]
pub fn generate(inp: &str) -> Vec<Direction> {
    inp.lines()
        .flat_map(|it| {
            let dir = it.chars().next().expect("Invalid input");
            let steps = it[2..].parse::<usize>().expect("Invalid input");

            let dir = match dir {
                'U' => Direction::Up,
                'D' => Direction::Down,
                'L' => Direction::Left,
                'R' => Direction::Right,
                _ => unreachable!("Unhandled"),
            };

            std::iter::repeat(dir).take(steps)
        })
        .collect()
}

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
struct Pos {
    row: i32,
    col: i32,
}

impl Pos {
    const fn is_neighbour_of(self, other: Self) -> bool {
        let dx = self.col.abs_diff(other.col);
        let dy = self.row.abs_diff(other.row);

        dx <= 1 && dy <= 1
    }
}

impl AddAssign for Pos {
    fn add_assign(&mut self, rhs: Self) {
        self.row += rhs.row;
        self.col += rhs.col;
    }
}

impl Pos {
    fn follow_direction(self, rhs: Self) -> Self {
        Self {
            row: self.row.cmp(&rhs.row) as i32,
            col: self.col.cmp(&rhs.col) as i32,
        }
    }
}

fn update_tail_knots(knots: &mut [Pos], tail_seen: &mut HashSet<Pos>) {
    for knot_idx in 1..knots.len() {
        let prev_knot = &knots[knot_idx - 1];
        if knots[knot_idx].is_neighbour_of(*prev_knot) {
            return;
        }

        let diff = prev_knot.follow_direction(knots[knot_idx]);
        knots[knot_idx] += diff;
    }

    tail_seen.insert(knots[knots.len() - 1]);
}

fn simulate_movement(dirs: &[Direction], num_knots: usize) -> HashSet<Pos> {
    let mut tail_seen = HashSet::new();
    tail_seen.insert(Pos { row: 0, col: 0 });

    let mut knots = vec![Pos { row: 0, col: 0 }; num_knots];

    for dir in dirs {
        dir.evaluate(&mut knots);
        update_tail_knots(&mut knots, &mut tail_seen);
    }

    tail_seen
}

#[aoc(day9, part1)]
pub fn part1(dirs: &[Direction]) -> usize {
    simulate_movement(dirs, 2).len()
}

#[aoc(day9, part2)]
pub fn part2(dirs: &[Direction]) -> usize {
    simulate_movement(dirs, 10).len()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_P1: &str = "R 4\n\
                                 U 4\n\
                                 L 3\n\
                                 D 1\n\
                                 R 4\n\
                                 D 1\n\
                                 L 5\n\
                                 R 2";

    #[test]
    fn test_sample_p1() {
        let data = generate(TEST_INPUT_P1);
        let res = part1(&data);
        assert_eq!(res, 13);
    }

    const TEST_INPUT_P2: &str = "R 5\n\
                                 U 8\n\
                                 L 8\n\
                                 D 3\n\
                                 R 17\n\
                                 D 10\n\
                                 L 25\n\
                                 U 20";

    #[test]
    fn test_sample_p2() {
        let data = generate(TEST_INPUT_P2);
        let res = part2(&data);
        assert_eq!(res, 36);
    }

    #[test]
    fn asd() {
        let data = generate(TEST_INPUT_P1);
        part2(&data);
    }
}
