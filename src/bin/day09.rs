use std::{collections::HashSet, fmt::Debug};

use nom::{
    IResult,
    bytes::complete::tag,
    character::complete::{newline, digit1, one_of},
    multi::{separated_list1},
    sequence::{separated_pair},
    combinator::{map_res, recognize, map}
};

const INPUT: &str = advent_of_code::get_input!();

#[derive(Debug, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", match self {
            Direction::Up => "Up",
            Direction::Down => "Down",
            Direction::Left => "Left",
            Direction::Right => "Right",
        })
     }
}

impl From<char> for Direction {
    fn from(input: char) -> Direction {
        match input {
            'U' => Direction::Up,
            'D' => Direction::Down,
            'L' => Direction::Left,
            'R' => Direction::Right,
            _   => panic!("Unable to parse Direction from string"),
        }
    }
}

#[derive(Debug, Default, Hash, PartialEq, Eq, Clone)]
struct Position {
    x: i32, 
    y: i32
}

impl Position {
    fn dist(&self, other: &Self) -> f64 {
        (((self.x - other.x) as f64).powf(2.0) + ((self.y - other.y) as f64).powf(2.0)).sqrt()
    }
    
    fn move_toward(&mut self, other: &Self) {
        let diff = (other - self).map(|x: i32| -> i32 { if x != 0 { x / x.abs() } else { 0 } } );
        
        self.x += diff.x;
        self.y += diff.y;
    }
    
    fn map(&mut self, f: impl Fn(i32) -> i32) -> Position {
        Position {
            x: f(self.x),
            y: f(self.y),
        }
    }
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result { write!(f, "({}, {})", self.x, self.y) }
}

impl<'a, 'b> std::ops::Add<&'b Position> for &'a Position {
    type Output = Position;
    #[inline(always)]
    fn add(self, other: &'b Position) -> Position {
        Position {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<'a, 'b> std::ops::Sub<&'b Position> for &'a Position {
    type Output = Position;
    #[inline(always)]
    fn sub(self, other: &'b Position) -> Position {
        Position {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<'a, 'b> std::ops::Sub<&'b Position> for Position {
    type Output = Position;
    #[inline(always)]
    fn sub(self, other: &'b Position) -> Position {
        Position {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl std::ops::ShrAssign<&Direction> for Position {
    fn shr_assign(&mut self, dir: &Direction) {
        match dir {
            Direction::Up => self.y += 1,
            Direction::Down => self.y -= 1,
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
        };
    }
}

#[derive(Debug, PartialEq)]
struct MoveInstruction {
    dir: Direction,
    count: usize
}

fn move_instruction(input: &str) -> IResult<&str, MoveInstruction> {
    map(
        separated_pair(
            map(one_of("UDLR"), |c| Direction::from(c) ),
            tag(" "),
            map_res(recognize(digit1), str::parse::<usize>
        )
    ), |(dir, count)| MoveInstruction{dir, count} )(input)
}

fn move_instructions(input: &str) -> IResult<&str, Vec<MoveInstruction>> { 
    separated_list1(newline, move_instruction)(input) 
}

fn visited_positions(move_instructions: &Vec<MoveInstruction>, size: usize) -> HashSet<Position>{
    let mut visited: HashSet<Position> = HashSet::new();
    visited.insert(Position::default());

    let mut rope : Vec<Position> = vec![Position::default(); size];

    // println!("rope: [{}]", &rope.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" "));

    for MoveInstruction{dir, count} in move_instructions {
        for _ in 0..*count {
            rope[0] >>= dir;
            
            for i in 1..rope.len() {
                if rope[i-1].dist(&rope[i]) > 1.5 {
                    let target = rope[i-1].clone();
                    rope[i].move_toward(&target);
                }
            }

            visited.insert(rope.last().unwrap().clone());
        }
    }
    visited
}

fn main() {
    let (_, move_instructions) = move_instructions(INPUT).unwrap();
    println!("The tail of a 2 sized rope visited {} positions", visited_positions(&move_instructions, 2).len());
    println!("The tail of a 10 sized rope visited {} positions", visited_positions(&move_instructions, 10).len());
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

const TEST_COMPLEX_INPUT: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";
    #[test]
    fn simple_case() {
        let (_, move_instructions) = move_instructions(TEST_INPUT).unwrap();
        assert_eq!(visited_positions(&move_instructions, 2).len(), 13);
    }

    #[test]
    fn complex_case() {
        let (_, move_instructions) = move_instructions(TEST_COMPLEX_INPUT).unwrap();
        assert_eq!(visited_positions(&move_instructions, 10).len(), 36);

    }
}