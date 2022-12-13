use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::cmp::Ordering;

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Node {
    Integer(i64),
    List(Vec<Node>),
}

fn parse_numbers_list(line: &str) -> Option<Node> {
    if line.contains('[') || line.contains(']') {
        return None;
    }

    Some(Node::List(
        line.split(',')
            .filter_map(|it| it.parse::<i64>().ok())
            .map(Node::Integer)
            .collect(),
    ))
}

fn parse_node(line: &str) -> Option<Node> {
    let line = line.strip_prefix('[')?.strip_suffix(']')?;
    if let Some(numbers_list) = parse_numbers_list(line) {
        return Some(numbers_list);
    }

    let mut data = vec![];

    let mut chrs = line.chars().collect_vec();
    while !chrs.is_empty() {
        if chrs[0] == ',' {
            chrs.drain(..1);
            continue;
        }

        if chrs[0] == '[' {
            let mut end_idx = 0;
            let mut stack = 0;
            for (idx, c) in chrs.iter().enumerate() {
                if *c == '[' {
                    stack += 1;
                } else if *c == ']' {
                    stack -= 1;
                    if stack == 0 {
                        end_idx = idx;
                        break;
                    }
                }
            }

            let sub_line = chrs[..=end_idx].iter().join("");
            data.push(parse_node(&sub_line)?);
            chrs.drain(..=end_idx);
        } else if chrs[0].is_ascii_digit() {
            let digits = chrs.iter().take_while(|it| it.is_ascii_digit()).join("");
            data.push(Node::Integer(digits.parse::<i64>().ok()?));
            chrs.drain(..digits.len());
        }
    }

    Some(Node::List(data))
}

#[aoc_generator(day13)]
pub fn generate(inp: &str) -> Vec<Node> {
    inp.lines().filter_map(parse_node).collect()
}

fn compare_lists(left: &[Node], right: &[Node]) -> Ordering {
    if left.is_empty() && !right.is_empty() {
        // If the left list runs out of items first, the inputs are in the right order.
        return Ordering::Less;
    } else if !left.is_empty() && right.is_empty() {
        // If the right list runs out of items first, the inputs are not in the right order.
        return Ordering::Greater;
    }

    for idx in 0..left.len() {
        if idx >= right.len() {
            return Ordering::Greater;
        }

        let order = compare_nodes(&left[idx], &right[idx]);
        if order != Ordering::Equal {
            return order;
        }
    }

    // All elements are equal AND left.len() <= right.len()
    left.len().cmp(&right.len())
}

fn compare_nodes(left: &Node, right: &Node) -> Ordering {
    match (left, right) {
        (Node::Integer(l), Node::Integer(r)) => l.cmp(r),
        (Node::List(ls), Node::List(rs)) => compare_lists(ls, rs),
        (Node::Integer(l), Node::List(_)) => {
            compare_nodes(&Node::List(vec![Node::Integer(*l)]), right)
        }
        (Node::List(_), Node::Integer(r)) => {
            compare_nodes(left, &Node::List(vec![Node::Integer(*r)]))
        }
    }
}

#[aoc(day13, part1)]
pub fn part1(lists: &[Node]) -> usize {
    let even_entry = lists.iter().step_by(2).cloned().collect_vec();
    let odd_entry = lists.iter().skip(1).step_by(2).cloned().collect_vec();

    assert_eq!(even_entry.len(), odd_entry.len());

    let list_pairs = even_entry.iter().zip(odd_entry.iter()).collect_vec();

    let mut result = 0;
    for (idx, (left, right)) in list_pairs.iter().enumerate() {
        if compare_nodes(left, right) == Ordering::Less {
            result += idx + 1;
        }
    }

    result
}

#[aoc(day13, part2)]
pub fn part2(lists: &[Node]) -> usize {
    let mut lists = lists.to_vec();
    // [[2]]
    let decoder_2 = Node::List(vec![Node::List(vec![Node::Integer(2)])]);
    lists.push(decoder_2.clone());

    // [[6]]
    let decoder_6 = Node::List(vec![Node::List(vec![Node::Integer(6)])]);
    lists.push(decoder_6.clone());

    lists.sort_by(compare_nodes);

    lists
        .iter()
        .enumerate()
        .filter_map(|(idx, it)| {
            if *it == decoder_2 || *it == decoder_6 {
                Some(idx + 1)
            } else {
                None
            }
        })
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_comparison() {
        let left = Node::List(vec![
            Node::List(vec![Node::Integer(1)]),
            Node::List(vec![Node::Integer(2), Node::Integer(3), Node::Integer(4)]),
        ]);

        let right = Node::List(vec![Node::List(vec![Node::Integer(1)]), Node::Integer(4)]);

        assert_eq!(compare_nodes(&left, &right), Ordering::Less);
    }

    #[test]
    fn test_comparison_2() {
        let left = Node::List(vec![
            Node::List(vec![Node::Integer(1), Node::Integer(2)]),
            Node::Integer(3),
        ]);

        let right = Node::List(vec![
            Node::List(vec![Node::Integer(1), Node::Integer(2)]),
            Node::Integer(2),
        ]);

        assert_eq!(compare_nodes(&left, &right), Ordering::Greater);
    }

    #[test]
    fn test_comparison_3() {
        let left = Node::List(vec![
            Node::List(vec![Node::Integer(4), Node::Integer(4)]),
            Node::Integer(4),
            Node::Integer(4),
        ]);
        let right = Node::List(vec![
            Node::List(vec![Node::Integer(4), Node::Integer(4)]),
            Node::Integer(4),
            Node::Integer(4),
            Node::Integer(4),
        ]);

        assert_eq!(compare_nodes(&left, &right), Ordering::Less);
    }

    #[test]
    fn test_comparison_4() {
        let left = Node::List(vec![
            Node::Integer(2),
            Node::List(vec![Node::Integer(1), Node::Integer(2)]),
            Node::Integer(2),
        ]);

        let right = Node::List(vec![
            Node::Integer(2),
            Node::List(vec![Node::Integer(1), Node::Integer(2)]),
            Node::Integer(3),
        ]);

        assert_eq!(compare_nodes(&left, &right), Ordering::Less);
    }

    #[test]
    fn test_sample_p1() {
        let inp = include_str!("../input/2022/day13_test.txt");
        let data = generate(inp);
        let res = part1(&data);
        assert_eq!(res, 13);
    }

    #[test]
    fn test_sample_p2() {
        let inp = include_str!("../input/2022/day13_test.txt");
        let data = generate(inp);
        let res = part2(&data);
        assert_eq!(res, 140);
    }
}
