use std::collections::HashSet;
use aoc_utils::cartesian::{p2, Point2, Direction, Cartesian2};

advent_of_code::solution!(6);

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Robot {
    pub pos: Point2,
    pub direction: Direction,
}
impl Robot {
    fn new(pos: Point2, direction: Direction) -> Self {
        Self { pos, direction }
    }

    fn next_pos(&self) -> Point2 {
        self.pos + self.direction.vector()
    }
}

fn point_in_bounds(point: &Point2, size: &Point2) -> bool {
    point.x() >= 0 && point.y() >= 0 && point.x() < size.x() && point.y() < size.y()
}

fn parse_input(input: &str) -> (Robot, HashSet<Point2>, Point2) {
    let mut obstacles = HashSet::new();
    let size = p2(
        input.lines().next().unwrap().len() as _,
        input.lines().count() as _
    );
    let mut robot = Robot::new(p2(0, 0), Direction::Up);
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let pos = p2(x as _ ,y as _);
            if c == '^' {
                robot.pos = pos;
            } else if c == '#' {
                obstacles.insert(pos);
            }
        }
    }

    (robot, obstacles, size)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut robot, obstacles, grid_size) = parse_input(input);
    let mut visited_positions = HashSet::new();
    let move_result = process_move(&mut robot, &obstacles, &grid_size, 
        |pos| {
            visited_positions.insert(pos);
        });

    match move_result {
        MoveResult::LoopDetected => panic!("Loop detected"),
        MoveResult::OutOfBounds => Some(visited_positions.len() as _),
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[derive(Debug, PartialEq, Eq)]
enum MoveResult {
    OutOfBounds,
    LoopDetected,
}

fn process_move(robot: &mut Robot, obstacles: &HashSet<Point2>, size: &Point2, mut pos_callback: impl FnMut(Point2)) -> MoveResult {
    let mut visited_robot_state: HashSet<Robot> = HashSet::new();
    
    visited_robot_state.insert(*robot);
    pos_callback(robot.pos);
    loop {
        let next_pos = robot.next_pos();

        if !point_in_bounds(&next_pos, size) {
            return MoveResult::OutOfBounds;
        }
        else if  visited_robot_state.contains(&Robot { pos: next_pos, direction: robot.direction }) {
            return MoveResult::LoopDetected;
        }
        else if obstacles.contains(&next_pos) {
            robot.direction = robot.direction.turn_right();
        }
        else
        {
            robot.pos = next_pos;
        }

        // println!("{:?}", robot);
        pos_callback(robot.pos);
        visited_robot_state.insert(*robot);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn test_parse_input() {
        let (robot, obstacles, grid_size) = parse_input(TEST_INPUT);
        assert_eq!(robot, Robot::new(p2(4, 6), Direction::Up));
        assert_eq!(grid_size, p2(10, 10));
        assert_eq!(obstacles, vec![
            p2(4, 0), p2(9, 1), p2(2, 3), p2(7, 4), p2(1, 6), p2(8, 7), p2(0, 8), p2(6, 9)
        ].into_iter().collect());
    }

    #[test]
    fn test_part_one() {
        let (robot, obstacles, grid_size) = parse_input(TEST_INPUT);
        
        let mut visited_positions = HashSet::new();
        let move_result = process_move(&mut robot.clone(), &obstacles, &grid_size, 
            |pos| {
                visited_positions.insert(pos);
            });

        assert_eq!(move_result, MoveResult::OutOfBounds);
        assert_eq!(visited_positions.len(), 41);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(TEST_INPUT);
        assert_eq!(result, None);
    }
}