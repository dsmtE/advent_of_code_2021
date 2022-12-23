use nom::{
    branch::alt,
    character::complete::{self, line_ending, one_of},
    combinator::map,
    multi::{many1, separated_list1},
    sequence::{pair, separated_pair, terminated},
    IResult,
};

const INPUT: &str = advent_of_code::get_input!();

type Map = Vec<Vec<Option<bool>>>;

fn display_map(map: &Map, pos_and_dir: Option<(&Coord, &Direction)>) {
    for (row, row_values) in map.iter().enumerate() {
        for (col, cell) in row_values.iter().enumerate() {
            if let Some((pos, dir)) = pos_and_dir {
                if pos.row == row as i32 && pos.col == col as i32 {
                    match dir {
                        Direction::R => print!(">"),
                        Direction::L => print!("<"),
                        Direction::U => print!("^"),
                        Direction::D => print!("v"),
                    }
                    continue;
                }
            }

            match cell {
                Some(true) => print!("#"),
                Some(false) => print!("."),
                None => print!(" "),
            }
        }
        println!();
    }
}

#[derive(Clone)]
struct Coord {
    row: i32,
    col: i32,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
enum Direction {
    R = 0,
    D = 1,
    L = 2,
    U = 3,
}

impl std::convert::From<u8> for Direction {
    fn from(v: u8) -> Self {
        match v {
            0 => Direction::R,
            1 => Direction::D,
            2 => Direction::L,
            3 => Direction::U,
            _ => panic!("invalid direction"),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
enum Turn {
    L = 0,
    R = 1,
}

impl std::convert::From<u8> for Turn {
    fn from(v: u8) -> Self {
        match v {
            0 => Turn::L,
            1 => Turn::R,
            _ => panic!("invalid turn"),
        }
    }
}

impl Direction {
    fn turn(self, turn: &Turn) -> Direction {
        return Direction::from((self as u8 + (*turn as u8 + 2) % 3 + 1) % 4);
    }

    fn offset(&self) -> Coord {
        use Direction::*;
        match &self {
            L => Coord { row: 0, col: -1 },
            R => Coord { row: 0, col: 1 },
            U => Coord { row: -1, col: 0 },
            D => Coord { row: 1, col: 0 },
        }
    }
}

enum Instruction {
    Rotate(Turn),
    Forward(u8),
}

impl std::fmt::Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instruction::Rotate(Turn::L) => write!(f, "L"),
            Instruction::Rotate(Turn::R) => write!(f, "R"),
            Instruction::Forward(v) => write!(f, "{}", v),
        }
    }
}

fn parse_map(input: &str) -> IResult<&str, Map> {
    map(
        separated_list1(
            line_ending,
            map(many1(one_of(".# ")), |line| {
                line.into_iter()
                    .map(|c| match c {
                        '.' => Some(false),
                        '#' => Some(true),
                        ' ' => None,
                        _ => panic!("invalid input"),
                    })
                    .collect()
            }),
        ),
        pad_map,
    )(input)
}

fn pad_map(map: Map) -> Map {
    // get max length of a row
    let max_length = map.iter().map(|row| row.len()).fold(0, std::cmp::max);

    // pad all rows to the same length
    map.into_iter()
        .map(|row| {
            row.into_iter()
                .chain(std::iter::repeat(None))
                .take(max_length)
                .collect()
        })
        .collect()
}

fn parse_instructions(input: &str) -> IResult<&str, Vec<Instruction>> {
    many1(alt((
        map(complete::u8, |v| Instruction::Forward(v)),
        map(one_of("LR"), |c| match c {
            'L' => Instruction::Rotate(Turn::L),
            'R' => Instruction::Rotate(Turn::R),
            _ => panic!("invalid input"),
        }),
    )))(input)
}
fn parse_map_and_instructions(input: &str) -> IResult<&str, (Map, Vec<Instruction>)> {
    separated_pair(
        parse_map,
        pair(line_ending, line_ending),
        parse_instructions,
    )(input)
}

fn wrap(map: &Map, current_pos: &Coord, direction: &Direction) -> (Coord, Direction) {
    let Coord { row: dr, col: dc } = direction.offset();
    let mut curr = current_pos.clone();
    // while an open or solid tile exists in the map when walking in the opposite direction, update pos
    while let Some(tile) = map
        .get((curr.row - dr) as usize)
        .and_then(|row| row.get((curr.col - dc) as usize))
    {
        if tile.is_none() {
            break;
        }
        curr.row -= dr;
        curr.col -= dc;
    }

    (curr, *direction)
}

fn wrap_cube_test(map: &Map, pos: &Coord, dir: &Direction) -> (Coord, Direction) {
    let (cube_row, cube_col, new_dir) = match (pos.row / 4, pos.col / 4, dir) {
        (0, 2, Direction::U) => (1, 0, Direction::D),
        (0, 2, Direction::L) => (1, 1, Direction::D),
        (0, 2, Direction::R) => (2, 3, Direction::L),
        (1, 0, Direction::U) => (0, 2, Direction::D),
        (1, 0, Direction::L) => (3, 3, Direction::U),
        (1, 0, Direction::D) => (2, 3, Direction::U),
        (1, 1, Direction::U) => (0, 2, Direction::R),
        (1, 1, Direction::D) => (3, 2, Direction::R),
        (1, 2, Direction::R) => (2, 3, Direction::D),
        (2, 2, Direction::L) => (1, 1, Direction::U),
        (2, 2, Direction::D) => (1, 0, Direction::U),
        (2, 3, Direction::D) => (1, 0, Direction::R),
        (2, 3, Direction::U) => (1, 2, Direction::L),
        (2, 3, Direction::R) => (0, 2, Direction::L),
        _ => unreachable!(),
    };
    // find idxes within the cube
    let (row_idx, col_idx) = (pos.row % 4, pos.col % 4);

    let i = match dir {
        Direction::L => 3 - row_idx,
        Direction::R => row_idx,
        Direction::U => col_idx,
        Direction::D => 3 - col_idx,
    };

    // find new idxes within the cube
    let new_row = match new_dir {
        Direction::L => 3 - i,
        Direction::R => i,
        Direction::U => 3,
        Direction::D => 0,
    };
    let new_col = match new_dir {
        Direction::L => 3,
        Direction::R => 0,
        Direction::U => i,
        Direction::D => 3 - i,
    };

    let new_pos = Coord {
        row: cube_row * 4 + new_row,
        col: cube_col * 4 + new_col,
    };

    (new_pos, new_dir)
}

fn wrap_cube_real(map: &Map, pos: &Coord, dir: &Direction) -> (Coord, Direction) {
    // this huge match statement only covers cases in the real input, but can be expanded to cover everything. It's just tedious
    let (cube_row, cube_col, new_dir) = match (pos.row / 50, pos.col / 50, dir) {
        (0, 1, Direction::U) => (3, 0, Direction::R),
        (0, 1, Direction::L) => (2, 0, Direction::R),
        (0, 2, Direction::U) => (3, 0, Direction::U),
        (0, 2, Direction::R) => (2, 1, Direction::L),
        (0, 2, Direction::D) => (1, 1, Direction::L),
        (1, 1, Direction::R) => (0, 2, Direction::U),
        (1, 1, Direction::L) => (2, 0, Direction::D),
        (2, 0, Direction::U) => (1, 1, Direction::R),
        (2, 0, Direction::L) => (0, 1, Direction::R),
        (2, 1, Direction::R) => (0, 2, Direction::L),
        (2, 1, Direction::D) => (3, 0, Direction::L),
        (3, 0, Direction::R) => (2, 1, Direction::U),
        (3, 0, Direction::D) => (0, 2, Direction::D),
        (3, 0, Direction::L) => (0, 1, Direction::D),
        _ => unreachable!(),
    };
    // find idxes within the cube
    let (row_idx, col_idx) = (pos.row % 50, pos.col % 50);

    let i = match dir {
        Direction::L => 49 - row_idx,
        Direction::R => row_idx,
        Direction::U => col_idx,
        Direction::D => 49 - col_idx,
    };

    // find new idxes within the cube
    let new_row = match new_dir {
        Direction::L => 49 - i,
        Direction::R => i,
        Direction::U => 49,
        Direction::D => 0,
    };
    let new_col = match new_dir {
        Direction::L => 49,
        Direction::R => 0,
        Direction::U => i,
        Direction::D => 49 - i,
    };

    let new_pos = Coord {
        row: cube_row * 50 + new_row,
        col: cube_col * 50 + new_col,
    };

    (new_pos, new_dir)
}

fn password(pos: &Coord, direction: &Direction) -> i32 {
    1000 * (pos.row + 1) + 4 * (pos.col + 1) + *direction as u8 as i32
}

fn solve(map: &Map, instructions: &Vec<Instruction>, wrap_funct: & dyn Fn(&Map, &Coord, &Direction) -> (Coord, Direction)) -> i32 {
    // go to the first open position on the top row (skip the Nones)
    let start_col = map[0]
        .iter()
        .position(|tile| !tile.unwrap_or(true))
        .unwrap() as i32;

    let mut pos = Coord {
        row: 0,
        col: start_col,
    };
    let mut dir = Direction::R;

    for instruction in instructions {
        // display_map(map, Some((&pos, &dir)));
        // println!("");
        match instruction {
            Instruction::Rotate(turn) => dir = dir.turn(turn),
            Instruction::Forward(amount) => {
                for _ in 0..*amount {
                    let Coord { row: dr, col: dc } = dir.offset();
                    let target_pos = map
                        .get((pos.row + dr) as usize)
                        .and_then(|row| row.get((pos.col + dc) as usize))
                        .unwrap_or(&None);

                    match target_pos {
                        Some(true) => break, // if new tile is solid, stop moving
                        Some(false) => {
                            // if new tile is open, move there
                            pos = Coord {
                                row: pos.row + dr,
                                col: pos.col + dc,
                            };
                        }
                        // if new tile is not found, wrap around
                        None => {
                            let (new_pos, new_dir) = wrap_funct(&map, &pos, &dir);
                            // if the new_pos is solid, stop moving
                            if map[new_pos.row as usize][new_pos.col as usize].unwrap_or(true) {
                                break;
                            }
                            // if the new_pos is open, move there
                            pos = new_pos;
                            dir = new_dir;
                        }
                    }
                }
            }
        }
    }

    password(&pos, &dir)
}

fn main() {
    let (_, (map, move_instructions)) = parse_map_and_instructions(INPUT).unwrap();
    println!("Part 1: {}", solve(&map, &move_instructions, &wrap));
    println!("Part 2: {}", solve(&map, &move_instructions, &wrap_cube_real));
}

#[cfg(test)]
mod tests {
    use advent_of_code::iterator_to_string;

    use super::*;
    const TEST_INPUT: &str = "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5";

    #[test]
    fn parsing_test() {
        let (_, (map, move_instructions)) = parse_map_and_instructions(TEST_INPUT).unwrap();

        display_map(&map, None);
        println!("{}", iterator_to_string(&move_instructions, ""));
    }

    #[test]
    // test turn function of Direction
    fn turn_test() {
        use Direction::*;
        assert_eq!(L.turn(&Turn::L), D);
        assert_eq!(L.turn(&Turn::R), U);
        assert_eq!(R.turn(&Turn::L), U);
        assert_eq!(R.turn(&Turn::R), D);
        assert_eq!(U.turn(&Turn::L), L);
        assert_eq!(U.turn(&Turn::R), R);
        assert_eq!(D.turn(&Turn::L), R);
        assert_eq!(D.turn(&Turn::R), L);
    }

    #[test]
    fn test() {
        let (_, (map, move_instructions)) = parse_map_and_instructions(TEST_INPUT).unwrap();
        assert_eq!(solve(&map, &move_instructions, &wrap), 6032);
        assert_eq!(solve(&map, &move_instructions, &wrap_cube_test), 5031);
    }
}
