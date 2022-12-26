use std::collections::HashMap;

use nom::{
    character::complete::{line_ending, one_of},
    combinator::{map, recognize},
    multi::{many1, separated_list1},
    sequence::pair,
    IResult,
};

const INPUT: &str = advent_of_code::get_input!();

const ROCKS: &str = "####\n\n.#.\n###\n.#.\n\n..#\n..#\n###\n\n#\n#\n#\n#\n\n##\n##";

type Pos = (u8, usize);

struct RockFormation {
    shape: Vec<u8>,
    width: u8,
    height: usize,
}

impl RockFormation {
    fn new(input: &str) -> Self {
        let shape = input
            .lines()
            .map(|line| {
                line.chars()
                    .fold(0u8, |acc, c| (acc << 1) | if c == '#' { 1 } else { 0 })
            })
            .collect::<Vec<u8>>();

        let width = input.lines().map(|line| line.len()).max().unwrap() as u8;
        let height = input.lines().count();
        Self {
            shape,
            width,
            height,
        }
    }

    fn collide(&self, pos: Pos, field: &RocksField) -> bool {
        for (dy, row) in self.shape.iter().enumerate() {
            let shift = 7 - self.width - pos.0;
            if let Some(v) = field.rocks.get(pos.1 + (self.height-1) - dy) {
                if v & (row << shift) != 0 {
                    return true;
                }
            }
        }
        false
    }
}

fn parse_rocks(input: &str) -> IResult<&str, Vec<RockFormation>> {
    separated_list1(
        pair(line_ending, line_ending),
        map(
            recognize(separated_list1(line_ending, many1(one_of("#.")))),
            |str| RockFormation::new(str),
        ),
    )(input)
}

impl std::fmt::Display for RockFormation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for row in self.shape.iter() {
            for i in 0..self.width {
                write!(
                    f, "{}",
                    if row & (1 << (self.width - 1 - i)) != 0 { '#' } else { '.' }
                )?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

enum Move {
    Left,
    Right,
}

impl TryFrom<char> for Move {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '<' => Ok(Self::Left),
            '>' => Ok(Self::Right),
            _ => Err(()),
        }
    }
}

impl std::fmt::Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}",
            match self {
                Move::Left => "<",
                Move::Right => ">",
            }
        )
    }
}

fn parse_moves(input: &str) -> IResult<&str, Vec<Move>> {
    many1(map(one_of("><"),|c| Move::try_from(c).unwrap()))(input)
}


struct RocksField {
    rocks: Vec<u8>,
}

pub fn shift(a: u8, b: i8) -> u8 {
    if b > 0 { a << b } else { a >> b.abs() }
}

impl RocksField {
    fn new() -> Self {
        Self {
            rocks: vec![],
        }
    }

    fn height(&self) -> usize { self.rocks.len() }

    fn add(&mut self, rock: &RockFormation, rock_position: Pos) {
        for (dy, row) in rock.shape.iter().enumerate().rev() {
            let shift = 7 - rock.width - rock_position.0;
            if let Some(v) = self.rocks.get_mut(rock_position.1 + (rock.height-1) - dy) {
                *v |= row << shift;
            } else {
                self.rocks.push(row << shift);
            }
        }
    }

    fn top(&self, size: usize) -> Vec<u8> {
        self.rocks.iter().rev().take(size).cloned().collect()
    }

    fn display(&self, rock: &RockFormation, rock_position: Pos, max: Option<usize>) {

        let max_y = std::cmp::max(
            self.height(),
            rock_position.1 + rock.height - 1
        );

        println!("rock pos: ({}, {})", rock_position.0, rock_position.1);

        for y in (0..=max_y).rev().take(max.unwrap_or(max_y+1)) {
            print!("|");
            for x in 0..=6 {
                print!(
                    "{}",
                    if self.rocks.get(y).unwrap_or(&0u8) & (1 << (6 - x)) != 0 {
                        '#'
                    } else if rock_position.1 <= y && y < rock_position.1 + rock.height
                        && rock_position.0 <= x && x < rock_position.0 + rock.width
                        && rock.shape[rock_position.1 + (rock.height - 1) - y] & shift(1,rock_position.0 as i8 + (rock.width as i8 - 1) - x as i8) != 0 {
                        '@'
                    } else {
                        '.'
                    }
                );
            }
            println!("|");
        }
        
        if let Some(max) = max {
            println!("... ({} more)\n", max_y - max);
        }else {
            println!("+-------+\n");
        }
    }
}

fn falling_rocks_height(rocks_formations: &[RockFormation], move_instructions: &[Move], rock_limit: usize, verbose: bool) -> usize {
    let mut move_index = 0;
    let mut field = RocksField::new();
    let mut cycle_found = false;
    let mut cycle_cache: HashMap<(usize, usize, Vec<u8>), (usize, usize)> = HashMap::new();

    let mut skipped_height = 0;
    let mut i = 0;
    while i < rock_limit {
        // println!("Rock {}", i);
        let rock_index: usize = i % rocks_formations.len();
        let rock = &rocks_formations[rock_index];
        
        // todo: better key than using last 100 line of the field
        let key = (rock_index, move_index, field.top(100));

        if !cycle_found {
            if let Some((cycle_i, cycle_field_height)) = cycle_cache.get(&key) {

                let blocks_passed = i - cycle_i;
                let pattern_height = field.height() - cycle_field_height;
                let remaining_blocks = rock_limit - i;
                let pattern_count = remaining_blocks / blocks_passed;
                let blocks_to_skip = pattern_count * blocks_passed;

                if blocks_to_skip != 0 {

                    assert_eq!(
                        blocks_to_skip % 5,
                        0,
                        "skipped blocks should be a multiple of 5 to be on the same rock next turn"
                    );
                    println!("Cycle detected");
                    field.display(rock, (2u8, field.height() + 3), Some(20));

                    dbg!(
                        i,
                        cycle_i,
                        field.height(),
                        cycle_field_height,
                        move_index,
                        blocks_passed,
                        pattern_height,
                        remaining_blocks,
                        pattern_count,
                        blocks_to_skip,
                    );
                    
                    skipped_height += pattern_height * pattern_count;
                    println!("{} : {}",
                        i + blocks_to_skip,
                        rock_limit - remaining_blocks % blocks_passed,
                    );
                    i += blocks_to_skip;

                    // move_index = (move_index + 4) % move_instructions.len();

                    dbg!(skipped_height, i);
                    
                    cycle_found = true;
                    continue;
                }
                // else {
                //     println!("No more blocks to skip");
                // }
            }
        }

        cycle_cache.insert(key, (i, field.height()));

        if cycle_found {
            println!("{}: {:?} {}", i, rock_index, move_index);
        }
        // use bottom left corner as rock position
        let mut rock_position: Pos = (2u8, field.height() + 3);

        if verbose { println!("New rock"); }

        loop {

            let move_instruction = &move_instructions[move_index];
            let move_shift: i8 = match move_instruction {
                Move::Left => -1,
                Move::Right => 1,
            };

            if verbose {
                println!("Move {} ({})", move_instruction, move_index);
                field.display(rock, rock_position, Some(20));
            }
            move_index = (move_index + 1) % move_instructions.len();
            
            // move only if rock isn't on the edge of the field
            if (rock_position.0 > 0 && move_shift == -1) || (rock_position.0 + (rock.width-1) < 6 && move_shift == 1) {
                // do not move if rock collides with another rock in the field
                let new_x_pos = (rock_position.0 as i8 + move_shift) as u8;
                if !rock.collide((new_x_pos, rock_position.1), &field) {
                    rock_position.0 = new_x_pos;
                }
            }

            if verbose {field.display(rock, rock_position, Some(20));}

            // do not move down if rock collides with another rock in the field or if it's on the bottom of the field
            if rock_position.1 > 0 && !rock.collide((rock_position.0, rock_position.1 - 1) , &field) {
                rock_position.1 -= 1;
            } else {
                break;
            }
        }

        field.add(rock, (rock_position.0, rock_position.1));
        i += 1;
    }

    field.height() + skipped_height
}

fn main() {
    let (_, move_instructions) = parse_moves(INPUT).unwrap();
    let (_, rocks_formations) = parse_rocks(ROCKS).unwrap();

    // let part1 = falling_rocks_height(&rocks_formations, &move_instructions, 2022, false);

    // println!("Height after 2022 rocks: {}", part1);
    // assert_eq!(part1, 3157);

    let part2 = falling_rocks_height(&rocks_formations, &move_instructions, 1000000000000, false);

    println!("Height after 1000000000000 rocks: {}", part2);
    assert_eq!(part2, 1581449275296);
}

#[cfg(test)]
mod tests {
    use advent_of_code::iterator_to_string;

    use super::*;
    const TEST_INPUT: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    #[test]
    fn test_parsing() {
        let (_, move_instructions) = parse_moves(TEST_INPUT).unwrap();
        println!("[{}]", TEST_INPUT);
        println!("{}", iterator_to_string(&move_instructions, ""));

        let (_, rocks_formations) = parse_rocks(ROCKS).unwrap();
        println!("{}", iterator_to_string(&rocks_formations, "\n"));
    }

    #[test]
    fn test01() {
        let (_, move_instructions) = parse_moves(TEST_INPUT).unwrap();
        let (_, rocks_formations) = parse_rocks(ROCKS).unwrap();

        let part01 = falling_rocks_height(&rocks_formations, &move_instructions, 2022, false);
        assert_eq!(part01, 3068);
    }

    #[test]
    fn test02() {
        let (_, move_instructions) = parse_moves(TEST_INPUT).unwrap();
        let (_, rocks_formations) = parse_rocks(ROCKS).unwrap();

        let part02 = falling_rocks_height(&rocks_formations, &move_instructions, 1000000000000, false);
        assert_eq!(part02, 1514285714288);
    }
}
