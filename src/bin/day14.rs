use std::cmp::Ordering;
use std::collections::HashSet;

use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::combinator::map;
use nom::multi::separated_list0;
use nom::{
    number::complete::float,
    IResult,
    character::complete,
    multi::{separated_list1},
    sequence::{pair, delimited, separated_pair},
    branch::alt,
};

const INPUT: &str = advent_of_code::get_input!();

use advent_of_code::mapped_iterator_to_string;

type Coord = (u32, u32);

fn parse(input: &str) -> IResult<&str, Vec<Vec<Coord>>> {
    separated_list1(
        line_ending,
        parse_line
    )(input)
}

fn parse_line(input: &str) -> IResult<&str, Vec<Coord>> {
    separated_list1(
        tag(" -> "),
        separated_pair(
            complete::u32,
            complete::char(','),
            complete::u32
        )
    )(input)
}

fn generate_rocks_positions_hashset(coords: &Vec<Vec<Coord>>) -> HashSet<Coord> {
    coords.iter()
        .flat_map(|line: &Vec<Coord>|
            line.windows(2)
            .map(|coord_window| 
                (coord_window[0].to_owned(), coord_window[1].to_owned())
            ).collect::<Vec<(Coord, Coord)>>()
        )
        .flat_map(|(from, to)| {
            if from.0 == to.0 { // vertical
                (std::cmp::min(from.1, to.1)..=std::cmp::max(from.1, to.1))
                    .map(|y| (from.0, y))
                    .collect::<Vec<Coord>>()
            } else { // horizontal
                (std::cmp::min(from.0, to.0)..=std::cmp::max(from.0, to.0))
                    .map(|x| (x, from.1))
                    .collect::<Vec<Coord>>()
            }
        })
    .collect::<HashSet<Coord>>()
}

fn fall_sand(rocks_coords: &HashSet<Coord>, sand_start_coord: Coord, with_floor: bool) -> usize {
    let mut sand_coords : HashSet<Coord> = HashSet::new();

    let max_y = rocks_coords.iter().map(|(_x, y)| y).max().unwrap();

    let floor = if with_floor { Some(max_y + 2) } else { None };
    
    'l: loop {
        let mut cur_pos = sand_start_coord;
        
        if floor.is_some() && sand_coords.contains(&cur_pos) {
            break 'l;
        }

        while let Some(next_pos) = sand_next_pos(&rocks_coords, &sand_coords, cur_pos, floor ) {
            cur_pos = next_pos;
            if floor.is_none() && cur_pos.1 == max_y + 1 {
                break 'l;
            }
        }
        sand_coords.insert(cur_pos);
    }

    sand_coords.len()
}

fn sand_next_pos(rocks_coords: &HashSet<Coord>, sands_coords: &HashSet<Coord>, (x, y): Coord, floor: Option<u32>) -> Option<Coord> {
    if let Some(floor_level) = floor {
        if y+1 == floor_level { return None; }
    }

    let fall_candidates = [(x, y + 1), (x - 1, y + 1), (x + 1, y + 1)];
    fall_candidates.into_iter().find(|candidate_coord| { 
        (!rocks_coords.contains(candidate_coord) && !sands_coords.contains(candidate_coord))
    })
}
fn main() {
    let (_, rock_build_coordinates) = parse(INPUT).unwrap();
    let rocks_coords: HashSet<Coord> = generate_rocks_positions_hashset(&rock_build_coordinates);

    println!("sand count : {}", fall_sand(&rocks_coords, (500, 0), false));
    println!("sand count with floor : {}", fall_sand(&rocks_coords, (500, 0), true));

}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn parsing_test_case() {
        assert_eq!(parse_line("498,4 -> 498,6 -> 496,6"), Ok(("", vec![(498,4), (498,6), (496,6)])));

        let (_, values) = parse(TEST_INPUT).unwrap();

        for line in &values {
            println!("{}", mapped_iterator_to_string(line, " -> ", |(x, y)| format!("{},{}", x, y)));
        }
    }

    #[test]
    fn simple_case() {
        let (_, rock_build_coordinates) = parse(TEST_INPUT).unwrap();

        let rocks_coords: HashSet<Coord> = generate_rocks_positions_hashset(&rock_build_coordinates);

        println!("{}", mapped_iterator_to_string(&rocks_coords, ", ", |(x, y)| format!("({},{})", x, y)));
        
        assert_eq!(fall_sand(&rocks_coords, (500, 0), false), 24);

        assert_eq!(fall_sand(&rocks_coords, (500, 0), true), 93);

    }


}