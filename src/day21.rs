use aoc_runner_derive::{aoc, aoc_generator};
use parse_display::{Display, FromStr};
use xxcalc::calculator::Calculator;

#[derive(Display, FromStr, PartialEq, Eq, Hash, Clone, Debug)]
pub enum Operation {
    #[display("{0}")]
    Number(i64),

    #[display("{0} {1} {2}")]
    Calculation(String, char, String),
}

#[derive(Display, FromStr, Clone, Debug)]
#[display("{name}: {operation}")]
pub struct Monkey {
    name: String,
    operation: Operation,
}

impl Monkey {
    fn evaluate(&self, monkeys: &[Self]) -> i64 {
        match &self.operation {
            Operation::Number(n) => *n,
            Operation::Calculation(lhs, op, rhs) => {
                let lhs_val = monkeys
                    .iter()
                    .find(|it| it.name.eq(lhs))
                    .expect("Monkey has to exist")
                    .evaluate(monkeys);

                let rhs_val = monkeys
                    .iter()
                    .find(|it| it.name.eq(rhs))
                    .expect("Monkey has to exist")
                    .evaluate(monkeys);

                match op {
                    '+' => lhs_val + rhs_val,
                    '-' => lhs_val - rhs_val,
                    '*' => lhs_val * rhs_val,
                    '/' => lhs_val / rhs_val,
                    _ => unreachable!("Unknown operation"),
                }
            }
        }
    }
}

#[aoc_generator(day21)]
pub fn generate(inp: &str) -> Vec<Monkey> {
    inp.lines().filter_map(|it| it.parse().ok()).collect()
}

#[aoc(day21, part1)]
pub fn part1(monkeys: &[Monkey]) -> i64 {
    let root = monkeys
        .iter()
        .find(|it| it.name.eq("root"))
        .expect("Root has to exist");

    root.evaluate(monkeys)
}

fn generate_equation(monkey: &Monkey, monkeys: &[Monkey], result: &mut String) {
    if monkey.name.eq("humn") {
        result.push('X');
        return;
    }

    match &monkey.operation {
        Operation::Number(n) => result.push_str(&n.to_string()),
        Operation::Calculation(lhs, op, rhs) => {
            let lhs = monkeys
                .iter()
                .find(|it| it.name.eq(lhs))
                .expect("Monkey has to exist");
            let rhs = monkeys
                .iter()
                .find(|it| it.name.eq(rhs))
                .expect("Monkey has to exist");

            result.push('(');

            generate_equation(lhs, monkeys, result);

            if monkey.name.eq("root") {
                result.push('=');
            } else {
                result.push(*op);
            }

            generate_equation(rhs, monkeys, result);

            result.push(')');
        }
    };
}

#[aoc(day21, part2)]
pub fn part2(monkeys: &[Monkey]) -> Option<i64> {
    let mut result = String::new();

    let root = monkeys
        .iter()
        .find(|it| it.name.eq("root"))
        .expect("'root' has to exist");
    generate_equation(root, monkeys, &mut result);

    xxcalc::linear_solver::LinearSolver
        .process(&result)
        .ok()?
        .as_f64()
        .map(|it| it as i64)
        .ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "root: pppw + sjmn\n\
                              dbpl: 5\n\
                              cczh: sllz + lgvd\n\
                              zczc: 2\n\
                              ptdq: humn - dvpt\n\
                              dvpt: 3\n\
                              lfqf: 4\n\
                              humn: 5\n\
                              ljgn: 2\n\
                              sjmn: drzm * dbpl\n\
                              sllz: 4\n\
                              pppw: cczh / lfqf\n\
                              lgvd: ljgn * ptdq\n\
                              drzm: hmdt - zczc\n\
                              hmdt: 32";

    #[test]
    fn test_sample_p1() {
        let data = generate(TEST_INPUT);
        let res = part1(&data);
        assert_eq!(res, 152);
    }

    #[test]
    fn test_sample_p2() {
        let data = generate(TEST_INPUT);
        let res = part2(&data);
        assert_eq!(res, Some(301));
    }
}
