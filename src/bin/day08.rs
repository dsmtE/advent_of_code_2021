use std::collections::HashMap;

use nom::{
    IResult,
    combinator::map_res,
    multi::{many1, separated_list1},
    sequence::{tuple, preceded, terminated, delimited},
    character::complete::{alphanumeric1, newline},
    bytes::complete::tag,
};

const INPUT: &str = aoc_utils::get_input!();

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Instruction {
    Right,
    Left
}

impl From<char> for Instruction {
    fn from(c: char) -> Self {
        match c {
            'R' => Instruction::Right,
            'L' => Instruction::Left,
            _ => panic!("Invalid instruction"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Mapping<'a> {
    left: &'a str,
    right: &'a str,
}


fn parse_instructions(input: &str) -> IResult<&str, Vec<Instruction>> {
    many1(
        map_res(
            nom::character::complete::one_of("RL"),
            |c| -> Result<Instruction, ()> { Ok(c.into())}
        )
    )(input)
}

fn parse_mapping(input: &str) -> IResult<&str, (&str, Mapping)> {
    map_res(
        nom::sequence::tuple((
            alphanumeric1,
            preceded(tag(" = ("), alphanumeric1),
            delimited(tag(", "), alphanumeric1, tag(")"))
        )),
        |(left,right1, right2)| -> Result<(&str, Mapping), ()> { Ok((left, Mapping { left: right1, right: right2 })) }
    )(input)
}

fn parse_instructions_and_mapping(input: &str) -> IResult<&str, (Vec<Instruction>, HashMap<&str, Mapping>)> {
    let (_, (instructions ,mappings)) = tuple((
        terminated(parse_instructions, newline),
        preceded(
            newline,
            separated_list1(newline, parse_mapping)
        )
    ))(input)?;

    let mapping = HashMap::from_iter(mappings.into_iter());

    Ok((input, (instructions, mapping)))
    
}

fn main() {
    let (instructions, mapping) = parse_instructions_and_mapping(INPUT).unwrap().1;
        
        let mut current_value = "AAA";
        let mut count: usize = 0;

        for instruction in instructions.iter().cycle() {
            let mapping = mapping.get(&current_value).unwrap();
            current_value = match instruction {
                Instruction::Right => mapping.right,
                Instruction::Left => mapping.left,
            };
            count += 1;
            
            if current_value == "ZZZ" {
                break;
            }
        }

        println!("Count: {}", count);
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

    #[test]
    fn parsing() {

        assert_eq!(parse_mapping("AAA = (BBB, CCC)").unwrap().1, ("AAA", Mapping { left: "BBB", right: "CCC" }));
        assert_eq!(parse_instructions_and_mapping(TEST_INPUT).unwrap().1,
            (
                vec![Instruction::Right, Instruction::Left], {
                HashMap::from_iter(vec![
                    ("AAA", Mapping { left: "BBB", right: "CCC" }),
                    ("BBB", Mapping { left: "DDD", right: "EEE" }),
                    ("CCC", Mapping { left: "ZZZ", right: "GGG" }),
                    ("DDD", Mapping { left: "DDD", right: "DDD" }),
                    ("EEE", Mapping { left: "EEE", right: "EEE" }),
                    ("GGG", Mapping { left: "GGG", right: "GGG" }),
                    ("ZZZ", Mapping { left: "ZZZ", right: "ZZZ" }),
                ].into_iter()
            )
        }));

    }

    #[test]
    fn first_start() {
        let (instructions, mapping) = parse_instructions_and_mapping(TEST_INPUT).unwrap().1;
        
        let mut current_value = "AAA";
        let mut count: usize = 0;

        for instruction in instructions.iter().cycle() {
            let mapping = mapping.get(&current_value).unwrap();
            current_value = match instruction {
                Instruction::Right => mapping.right,
                Instruction::Left => mapping.left,
            };
            count += 1;
            
            if current_value == "ZZZ" {
                break;
            }
        }

        println!("Count: {}", count);
    }

    #[test]
    fn second_star() {
    }

}