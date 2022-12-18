use std::{fmt::write, collections::HashSet};

use nom::{multi::{many1, separated_list1}, IResult, combinator::map, branch::alt, character::complete::{self, line_ending}, sequence::pair};

use advent_of_code::iterator_to_string;
use advent_of_code::mapped_iterator_to_string;

const INPUT: &str = advent_of_code::get_input!();

const ROCKS: &str = "####\n\n.#.\n###\n.#.\n\n..#\n..#\n###\n\n#\n#\n#\n#\n\n##\n##";

type Coord = (i64, i64);
struct RockFormation(Vec<Vec<bool>>);

impl RockFormation {
    fn height(&self) -> usize { self.0.len() }
    fn width(&self) -> usize { self.0.iter().map(|row| row.len()).max().unwrap()}

    fn collide_in_field(&self, field: &HashSet<Coord>, rock_position: Coord) -> bool {

        // chec if it in the bound of the field
        if rock_position.1 < 0 || rock_position.0 < 0 || rock_position.0 + self.width() as i64 - 1 >= 7  {
            return true;
        }

        let height = self.height();
        self.0.iter().enumerate().any(|(j, row)| {
            row.iter().enumerate().any(|(i, &b)| {
                b && field.contains(&(rock_position.0 + i as i64, rock_position.1 + (height-1-j) as i64))
            })
        })
    }
}

impl std::fmt::Display for RockFormation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", 
            mapped_iterator_to_string(
                &self.0, "\n",
                |row: &Vec<bool>| mapped_iterator_to_string(row, "", |&b| if b { '#' } else { '.' })
            ),
        )
    }
}

enum Move {
    Left,
    Right,
}

impl std::fmt::Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", match self {
            Move::Left => "<",
            Move::Right => ">",
        })
    }
}

fn parse_moves(input: &str) -> IResult<&str, Vec<Move>> {
    many1(
        alt((
            map(complete::char('>'), |_| Move::Right),
            map(complete::char('<'), |_| Move::Left),
        ))
    )(input)
}

fn parse_rocks(input: &str) -> IResult<&str, Vec<RockFormation>> {
    separated_list1(
        pair(line_ending, line_ending),
        map(separated_list1(
            line_ending,
            many1(
                alt((
                    map(complete::char('#'), |_| true),
                    map(complete::char('.'), |_| false),
                ))
            )
        ),
        |rock_formation| RockFormation(rock_formation) )
    )(input)
}


struct RocksField{
    positions: HashSet<Coord>,
    check_offset: usize,
}
impl RocksField {

    fn max_height(&self) -> i64 {
        self.positions.iter().map(|(_, y)| *y+1).max().unwrap_or(0)
    }

    fn add_rock(&mut self, rock: &RockFormation, rock_position: Coord) {
        let height = rock.height();
        rock.0.iter().enumerate().for_each(|(j, row)| {
            row.iter().enumerate().for_each(|(i, &b)| {
                if b {
                    self.positions.insert((rock_position.0 + i as i64, rock_position.1 + (height-1-j) as i64));
                }
            })
        });
    }

    fn collide(&self, rock: &RockFormation, rock_position: Coord) -> bool {
        // check boundry
        if rock_position.1 < 0 || rock_position.0 < 0 || rock_position.0 + rock.width() as i64 - 1 >= 7  {
            return true;
        }

        let height = rock.height();
        rock.0.iter().enumerate().any(|(j, row)| {
            row.iter().enumerate().any(|(i, &b)| {
                b && self.positions.contains(&(rock_position.0 + i as i64, rock_position.1 + (height-1-j) as i64 - self.check_offset as i64))
            })
        })
    }

    fn display(&self, rock: &RockFormation, rock_position: Coord) {
        let max_y = std::cmp::max(
            self.positions.iter().map(|(_, y)| *y).max().unwrap_or(0),
            rock_position.1 + rock.height() as i64 - 1
        );

        println!("rock_position: ({},{})", rock_position.0, rock_position.1);
        for y in (0..=max_y).rev() {
            print!("|");
            (0..=6).map(|x| {
                if self.positions.contains(&(x, y)) {
                    '#'
                } else if rock_position.1 <= y && y < rock_position.1 + rock.height() as i64
                    && rock_position.0 <= x && x < rock_position.0 + rock.width() as i64
                    && rock.0[((rock.height() as i64 - 1) - (y - rock_position.1)) as usize][(x - rock_position.0) as usize]
                {
                    '@'
                } else {
                    '.'
                }
            }).for_each(|c| print!("{}", c));
            println!("|");
        }
        println!("+-------+");
    }
}


fn falling_rocks_height(rocks_formations: &[RockFormation], move_instructions: &[Move], rock_limit: usize, verbose: bool) -> i64 {
    let mut rocks = rocks_formations.iter().cycle();
        let mut moves = move_instructions.iter().cycle();

        let mut rocks_field = RocksField { positions: HashSet::new(), check_offset: 0 };

        let mut rock_placed = 0;

        while rock_placed < rock_limit {
            let current_rock_formation = rocks.next().unwrap();

            // use bottom left corner as rock position
            let mut current_rock_position: Coord = (2, rocks_field.max_height() + 3);
            
            if verbose { println!("New rock formation"); }

            loop {

                if verbose { rocks_field.display(current_rock_formation, current_rock_position); }
                // wind
                let next_move = moves.next().unwrap();
                let desired_next_position: Coord = match next_move {
                    Move::Left => (current_rock_position.0 - 1, current_rock_position.1),
                    Move::Right => (current_rock_position.0 + 1, current_rock_position.1),
                };

                if !rocks_field.collide(current_rock_formation, desired_next_position) {
                    current_rock_position = desired_next_position;
                }

                if verbose{
                    println!("Wind move {}", match next_move {
                        Move::Left => "<-",
                        Move::Right => "->",
                    });
                    rocks_field.display(current_rock_formation, current_rock_position);
                }

                // downward
                let desired_next_position: Coord = (current_rock_position.0, current_rock_position.1 - 1);
                if !rocks_field.collide(current_rock_formation, desired_next_position) {
                    current_rock_position = desired_next_position;
                } else {
                    rocks_field.add_rock(current_rock_formation, current_rock_position);
                    rock_placed += 1;
                    if verbose {
                        println!("rock_placed: {}", rock_placed);
                        rocks_field.display(current_rock_formation, current_rock_position);
                    }
                    break;
                }
            }
        }
        rocks_field.max_height()
}
fn main() {
    let (_, move_instructions) = parse_moves(INPUT).unwrap();
    let (_, rocks_formations) = parse_rocks(ROCKS).unwrap();


    let result = falling_rocks_height(&rocks_formations, &move_instructions, 2022, false);

    println!("Max height after 2022 rocks: {}", result);
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;
    const TEST_INPUT: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    #[test]
    fn test_parsing() {
        let (_, move_instructions) = parse_moves(TEST_INPUT).unwrap();
        println!("[{}]", TEST_INPUT);
        println!("{}", iterator_to_string(&move_instructions, ""));

        let (_, rocks_formations) = parse_rocks(ROCKS).unwrap();
        println!("{}", iterator_to_string(&rocks_formations, "\n\n"));
    }

    #[test]
    fn test01() {
        let (_, move_instructions) = parse_moves(TEST_INPUT).unwrap();
        let (_, rocks_formations) = parse_rocks(ROCKS).unwrap();

        let part01 = falling_rocks_height(&rocks_formations, &move_instructions, 2022, false);
        assert_eq!(part01, 3068);
    }

}