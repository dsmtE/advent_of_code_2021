use aoc_utils::{cartesian::{p2, v2, Cartesian2, Point2, Vector2}, nom_parsing::number};
use nom::{bytes::complete::tag, character::complete::newline, combinator::map, multi::separated_list1, sequence::{preceded, separated_pair, tuple}, IResult, Parser};

use std::cmp::Ordering::*;

advent_of_code::solution!(14);

type Robot = (Point2, Vector2);

pub fn parse_comma_coordinate(input: &str) -> IResult<&str, (i64, i64)> {
    separated_pair(number, tag(","), number)(input)
}

pub fn parse_robot(input: &str) -> IResult<&str, Robot> {
    tuple((
        map(preceded(tag("p="), parse_comma_coordinate), |(x, y)| p2(x, y)),
        map(preceded(tag(" v="), parse_comma_coordinate), |(x, y)| v2(x, y)),
    ))(input)
}

pub fn parse_input(input: &str) -> IResult<&str, Vec<Robot>> {
    separated_list1(newline, parse_robot)(input)
}

fn final_pos(pos: Point2, dir: Vector2, steps: i64, grid_size: &Point2) -> Point2 {
    let mut final_pos = pos + (dir * steps);
    *final_pos.x_ref_mut() = final_pos.x().rem_euclid(grid_size.x());
    *final_pos.y_ref_mut() = final_pos.y().rem_euclid(grid_size.y());
    final_pos
}

fn quadran_count(robots_positions: &[Point2], grid_size: &Point2) -> [u32; 4] {
    let mut quadrants_count = [0; 4];

    for pos in robots_positions {
        match (pos.x().cmp(&((grid_size.x()-1)/2)), pos.y().cmp(&((grid_size.y()-1)/2))) {
            (Less, Less) => quadrants_count[0] += 1,
            (Greater, Less) => quadrants_count[1] += 1,
            (Less, Greater) => quadrants_count[2] += 1,
            (Greater, Greater) => quadrants_count[3] += 1,
            _ => (),
        }
    }

    quadrants_count
}

pub fn part_one(input: &str) -> Option<u32> {
    let robots = parse_input(input).unwrap().1;
    let grid_size = p2(101, 103);

    let robots_final_pos = robots.iter().map(|(pos, dir)| final_pos(*pos, *dir, 100, &grid_size)).collect::<Vec<_>>();
    let quadrants_count = quadran_count(&robots_final_pos, &grid_size);

    Some(quadrants_count.iter().product())
}

pub fn part_two(input: &str) -> Option<u32> {
    todo!()
}


#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";

    #[test]
    fn test_parse_input() {

        assert_eq!(parse_comma_coordinate("0,4"), Ok(("", (0, 4))));
        assert_eq!(parse_comma_coordinate("2,-3"), Ok(("", (2, -3))));

        assert_eq!(parse_robot("p=0,4 v=3,-3"), Ok(("", (p2(0, 4), v2(3, -3)))));
        
        let robots = parse_input(TEST_INPUT).unwrap().1;

        assert_eq!(robots, vec![
            (p2(0, 4), v2(3, -3)),
            (p2(6, 3), v2(-1, -3)),
            (p2(10, 3), v2(-1, 2)),
            (p2(2, 0), v2(2, -1)),
            (p2(0, 0), v2(1, 3)),
            (p2(3, 0), v2(-2, -2)),
            (p2(7, 6), v2(-1, -3)),
            (p2(3, 0), v2(-1, -2)),
            (p2(9, 3), v2(2, 3)),
            (p2(7, 3), v2(-1, 2)),
            (p2(2, 4), v2(2, -3)),
            (p2(9, 5), v2(-3, -3)),
        ]);
    }

    #[test]
    fn test_part_one() {
        let robots = parse_input(TEST_INPUT).unwrap().1;
        let grid_size = p2(11, 7);

        let robots_final_pos = robots.iter().map(|(pos, dir)| final_pos(*pos, *dir, 100, &grid_size)).collect::<Vec<_>>();
        let quadrants_count = quadran_count(&robots_final_pos, &grid_size);

        assert_eq!(quadrants_count, [1, 3, 4, 1]);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(TEST_INPUT);
        assert_eq!(result, None);
    }
}