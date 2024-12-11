advent_of_code::solution!(8);

use std::collections::{HashMap, HashSet};

use aoc_utils::cartesian::{p2, Cartesian2, Point2};
use itertools::Itertools;

pub fn parse_input(input: &str) -> (HashMap<char, Vec<Point2>>, Point2) {
    let mut antennas_positions = HashMap::<char, Vec<_>>::new();
    let size = p2(
        input.lines().next().unwrap().len() as _,
        input.lines().count() as _
    );

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let pos = p2(x as _ ,y as _);
            if c.is_alphanumeric() {
                antennas_positions.entry(c).or_default().push(pos);
            }
        }
    }

    (antennas_positions, size)
}

fn point_in_bounds(point: &Point2, size: &Point2) -> bool {
    point.x() >= 0 && point.y() >= 0 && point.x() < size.x() && point.y() < size.y()
}

pub fn part_one(input: &str) -> Option<u32> {
    let (antennas_positions, size) = parse_input(input);

    let mut antinodes_positions: HashSet<Point2> = HashSet::new();

    for (_, positions) in antennas_positions.iter() {
        positions.iter().permutations(2).for_each(|antenna_pair| {
            let a = antenna_pair[0];
            let b = antenna_pair[1];
            let antinode_possible_pos = *a + (*a - *b);
            if point_in_bounds(&antinode_possible_pos, &size) {
                antinodes_positions.insert(antinode_possible_pos);
            }
        });
    }

    Some(antinodes_positions.len() as _)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (antennas_positions, size) = parse_input(input);

    let mut antinodes_positions: HashSet<Point2> = HashSet::new();

    for (_, positions) in antennas_positions.iter() {
        positions.iter().permutations(2).for_each(|antenna_pair| {
            let a = antenna_pair[0];
            let b = antenna_pair[1];
            let distance = *a - *b;
            let mut antinode_possible_pos = *a;
            while point_in_bounds(&antinode_possible_pos, &size) {
                antinodes_positions.insert(antinode_possible_pos);
                antinode_possible_pos += distance;
            }
        });
    }

    Some(antinodes_positions.len() as _)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    #[test]
    fn parse_input_test() {
        let (antennas_positions, size) = parse_input(TEST_INPUT);
        assert_eq!(size, p2(12, 12));
        assert_eq!(antennas_positions, {
            HashMap::from_iter(
                [
                    ('0', vec![p2(8, 1), p2(5, 2), p2(7, 3), p2(4, 4)]),
                    ('A', vec![p2(6, 5), p2(8, 8), p2(9, 9)]),
                ]
            )
        });
    }

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(TEST_INPUT), Some(14));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(TEST_INPUT), Some(34));
    }
}