use std::{fmt::Debug};

use nom::{
    IResult,
    bytes::complete::tag,
    character::complete::{self, newline, digit1},
    multi::{separated_list1},
    sequence::{preceded, pair},
    combinator::{map_res, recognize, map, opt},
    branch::alt
};

const INPUT: &str = advent_of_code::get_input!();

#[derive(Debug, PartialEq)]
enum Instruction {
    AddX(i32),
    Noop,
}

fn instruction(input: &str) -> IResult<&str, Instruction> {
    alt((
        map(tag("noop"), |_| Instruction::Noop),
        map(
            preceded(
                tag("addx "),
                map_res(
                    recognize(pair(opt(complete::char('-')), digit1))
                    ,
                    str::parse::<i32>
                )
            ),
            |value| Instruction::AddX(value))
    ))(input)
}

fn instructions(input: &str) -> IResult<&str, Vec<Instruction>> { 
    separated_list1(newline, instruction)(input) 
}


#[inline(always)]
fn signal_strength(cycle: &usize, register_value: &i32) -> i32 {
    *cycle as i32 * register_value
}

fn compute_sum_signal_strength(
    instructions: &Vec<Instruction>,
    wanted_signal_strength_predicate: impl Fn(usize) -> bool,
    break_at_cycle: Option<usize>,
) -> i32 {

    let mut register: i32 = 1;
    let mut total: i32 = 0;
    let mut cycle: usize = 1;

    for instruction in instructions {
        if wanted_signal_strength_predicate(cycle) {
            total += signal_strength(&cycle, &register);
        }

        if let Some(bc) = break_at_cycle {
            if cycle >= bc { break; }
        }
        cycle += 1;
        
        if let Instruction::AddX(val) = instruction {
            if wanted_signal_strength_predicate(cycle) {
                total += signal_strength(&cycle, &register);
            }

            register += val;
            cycle += 1;
        };
    }

    total
}

fn get_pixel(cycle: usize, register: i32, sprite_width: usize, cols_count: usize) -> char {
    let curr_col = (cycle - 1) % cols_count;
    if (curr_col as i32).abs_diff(register) <= sprite_width as u32 / 2 { '█' } else { ' ' }
}

fn CRT_print(instructions: &Vec<Instruction>) -> String {

    let mut register: i32 = 1;
    let mut cycle: usize = 1;

    let mut crt_string = String::new();

    for instruction in instructions {

        crt_string.push(get_pixel(cycle, register, 3, 40));

        if cycle % 40 == 0 { crt_string.push('\n') }

        cycle += 1;

        if let Instruction::AddX(val) = instruction {

            crt_string.push(get_pixel(cycle, register, 3, 40));
            if cycle % 40 == 0 { crt_string.push('\n') }

            register += val;
            cycle += 1;
        };
    }

    crt_string
}


fn main() {
    let (_, instructions) = instructions(INPUT).unwrap();

        let sum = compute_sum_signal_strength(
            &instructions, |cycle| cycle % 40 == 20,
            Some(220)
        );

        println!("Sum of signal strengths: {}", sum);

        println!("{}", CRT_print(&instructions));
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "addx 15\naddx -11\naddx 6\naddx -3\naddx 5\naddx -1\naddx -8\naddx 13\naddx 4\nnoop\naddx -1\naddx 5\naddx -1\naddx 5\naddx -1\naddx 5\naddx -1\naddx 5\naddx -1\naddx -35\naddx 1\naddx 24\naddx -19\naddx 1\naddx 16\naddx -11\nnoop\nnoop\naddx 21\naddx -15\nnoop\nnoop\naddx -3\naddx 9\naddx 1\naddx -3\naddx 8\naddx 1\naddx 5\nnoop\nnoop\nnoop\nnoop\nnoop\naddx -36\nnoop\naddx 1\naddx 7\nnoop\nnoop\nnoop\naddx 2\naddx 6\nnoop\nnoop\nnoop\nnoop\nnoop\naddx 1\nnoop\nnoop\naddx 7\naddx 1\nnoop\naddx -13\naddx 13\naddx 7\nnoop\naddx 1\naddx -33\nnoop\nnoop\nnoop\naddx 2\nnoop\nnoop\nnoop\naddx 8\nnoop\naddx -1\naddx 2\naddx 1\nnoop\naddx 17\naddx -9\naddx 1\naddx 1\naddx -3\naddx 11\nnoop\nnoop\naddx 1\nnoop\naddx 1\nnoop\nnoop\naddx -13\naddx -19\naddx 1\naddx 3\naddx 26\naddx -30\naddx 12\naddx -1\naddx 3\naddx 1\nnoop\nnoop\nnoop\naddx -9\naddx 18\naddx 1\naddx 2\nnoop\nnoop\naddx 9\nnoop\nnoop\nnoop\naddx -1\naddx 2\naddx -37\naddx 1\naddx 3\nnoop\naddx 15\naddx -21\naddx 22\naddx -6\naddx 1\nnoop\naddx 2\naddx 1\nnoop\naddx -10\nnoop\nnoop\naddx 20\naddx 1\naddx 2\naddx 2\naddx -6\naddx -11\nnoop\nnoop\nnoop";

    #[test]
    fn simple_case() {
        let (_, instructions) = instructions(TEST_INPUT).unwrap();
        
        let sum = compute_sum_signal_strength(
            // match the 20th, 60th, 100th, 140th, 180th, and 220th cycles.
            &instructions, |cycle| cycle % 40 == 20,
            Some(220)
        );
        
        assert_eq!(sum, 13140);
    }

    #[test]
    fn complex_case() {
        let (_, instructions) = instructions(TEST_INPUT).unwrap();
        
        assert_eq!(CRT_print(&instructions), "██  ██  ██  ██  ██  ██  ██  ██  ██  ██  \n███   ███   ███   ███   ███   ███   ███ \n████    ████    ████    ████    ████    \n█████     █████     █████     █████     \n██████      ██████      ██████      ████\n███████       ███████       ███████     \n");
    }

    #[test]
    fn parsing_test() {
        assert_eq!(instruction("addx 15"), Ok(("", Instruction::AddX(15))));
        assert_eq!(instruction("addx -14"), Ok(("", Instruction::AddX(-14))));
        assert_eq!(instruction("noop"), Ok(("", Instruction::Noop)));
    }
}