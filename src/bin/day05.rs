use nom::{
    IResult,
    bytes::complete::tag,
    character::complete::{self, newline, alpha1, multispace1, multispace0, digit1},
    multi::{separated_list1, many1},
    sequence::{tuple, delimited, terminated, preceded},
    combinator::{map_res, recognize},
    branch::alt
};

const INPUT: &str = advent_of_code::get_input!();

// parsing move instructions
#[derive(Debug, PartialEq, Eq)]
struct MoveInstruction {
    count: usize,
    from: usize,
    to: usize
}

fn parse_usize(input: &str) -> IResult<&str, usize> {
    map_res(recognize(digit1), str::parse)(input)
}

fn move_instruction(input: &str) -> IResult<&str, MoveInstruction> {
    let (input, (_, count, _, from, _, to)) = tuple((
        tag("move "),
        parse_usize,
        tag(" from "),
        parse_usize,
        tag(" to "),
        parse_usize
    ))(input)?;
    Ok((input, MoveInstruction{count, from: from-1, to: to-1}))
}

fn move_instructions(input: &str) -> IResult<&str, Vec<MoveInstruction>> { 
    separated_list1(newline, move_instruction)(input) 
}

fn crate_letter(input: &str) -> IResult<&str, Option<char>> {
    let (input, crate_str) = alt((
        tag("   "),
        delimited(complete::char('['), alpha1, complete::char(']')
        )
    ))(input)?;

    Ok((input, match crate_str {
        "   " => None,
        value => Some(value.chars().next().unwrap())
    }))
}

fn stacks_and_move_instructions(input: &str) -> IResult<&str, (Vec<Vec<char>>, Vec<MoveInstruction>)> {
    let (input, horizontal_stack) = terminated(
        separated_list1(
            newline, 
            separated_list1(
                complete::char(' '), 
                crate_letter
            )
        ), 
        newline)(input)?;

    // numbers
    let (input, _) = many1(preceded(multispace1, digit1))(input)?;
    let (input, _) = multispace0(input)?;

    let (input, move_instructions) = move_instructions(input)?;
    
    // Build stacks from horizontal parsing
    let mut stacks: Vec<Vec<char>> = Vec::with_capacity(horizontal_stack.len());
    for _ in 0..=horizontal_stack.len() { stacks.push(vec![]); }
    for chars in horizontal_stack.iter().rev() { 
        for (i, char) in chars.iter().enumerate() {
            if let Some(c) = char { stacks[i].push(*c); }
        }
    }

    return Ok((input, (stacks, move_instructions)));
}

fn top_items(stacks: Vec<Vec<char>>) -> String {
    stacks.iter()
    .map(|stack| stack.last().expect("Stack should not be empty"))
    .collect::<String>()
}

fn main() {

    let (_, (stacks, move_instructions)) = stacks_and_move_instructions(INPUT).unwrap();

    let mut stacks01: Vec<Vec<char>> = stacks.clone();
    let mut stacks02: Vec<Vec<char>> = stacks.clone();

    move_instructions.iter().for_each(|move_instruction| {
        for _ in 0..move_instruction.count {
            let c = stacks01[move_instruction.from].pop().expect("Unable to retrive item from empty stack");
            stacks01[move_instruction.to].push(c);
        }
    });

    move_instructions.iter().for_each(|move_instruction| {
        let from_stack_len = stacks02[move_instruction.from].len();
        let extracted_items = stacks02[move_instruction.from]
        .drain(from_stack_len-move_instruction.count..)
        .collect::<Vec<char>>();
        stacks02[move_instruction.to].extend(extracted_items);
    });

    println!("result: {}", top_items(stacks01));

    println!("advanced result: {}", top_items(stacks02));
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
    #[test]
    fn simple_case() {

        let (_, (stacks, move_instructions)) = stacks_and_move_instructions(TEST_INPUT).unwrap();

        assert_eq!(stacks, vec![
            vec!['Z', 'N'],
            vec!['M', 'C', 'D'],
            vec!['P'],
            ]);

        assert_eq!(move_instructions, vec![
            MoveInstruction{count:1, from: 1, to: 0},
            MoveInstruction{count:3, from: 0, to: 2},
            MoveInstruction{count:2, from: 1, to: 0},
            MoveInstruction{count:1, from: 0, to: 1}
            ]);
    }

    #[test]
    fn complexe_case() {
    }
}