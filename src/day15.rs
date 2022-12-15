use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use parse_display_derive::{Display, FromStr};
use std::collections::HashSet;

#[derive(Display, FromStr, Copy, PartialEq, Eq, Hash, Clone, Debug)]
#[display("Sensor at x={x}, y={y}: closest beacon is at x={bx}, y={by}")]
pub struct Sensor {
    x: isize,
    y: isize,
    bx: isize,
    by: isize,

    #[from_str(default)]
    distance: isize,
}

#[allow(clippy::cast_possible_wrap)]
fn manhattan_distance((lx, ly): (isize, isize), (rx, ry): (isize, isize)) -> isize {
    let dist = lx.abs_diff(rx) + ly.abs_diff(ry);
    dist as isize
}

#[aoc_generator(day15)]
pub fn generate(inp: &str) -> Vec<Sensor> {
    let mut sensors = inp
        .lines()
        .filter_map(|it| it.parse::<Sensor>().ok())
        .collect_vec();

    for sens in &mut sensors {
        sens.distance = manhattan_distance((sens.x, sens.y), (sens.bx, sens.by));
    }

    sensors
}

// None result = x,y is beacon or sensor
fn can_be_beacon(
    (x, y): (isize, isize),
    sensors: &[Sensor],
    taken_spots: &HashSet<(isize, isize)>,
) -> Option<bool> {
    if taken_spots.contains(&(x, y)) {
        return None;
    }

    // x,y can be a beacon, if its distance to all sensors is greater than the distance of that sensor to its beacon
    Some(sensors.iter().all(|it| {
        let dist_to_sensor = manhattan_distance((x, y), (it.x, it.y));
        dist_to_sensor > it.distance
    }))
}

fn get_taken_coords(sensors: &[Sensor]) -> HashSet<(isize, isize)> {
    sensors
        .iter()
        .flat_map(|it| vec![(it.x, it.y), (it.bx, it.by)])
        .collect()
}

#[allow(clippy::cast_possible_wrap)]
fn count_in_row(target_row: isize, sensors: &[Sensor]) -> Option<usize> {
    let min_x = sensors.iter().map(|it| it.x - it.distance).min()?;
    let max_x = sensors.iter().map(|it| it.x + it.distance).max()?;

    let range = (min_x..=max_x).collect_vec();

    let taken_spots = get_taken_coords(sensors);

    let result = range
        .iter()
        .filter(|it| can_be_beacon((**it, target_row), sensors, &taken_spots) == Some(false))
        .count();

    Some(result)
}

#[aoc(day15, part1)]
pub fn part1(sensors: &[Sensor]) -> Option<usize> {
    const TARGET_ROW: isize = 2_000_000;
    count_in_row(TARGET_ROW, sensors)
}

#[allow(clippy::cast_possible_wrap)]
fn find_beacon(bound: isize, sensors: &[Sensor]) -> Option<isize> {
    let taken_spots = get_taken_coords(sensors);

    sensors.iter().find_map(|it| {
        let x_range = (it.x - it.distance - 1).max(0)..=it.x.min(bound);
        let y_range = it.y..=bound;

        x_range.zip(y_range).find_map(|p| {
            if can_be_beacon(p, sensors, &taken_spots)? {
                Some(p.0 * 4_000_000 + p.1)
            } else {
                None
            }
        })
    })
}

#[aoc(day15, part2)]
pub fn part2(sensors: &[Sensor]) -> Option<isize> {
    find_beacon(4_000_000, sensors)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15\n\
                              Sensor at x=9, y=16: closest beacon is at x=10, y=16\n\
                              Sensor at x=13, y=2: closest beacon is at x=15, y=3\n\
                              Sensor at x=12, y=14: closest beacon is at x=10, y=16\n\
                              Sensor at x=10, y=20: closest beacon is at x=10, y=16\n\
                              Sensor at x=14, y=17: closest beacon is at x=10, y=16\n\
                              Sensor at x=8, y=7: closest beacon is at x=2, y=10\n\
                              Sensor at x=2, y=0: closest beacon is at x=2, y=10\n\
                              Sensor at x=0, y=11: closest beacon is at x=2, y=10\n\
                              Sensor at x=20, y=14: closest beacon is at x=25, y=17\n\
                              Sensor at x=17, y=20: closest beacon is at x=21, y=22\n\
                              Sensor at x=16, y=7: closest beacon is at x=15, y=3\n\
                              Sensor at x=14, y=3: closest beacon is at x=15, y=3\n\
                              Sensor at x=20, y=1: closest beacon is at x=15, y=3";

    #[test]
    fn test_sample_p1() {
        let data = generate(TEST_INPUT);
        let res = count_in_row(10, &data);
        assert_eq!(res.unwrap(), 26);
    }

    #[test]
    fn test_sample_p2() {
        let data = generate(TEST_INPUT);
        let res = find_beacon(20, &data);
        assert_eq!(res.unwrap(), 56_000_011);
    }
}
