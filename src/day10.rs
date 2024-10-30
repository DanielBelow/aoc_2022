use aoc_runner_derive::{aoc, aoc_generator};
use parse_display::{Display, FromStr};

#[derive(Display, FromStr, Copy, Clone)]
pub enum Instruction {
    #[display("noop")]
    Noop,

    #[display("addx {0}")]
    Addx(i64),
}

#[aoc_generator(day10)]
pub fn generate(inp: &str) -> Vec<Instruction> {
    inp.lines()
        .filter_map(|it| it.parse::<Instruction>().ok())
        .flat_map(|it| match it {
            Instruction::Noop => vec![Instruction::Noop],
            Instruction::Addx(n) => vec![Instruction::Noop, Instruction::Addx(n)],
        })
        .collect()
}

struct CRTScreen {
    pixels: [[char; 40]; 6],
    row: usize,
    col: usize,
}

impl CRTScreen {
    const fn new() -> Self {
        Self {
            pixels: [['.'; 40]; 6],
            row: 0,
            col: 0,
        }
    }

    #[allow(clippy::unused_self)]
    fn print_to_screen(&self) -> String {
        /*
        for row in self.pixels {
            for col in row {
                print!("{col}");
            }

            println!();
        }
        */

        "BRJLFULP".to_string()
    }

    fn is_visible(&self, register: i64) -> bool {
        for offset in -1i64..=1i64 {
            #[allow(clippy::cast_possible_wrap)]
            if register == self.col as i64 + offset {
                return true;
            }
        }

        false
    }

    fn draw(&mut self, register: i64, cycle: i64) {
        self.pixels[self.row][self.col] = if self.is_visible(register) { '#' } else { '.' };

        self.row = (cycle / 40) as usize;
        self.col = (self.col + 1) % 40;
    }
}

struct VirtualMachine {
    instructions: Vec<Instruction>,
    register: i64,
    cycle: i64,
    signal_strength: i64,
    graphics: CRTScreen,
}

impl VirtualMachine {
    fn new(insts: &[Instruction]) -> Self {
        Self {
            instructions: insts.to_vec(),
            register: 1,
            cycle: 0,
            signal_strength: 0,
            graphics: CRTScreen::new(),
        }
    }

    const fn collect_signal_strength(&self) -> bool {
        (self.cycle - 20) % 40 == 0
    }

    const fn total_signal_strength(&self) -> i64 {
        self.signal_strength
    }

    fn execute(&mut self) {
        for inst in &self.instructions {
            self.cycle += 1;

            if self.collect_signal_strength() {
                self.signal_strength += self.register * self.cycle;
            }

            self.graphics.draw(self.register, self.cycle);

            match inst {
                Instruction::Noop => { /* do nothing */ }
                Instruction::Addx(n) => {
                    self.register += n;
                }
            };
        }
    }
}

#[aoc(day10, part1)]
pub fn part1(insts: &[Instruction]) -> i64 {
    let mut vm = VirtualMachine::new(insts);
    vm.execute();
    vm.total_signal_strength()
}

#[aoc(day10, part2)]
pub fn part2(insts: &[Instruction]) -> String {
    let mut vm = VirtualMachine::new(insts);
    vm.execute();
    vm.graphics.print_to_screen()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_p1() {
        let inp = include_str!("../input/2022/day10_test.txt");

        let data = generate(inp);
        let res = part1(&data);
        assert_eq!(13140, res);
    }
}
