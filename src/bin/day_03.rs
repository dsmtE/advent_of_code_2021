use nom::{
    branch::alt, bytes::complete::tag, character::complete::{anychar, digit1}, combinator::{map_res, recognize}, multi::{many0, many_till}, sequence::{delimited, preceded, separated_pair}, IResult, Parser
};

advent_of_code::solution!(3);

type Mul = (u32, u32);

#[derive(PartialEq, Debug)]
enum Instruction {
    Mul(Mul),
    Do,
    Dont
}

fn mul_instruction(input: &str) -> IResult<&str, Mul> {
    preceded(
        tag("mul"),
        delimited(
            tag("("),
            separated_pair(
                map_res(recognize(digit1), str::parse::<u32>),
                tag(","),
                map_res(recognize(digit1), str::parse::<u32>)
            ),
            tag(")"),
        )
    )(input)
}

fn instruction(input: &str) -> IResult<&str, Instruction> {
    alt((
        mul_instruction.map(Instruction::Mul),
        tag("do()").map(|_| Instruction::Do),
        tag("don't()").map(|_| Instruction::Dont)
    ))(input)
}

pub fn parse_mul(input: &str) -> IResult<&str, Vec<Mul>> {
    many0(many_till(anychar, mul_instruction).map(|(_, mul)| mul))(input)
}

fn parse_instruction(input: &str) -> IResult<&str, Vec<Instruction>> {
    many0(many_till(anychar, instruction).map(|(_, i)| i))(input)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_, mul_list) = parse_mul(input).unwrap();
    mul_list.iter().map(|(a,b)| a*b).sum::<u32>().into()
}

pub fn part_two(input: &str) -> Option<u32> {
    let (_, instructions) = parse_instruction(input).unwrap();

    let mut ignore = false;
    let mut result = 0;
    for instruction in instructions {
        match instruction {
            Instruction::Mul(mul) => {
                if !ignore {
                    result += mul.0 * mul.1;
                }
            },
            Instruction::Do => {
                ignore = false;
            },
            Instruction::Dont => {
                ignore = true;
            }
        }
    }
    result.into()

}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const TEST_INPUT_PART_TWO: &str = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
    #[test]
    fn test_parse_input() {

        assert_eq!(
            mul_instruction("mul(2,4)"),
            Ok(("", (2,4)))
        );
        assert_eq!(
            many_till(anychar, mul_instruction)("xxmul(3,2smul(5,8)"),
            Ok(("", ("xxmul(3,2s".chars().collect(), (5,8))))
        );

        assert_eq!(
            parse_mul(TEST_INPUT),
            Ok((")", vec![(2,4),(5,5),(11,8),(8,5)]))
        );

        assert_eq!(
            parse_instruction(TEST_INPUT_PART_TWO),
            Ok((")", vec![
                Instruction::Mul((2,4)),
                Instruction::Dont,
                Instruction::Mul((5,5)),
                Instruction::Mul((11,8)),
                Instruction::Do,
                Instruction::Mul((8,5)),
            ]))
        );

        
    }

    #[test]
    fn test_part_one() {
        let result = part_one(TEST_INPUT);
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(TEST_INPUT_PART_TWO);
        assert_eq!(result, Some(48));
    }
}