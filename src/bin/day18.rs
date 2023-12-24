use aoc_utils::{
    cartesian::{Direction,Point2, p2, Cartesian2},
    pick_area, shoelace_area,
    nom_parsing::number,
};
use nom::{
    IResult,
    multi::separated_list1,
    character::complete::{newline, one_of},
    sequence::{tuple, delimited, terminated, preceded},
    combinator::map_res,
    bytes::complete::{tag, take_while_m_n},
};

use nom::character::complete::space1;

const INPUT: &str = aoc_utils::get_input!();


fn parse_instructions(test_input: &str) -> Vec<(Direction, u32, &str)> {
    separated_list1(newline, parse_instruction)(test_input).unwrap().1
}

fn parse_instruction(input: &str) -> IResult<&str, (Direction, u32, &str)> {
    tuple((
        terminated(map_res(one_of("UDLR"), |c| -> Result<Direction, ()> { Ok(Direction::from(c)) }), space1),
        terminated(number::<u32>, space1),
        delimited(tag("("), parse_hex_color, tag(")")),
    ))(input)
}

fn parse_hex_color(input: &str) -> IResult<&str, &str> {
    preceded(tag("#"), take_while_m_n(6, 6, |c: char| c.is_ascii_hexdigit()))(input)
}

fn instructions_corners_and_edge_length(instructions: &[(Direction, u32, &str)], part2: bool) -> (Vec<Point2>, u64) {
    let mut corners = vec![];
    let mut current_position = p2(0, 0);
    let mut edge_length = 0;

    for instruction in instructions {
        let (mut direction, mut distance, color_hex_str) = *instruction;

        if part2 {
            distance = u32::from_str_radix(&color_hex_str[0..5], 16).unwrap();
            direction = match color_hex_str.chars().nth(5).unwrap() {
                '0' => Direction::Right,
                '1' => Direction::Down,
                '2' => Direction::Left,
                '3' => Direction::Up,
                _ => unreachable!(),
            };   
        }

        current_position += direction.vector() * distance as i64;
        corners.push(current_position);
        edge_length += distance as u64;
    }

    (corners, edge_length)
}

fn main() {
    let instructions = parse_instructions(INPUT);

    let (corners, edge_length) = instructions_corners_and_edge_length(&instructions, false);
    let corners_as_tuples = corners.iter().map(|&p| (p.x(), p.y())).collect::<Vec<_>>();
    let area = pick_area(shoelace_area(&corners_as_tuples) as u64, edge_length);

    println!("Part 1: {}", area);

    let (corners, edge_length) = instructions_corners_and_edge_length(&instructions, true);
    let corners_as_tuples = corners.iter().map(|&p| (p.x(), p.y())).collect::<Vec<_>>();
    let area = pick_area(shoelace_area(&corners_as_tuples) as u64, edge_length);

    println!("Part 2: {}", area);
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)
";

    #[test]
    fn parsing() {
        let instructions = parse_instructions(TEST_INPUT);

        assert_eq!(instructions, vec![
            (Direction::Right, 6, "70c710"),
            (Direction::Down, 5, "0dc571"),
            (Direction::Left, 2, "5713f0"),
            (Direction::Down, 2, "d2c081"),
            (Direction::Right, 2, "59c680"),
            (Direction::Down, 2, "411b91"),
            (Direction::Left, 5, "8ceee2"),
            (Direction::Up, 2, "caa173"),
            (Direction::Left, 1, "1b58a2"),
            (Direction::Up, 2, "caa171"),
            (Direction::Right, 2, "7807d2"),
            (Direction::Up, 3, "a77fa3"),
            (Direction::Left, 2, "015232"),
            (Direction::Up, 2, "7a21e3"),
        ]);
    }

    #[test]
    fn first_start() {
        let instructions = parse_instructions(TEST_INPUT);

        let (corners, edge_length) = instructions_corners_and_edge_length(&instructions, false);

        assert_eq!(corners, vec![
            p2(6, 0), p2(6, 5), p2(4, 5), p2(4, 7), p2(6, 7), p2(6, 9), p2(1, 9), 
            p2(1, 7), p2(0, 7), p2(0, 5), p2(2, 5), p2(2, 2), p2(0, 2), p2(0, 0)]);

        assert_eq!(edge_length, 38);

        let corners_as_tuples = corners.iter().map(|&p| (p.x(), p.y())).collect::<Vec<_>>();
        let area = pick_area(shoelace_area(&corners_as_tuples) as u64, edge_length);

        assert_eq!(area, 62);
    }

    #[test]
    fn second_start() {
        let instructions = parse_instructions(TEST_INPUT);

        let (corners, edge_length) = instructions_corners_and_edge_length(&instructions, true);
        let corners_as_tuples = corners.iter().map(|&p| (p.x(), p.y())).collect::<Vec<_>>();
        let area = pick_area(shoelace_area(&corners_as_tuples) as u64, edge_length);
    
        assert_eq!(area, 952408144115);
    }
    
    
}