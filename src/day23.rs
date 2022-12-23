use aoc_runner_derive::{aoc, aoc_generator};
use itertools::iproduct;
use std::collections::HashMap;

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum Tile {
    Elf,
    Empty,
}

#[aoc_generator(day23)]
pub fn generate(inp: &str) -> Vec<Vec<Tile>> {
    inp.lines().fold(vec![], |mut acc, it| {
        let row = it.chars().fold(vec![], |mut acc, it| {
            acc.push(if it == '.' { Tile::Empty } else { Tile::Elf });
            acc
        });

        acc.push(row);
        acc
    })
}

#[derive(Copy, Clone, Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

fn consider_direction(round: usize) -> Vec<Direction> {
    let mut v = vec![
        Direction::North,
        Direction::South,
        Direction::West,
        Direction::East,
    ];
    v.rotate_left(round % 4);
    v
}

fn get_adjacent_row(direction: Direction, x: usize, y: usize, tiles: &[Vec<Tile>]) -> [Tile; 3] {
    match direction {
        Direction::North => [tiles[y - 1][x - 1], tiles[y - 1][x], tiles[y - 1][x + 1]],
        Direction::East => [tiles[y - 1][x + 1], tiles[y][x + 1], tiles[y + 1][x + 1]],
        Direction::South => [tiles[y + 1][x - 1], tiles[y + 1][x], tiles[y + 1][x + 1]],
        Direction::West => [tiles[y - 1][x - 1], tiles[y][x - 1], tiles[y + 1][x - 1]],
    }
}

fn moves(x: usize, y: usize, round: usize, tiles: &[Vec<Tile>]) -> Option<(usize, usize)> {
    let directions = vec![
        get_adjacent_row(Direction::North, x, y, tiles),
        get_adjacent_row(Direction::East, x, y, tiles),
        get_adjacent_row(Direction::South, x, y, tiles),
        get_adjacent_row(Direction::West, x, y, tiles),
    ];

    if directions.iter().flatten().all(|it| *it == Tile::Empty) {
        return None;
    }

    for dir in consider_direction(round) {
        match dir {
            Direction::North => {
                // N, NE, NW => N
                if directions[0].iter().all(|it| *it == Tile::Empty) {
                    return Some((y - 1, x));
                }
            }
            Direction::East => {
                // E, NE, SE => E
                if directions[1].iter().all(|it| *it == Tile::Empty) {
                    return Some((y, x + 1));
                }
            }
            Direction::South => {
                // S, SE, SW => S
                if directions[2].iter().all(|it| *it == Tile::Empty) {
                    return Some((y + 1, x));
                }
            }
            Direction::West => {
                // W, NW, SW => W
                if directions[3].iter().all(|it| *it == Tile::Empty) {
                    return Some((y, x - 1));
                }
            }
        }
    }

    None
}

#[allow(dead_code)]
fn print_map(tiles: &[Vec<Tile>]) {
    for row in tiles {
        for col in row {
            if *col == Tile::Empty {
                print!(".");
            } else {
                print!("#");
            }
        }

        println!();
    }

    println!();
}

fn grow_map(tiles: &mut Vec<Vec<Tile>>) {
    // extend map
    if tiles[0].iter().any(|it| *it == Tile::Elf) {
        tiles.insert(0, vec![Tile::Empty; tiles[0].len()]);
    }

    if tiles[tiles.len() - 1].iter().any(|it| *it == Tile::Elf) {
        tiles.push(vec![Tile::Empty; tiles[0].len()]);
    }

    if tiles.iter().any(|it| it[0] == Tile::Elf) {
        for row in tiles.iter_mut() {
            row.insert(0, Tile::Empty);
        }
    }

    if tiles.iter().any(|it| it[it.len() - 1] == Tile::Elf) {
        for row in tiles.iter_mut() {
            row.push(Tile::Empty);
        }
    }
}

fn collect_move_proposals(
    round: usize,
    tiles: &[Vec<Tile>],
    move_proposals: &mut HashMap<(usize, usize), Vec<(usize, usize)>>,
) {
    for (y, x) in iproduct!(0..tiles.len(), 0..tiles[0].len()) {
        if tiles[y][x] == Tile::Elf {
            if let Some((ny, nx)) = moves(x, y, round, tiles) {
                move_proposals.entry((ny, nx)).or_default().push((y, x));
            }
        }
    }
}

fn move_elves(
    tiles: &mut [Vec<Tile>],
    move_proposals: &HashMap<(usize, usize), Vec<(usize, usize)>>,
) -> bool {
    let mut any_moved = false;

    for (&(dest_y, dest_x), v) in move_proposals {
        if v.len() != 1 {
            continue;
        }

        let (from_y, from_x) = v[0];
        tiles[from_y][from_x] = Tile::Empty;
        tiles[dest_y][dest_x] = Tile::Elf;

        any_moved = true;
    }

    any_moved
}

#[aoc(day23, part1)]
pub fn part1(tiles: &[Vec<Tile>]) -> usize {
    let mut tiles = tiles.to_vec();
    let mut move_proposals: HashMap<(usize, usize), Vec<(usize, usize)>> = HashMap::new();

    for round in 0..10 {
        grow_map(&mut tiles);

        // First half: propose moves
        collect_move_proposals(round, &tiles, &mut move_proposals);

        // Second half: resolve
        let _ = move_elves(&mut tiles, &move_proposals);

        move_proposals.clear();
    }

    let (mut min_x, mut max_x, mut min_y, mut max_y) =
        (usize::MAX, usize::MIN, usize::MAX, usize::MIN);

    for (y, x) in iproduct!(0..tiles.len(), 0..tiles[0].len()) {
        if tiles[y][x] == Tile::Elf {
            min_x = min_x.min(x);
            max_x = max_x.max(x);

            min_y = min_y.min(y);
            max_y = max_y.max(y);
        }
    }

    let mut result = 0;
    for (y, x) in iproduct!(min_y..=max_y, min_x..=max_x) {
        if tiles[y][x] == Tile::Empty {
            result += 1;
        }
    }

    result
}

#[aoc(day23, part2)]
pub fn part2(tiles: &[Vec<Tile>]) -> usize {
    let mut tiles = tiles.to_vec();
    let mut move_proposals: HashMap<(usize, usize), Vec<(usize, usize)>> = HashMap::new();

    for round in 0.. {
        grow_map(&mut tiles);

        // First half: propose moves
        collect_move_proposals(round, &tiles, &mut move_proposals);

        // Second half: resolve
        if !move_elves(&mut tiles, &move_proposals) {
            return round + 1;
        }

        move_proposals.clear();
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "..............\n\
                              ..............\n\
                              .......#......\n\
                              .....###.#....\n\
                              ...#...#.#....\n\
                              ....#...##....\n\
                              ...#.###......\n\
                              ...##.#.##....\n\
                              ....#..#......\n\
                              ..............\n\
                              ..............\n\
                              ..............";

    #[test]
    fn test_sample_p1() {
        let data = generate(TEST_INPUT);
        let res = part1(&data);
        assert_eq!(res, 110);
    }

    #[test]
    fn test_sample_p2() {
        let data = generate(TEST_INPUT);
        let res = part2(&data);
        assert_eq!(res, 20);
    }
}
