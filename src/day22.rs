use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use num_complex::Complex;
use parse_display::{Display, FromStr};
use std::collections::HashMap;

const UP: Complex<i64> = Complex::new(0, -1);
const RIGHT: Complex<i64> = Complex::new(1, 0);
const DOWN: Complex<i64> = Complex::new(0, 1);
const LEFT: Complex<i64> = Complex::new(-1, 0);

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub enum Tile {
    Empty,
    Wall,
}

#[derive(Display, FromStr, PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub enum Instruction {
    #[display("{0}")]
    Move(usize),

    #[display("R")]
    Right,

    #[display("L")]
    Left,
}

pub struct Input {
    grid: HashMap<Complex<i64>, Tile>,
    movements: Vec<Instruction>,
    size: i64,
}

fn parse_movements(txt: &str) -> Vec<Instruction> {
    txt.split_inclusive(char::is_alphabetic)
        .flat_map(|it| {
            let num = it
                .trim()
                .chars()
                .take_while(|it| it.is_numeric())
                .join("")
                .parse::<usize>()
                .ok();

            let mut result = vec![];

            if let Some(num) = num {
                result.push(Instruction::Move(num));
            }

            let direction = it
                .trim()
                .chars()
                .skip_while(|it| it.is_numeric())
                .join("")
                .parse::<Instruction>()
                .ok();
            if let Some(direction) = direction {
                result.push(direction);
            }

            result
        })
        .collect()
}

#[aoc_generator(day22)]
pub fn generate(inp: &str) -> Option<Input> {
    let (grid, pw) = inp.split_once("\n\n")?;

    let mut grid_map = HashMap::new();

    for (row, line) in grid.lines().filter(|it| !it.is_empty()).enumerate() {
        for (col, chr) in line.chars().enumerate() {
            if !chr.is_whitespace() {
                assert!(chr == '.' || chr == '#');

                grid_map.insert(
                    #[allow(clippy::cast_possible_wrap)]
                    Complex::new(col as i64, row as i64),
                    if chr == '.' { Tile::Empty } else { Tile::Wall },
                );
            }
        }
    }

    let movements = parse_movements(pw);

    Some(Input {
        grid: grid_map,
        movements,
        size: 50,
    })
}

#[derive(Debug)]
struct Player {
    direction: Complex<i64>,
    position: Complex<i64>,
}

impl Player {
    fn turn_left(&mut self) {
        self.direction *= -Complex::i();
    }

    fn turn_right(&mut self) {
        self.direction *= Complex::i();
    }
}

fn get_start_pos(grid: &HashMap<Complex<i64>, Tile>) -> Complex<i64> {
    grid.iter()
        .filter(|(it, _)| it.im == 0)
        .sorted_by_key(|(it, _)| it.re)
        .filter(|(_, it)| **it == Tile::Empty)
        .map(|(it, _)| it)
        .next()
        .copied()
        .expect("Empty input?")
}

#[allow(clippy::too_many_lines)]
#[aoc(day22, part1)]
pub fn part1(inp: &Input) -> Option<i64> {
    let mut player = Player {
        direction: RIGHT,
        position: get_start_pos(&inp.grid),
    };

    for inst in &inp.movements {
        assert_eq!(inp.grid.get(&player.position), Some(&Tile::Empty));

        match *inst {
            Instruction::Move(n) => {
                for _ in 0..n {
                    let new_pos = player.position + player.direction;

                    match inp.grid.get(&new_pos) {
                        None => {
                            // wrap
                            if player.direction == RIGHT {
                                let min_re_in_row = inp
                                    .grid
                                    .keys()
                                    .filter(|it| it.im == player.position.im)
                                    .map(|it| it.re)
                                    .min()?;
                                if inp
                                    .grid
                                    .get(&Complex::new(min_re_in_row, player.position.im))
                                    == Some(&Tile::Empty)
                                {
                                    player.position.re = min_re_in_row;
                                }
                            } else if player.direction == LEFT {
                                let max_re_in_row = inp
                                    .grid
                                    .keys()
                                    .filter(|it| it.im == player.position.im)
                                    .map(|it| it.re)
                                    .max()?;
                                if inp
                                    .grid
                                    .get(&Complex::new(max_re_in_row, player.position.im))
                                    == Some(&Tile::Empty)
                                {
                                    player.position.re = max_re_in_row;
                                }
                            } else if player.direction == UP {
                                let min_im_in_col = inp
                                    .grid
                                    .keys()
                                    .filter(|it| it.re == player.position.re)
                                    .map(|it| it.im)
                                    .max()?;
                                if inp
                                    .grid
                                    .get(&Complex::new(player.position.re, min_im_in_col))
                                    == Some(&Tile::Empty)
                                {
                                    player.position.im = min_im_in_col;
                                }
                            } else {
                                assert_eq!(player.direction, DOWN);

                                let max_im_in_col = inp
                                    .grid
                                    .keys()
                                    .filter(|it| it.re == player.position.re)
                                    .map(|it| it.im)
                                    .min()?;

                                if inp
                                    .grid
                                    .get(&Complex::new(player.position.re, max_im_in_col))
                                    == Some(&Tile::Empty)
                                {
                                    player.position.im = max_im_in_col;
                                }
                            }
                        }
                        Some(Tile::Wall) => {
                            break;
                        }
                        Some(Tile::Empty) => {
                            player.position = new_pos;
                        }
                    }
                }
            }
            Instruction::Right => {
                player.turn_right();
            }
            Instruction::Left => {
                player.turn_left();
            }
        }
    }
    assert_eq!(inp.grid.get(&player.position), Some(&Tile::Empty));

    let mut result = 1000 * (player.position.im + 1) + 4 * (player.position.re + 1);
    result += match player.direction {
        RIGHT => 0,
        DOWN => 1,
        LEFT => 2,
        UP => 3,
        _ => unreachable!(),
    };

    Some(result)
}

fn wrap_around_cube(player: &Player, size: i64) -> (Complex<i64>, Complex<i64>) {
    if player.direction == RIGHT {
        return if player.position.im < size {
            // Wrap right from quadrant 2
            (
                Complex::new(size * 2 - 1, size * 3 - 1 - player.position.im),
                LEFT,
            )
        } else if player.position.im >= size && player.position.im < size * 2 {
            // Wrap right from quadrant 3
            (
                Complex::new(size * 2 + player.position.im - size, size - 1),
                UP,
            )
        } else if player.position.im >= size * 2 && player.position.im < size * 3 {
            // Wrap right from quadrant 4
            (
                Complex::new(size * 3 - 1, size * 3 - 1 - player.position.im),
                LEFT,
            )
        } else if player.position.im >= size * 3 {
            // Wrap right from quadrant 6
            (
                Complex::new(player.position.im - size * 3 + size, size * 3 - 1),
                UP,
            )
        } else {
            unreachable!()
        };
    } else if player.direction == LEFT {
        return if player.position.im < size {
            // Wrap left from quadrant 1
            (
                Complex::new(0, size * 2 - 1 + size - player.position.im),
                RIGHT,
            )
        } else if player.position.im >= size && player.position.im < size * 2 {
            // Wrap left from quadrant 3
            (Complex::new(player.position.im - size, size * 2), DOWN)
        } else if player.position.im >= size * 2 && player.position.im < size * 3 {
            // Wrap left from quadrant 5
            (Complex::new(size, size * 3 - 1 - player.position.im), RIGHT)
        } else if player.position.im >= size * 3 {
            // Wrap left from quadrant 6
            (Complex::new(player.position.im - size * 2, 0), DOWN)
        } else {
            unreachable!()
        };
    } else if player.direction == UP {
        return if player.position.re < size {
            // Wrap up from quadrant 5
            (Complex::new(size, size + player.position.re), RIGHT)
        } else if player.position.re >= size && player.position.re < size * 2 {
            // Wrap up from quadrant 1
            (Complex::new(0, size * 3 + player.position.re - size), RIGHT)
        } else if player.position.re >= size * 2 && player.position.re < size * 3 {
            // Wrap up from quadrant 2
            (
                Complex::new(player.position.re - size * 2, size * 4 - 1),
                UP,
            )
        } else {
            unreachable!()
        };
    }

    assert_eq!(player.direction, DOWN);

    if player.position.re < size {
        // Wrap down from quadrant 6
        (Complex::new(size * 2 + player.position.re, 0), DOWN)
    } else if player.position.re >= size && player.position.re < size * 2 {
        // Wrap down from quadrant 4
        (
            Complex::new(size - 1, size * 3 + (player.position.re - size)),
            LEFT,
        )
    } else if player.position.re >= size * 2 && player.position.re < size * 3 {
        // Wrap down from quadrant 2
        (
            Complex::new(size * 2 - 1, size + (player.position.re - size * 2)),
            LEFT,
        )
    } else {
        unreachable!()
    }
}

#[allow(clippy::too_many_lines)]
#[aoc(day22, part2)]
pub fn part2(inp: &Input) -> i64 {
    let mut player = Player {
        direction: RIGHT,
        position: get_start_pos(&inp.grid),
    };

    for inst in &inp.movements {
        assert_eq!(inp.grid.get(&player.position), Some(&Tile::Empty));

        match *inst {
            Instruction::Move(n) => {
                for _ in 0..n {
                    let new_pos = player.position + player.direction;

                    match inp.grid.get(&new_pos) {
                        None => {
                            let (new_pos, new_dir) = wrap_around_cube(&player, inp.size);
                            assert!(inp.grid.contains_key(&new_pos), "{new_pos:?} not in map!");
                            if inp.grid.get(&new_pos) == Some(&Tile::Empty) {
                                player.position = new_pos;
                                player.direction = new_dir;
                            }
                        }
                        Some(Tile::Wall) => {
                            break;
                        }
                        Some(Tile::Empty) => {
                            player.position = new_pos;
                        }
                    }
                }
            }
            Instruction::Right => {
                player.turn_right();
            }
            Instruction::Left => {
                player.turn_left();
            }
        }
    }

    assert_eq!(inp.grid.get(&player.position), Some(&Tile::Empty));

    let mut result = 1000 * (player.position.im + 1) + 4 * (player.position.re + 1);
    result += match player.direction {
        RIGHT => 0,
        DOWN => 1,
        LEFT => 2,
        UP => 3,
        _ => unreachable!(),
    };

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r"
        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5";

    #[test]
    fn test_sample_p1() {
        let data = generate(TEST_INPUT).expect("Failed generating test input");
        let res = part1(&data);
        assert_eq!(res, Some(6032));
    }

    type TestTuple = ((Complex<i64>, Complex<i64>), (Complex<i64>, Complex<i64>));

    fn do_test(data: &[TestTuple]) {
        for &((pos, dir), (pos_e, dir_e)) in data {
            let player = Player {
                position: pos,
                direction: dir,
            };

            let (pr, dr) = wrap_around_cube(&player, 4);
            assert_eq!(pr, pos_e, "Move {dir:?} from {pos:?}");
            assert_eq!(dr, dir_e);
        }
    }

    #[test]
    fn wrap_from_1() {
        let test_data = vec![
            ((Complex::new(4, 0), LEFT), (Complex::new(0, 11), RIGHT)),
            ((Complex::new(4, 1), LEFT), (Complex::new(0, 10), RIGHT)),
            ((Complex::new(4, 2), LEFT), (Complex::new(0, 9), RIGHT)),
            ((Complex::new(4, 3), LEFT), (Complex::new(0, 8), RIGHT)),
            ((Complex::new(4, 0), UP), (Complex::new(0, 12), RIGHT)),
            ((Complex::new(5, 0), UP), (Complex::new(0, 13), RIGHT)),
            ((Complex::new(6, 0), UP), (Complex::new(0, 14), RIGHT)),
            ((Complex::new(7, 0), UP), (Complex::new(0, 15), RIGHT)),
        ];

        do_test(&test_data);
    }

    #[test]
    fn wrap_from_2() {
        let test_data = vec![
            ((Complex::new(8, 0), UP), (Complex::new(0, 15), UP)),
            ((Complex::new(9, 0), UP), (Complex::new(1, 15), UP)),
            ((Complex::new(10, 0), UP), (Complex::new(2, 15), UP)),
            ((Complex::new(11, 0), UP), (Complex::new(3, 15), UP)),
            ((Complex::new(11, 0), RIGHT), (Complex::new(7, 11), LEFT)),
            ((Complex::new(11, 1), RIGHT), (Complex::new(7, 10), LEFT)),
            ((Complex::new(11, 2), RIGHT), (Complex::new(7, 9), LEFT)),
            ((Complex::new(11, 3), RIGHT), (Complex::new(7, 8), LEFT)),
            ((Complex::new(8, 3), DOWN), (Complex::new(7, 4), LEFT)),
            ((Complex::new(9, 3), DOWN), (Complex::new(7, 5), LEFT)),
            ((Complex::new(10, 3), DOWN), (Complex::new(7, 6), LEFT)),
            ((Complex::new(11, 3), DOWN), (Complex::new(7, 7), LEFT)),
        ];

        do_test(&test_data);
    }

    #[test]
    fn wrap_from_3() {
        let test_data = vec![
            ((Complex::new(4, 4), LEFT), (Complex::new(0, 8), DOWN)),
            ((Complex::new(4, 5), LEFT), (Complex::new(1, 8), DOWN)),
            ((Complex::new(4, 6), LEFT), (Complex::new(2, 8), DOWN)),
            ((Complex::new(4, 7), LEFT), (Complex::new(3, 8), DOWN)),
            ((Complex::new(4, 4), RIGHT), (Complex::new(8, 3), UP)),
            ((Complex::new(4, 5), RIGHT), (Complex::new(9, 3), UP)),
            ((Complex::new(4, 6), RIGHT), (Complex::new(10, 3), UP)),
            ((Complex::new(4, 7), RIGHT), (Complex::new(11, 3), UP)),
        ];

        do_test(&test_data);
    }

    #[test]
    fn wrap_from_4() {
        let test_data = vec![
            ((Complex::new(7, 8), RIGHT), (Complex::new(11, 3), LEFT)),
            ((Complex::new(7, 9), RIGHT), (Complex::new(11, 2), LEFT)),
            ((Complex::new(7, 10), RIGHT), (Complex::new(11, 1), LEFT)),
            ((Complex::new(7, 11), RIGHT), (Complex::new(11, 0), LEFT)),
            ((Complex::new(4, 11), DOWN), (Complex::new(3, 12), LEFT)),
            ((Complex::new(5, 11), DOWN), (Complex::new(3, 13), LEFT)),
            ((Complex::new(6, 11), DOWN), (Complex::new(3, 14), LEFT)),
            ((Complex::new(7, 11), DOWN), (Complex::new(3, 15), LEFT)),
        ];

        do_test(&test_data);
    }

    #[test]
    fn wrap_from_5() {
        let test_data = vec![
            ((Complex::new(0, 8), UP), (Complex::new(4, 4), RIGHT)),
            ((Complex::new(1, 8), UP), (Complex::new(4, 5), RIGHT)),
            ((Complex::new(2, 8), UP), (Complex::new(4, 6), RIGHT)),
            ((Complex::new(3, 8), UP), (Complex::new(4, 7), RIGHT)),
            ((Complex::new(0, 8), LEFT), (Complex::new(4, 3), RIGHT)),
            ((Complex::new(0, 9), LEFT), (Complex::new(4, 2), RIGHT)),
            ((Complex::new(0, 10), LEFT), (Complex::new(4, 1), RIGHT)),
            ((Complex::new(0, 11), LEFT), (Complex::new(4, 0), RIGHT)),
        ];

        do_test(&test_data);
    }

    #[test]
    fn wrap_from_6() {
        let test_data = vec![
            ((Complex::new(0, 12), LEFT), (Complex::new(4, 0), DOWN)),
            ((Complex::new(0, 13), LEFT), (Complex::new(5, 0), DOWN)),
            ((Complex::new(0, 14), LEFT), (Complex::new(6, 0), DOWN)),
            ((Complex::new(0, 15), LEFT), (Complex::new(7, 0), DOWN)),
            ((Complex::new(3, 12), RIGHT), (Complex::new(4, 11), UP)),
            ((Complex::new(3, 13), RIGHT), (Complex::new(5, 11), UP)),
            ((Complex::new(3, 14), RIGHT), (Complex::new(6, 11), UP)),
            ((Complex::new(3, 15), RIGHT), (Complex::new(7, 11), UP)),
            ((Complex::new(0, 15), DOWN), (Complex::new(8, 0), DOWN)),
            ((Complex::new(1, 15), DOWN), (Complex::new(9, 0), DOWN)),
            ((Complex::new(2, 15), DOWN), (Complex::new(10, 0), DOWN)),
            ((Complex::new(3, 15), DOWN), (Complex::new(11, 0), DOWN)),
        ];

        do_test(&test_data);
    }
}
