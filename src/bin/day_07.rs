use itertools::Itertools;
use nom::{bytes::complete::tag, character::complete::newline, multi::separated_list1, sequence::separated_pair, IResult};

use aoc_utils::nom_parsing::number;
use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};

advent_of_code::solution!(7);

#[derive(Debug, PartialEq)] 
pub struct Equation {
    expected_result: i64,
    numbers: Vec<i64>,
}

pub fn parse_input(input: &str) -> IResult<&str, Vec<Equation>> {
    separated_list1(newline,parse_equation)(input)
}

pub fn parse_equation(input: &str) ->  IResult<&str, Equation> {
    separated_pair(number, tag(": "), separated_list1(tag(" "), number)
    )(input).map(|(input, (wanted_result, numbers))| 
        (input, Equation { expected_result: wanted_result, numbers })
    )
}

#[derive(Clone, PartialEq)]
pub enum Operator {
    Add,
    Mul,
    Concat,
}

impl std::fmt::Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operator::Add => write!(f, "+"),
            Operator::Mul => write!(f, "*"),
            Operator::Concat => write!(f, "||"),
        }
    }
}

impl std::fmt::Debug for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

pub fn number_of_digits(mut number: i64) -> usize {
    let mut count = 0;
    while number > 0 {
        number /= 10;
        count += 1;
    }
    count
}

pub fn evaluate_with_operator(operator: &Operator, a: i64, b: i64) -> i64 {
    match operator {
        Operator::Add => a + b,
        Operator::Mul => a * b,
        Operator::Concat => a * 10_i64.pow(number_of_digits(b) as _) as i64 + b
    }
}

pub fn apply_operator_to_numbers(operators: &[Operator], numbers: &[i64]) -> i64 {
    assert_eq!(operators.len(), numbers.len()-1);
    let mut result = numbers[0];
    // could be better using simple heuristic and stop if the result is already bigger than the expected result
    for (number, op) in numbers.iter().skip(1).zip(operators.iter()) {
        result = evaluate_with_operator(op, result, *number);
    }
    result
}

pub fn equation_is_possible(operation: &Equation, operators_to_use: Vec<Operator>) -> bool {
    // try every combination of operators and numbers to see if the wanted result can be achieved

    if cfg!(test) {
        println!(" {} with {:?}",
        operation.expected_result,
        operation.numbers
        );
    }

    if operation.numbers.len() == 1 {
        return operation.numbers[0] == operation.expected_result;
    }

    let operators_combinations= 
        (0..operation.numbers.len()-1)
        .map(|_| operators_to_use.clone())
        .multi_cartesian_product().collect::<Vec<_>>();

    if cfg!(test) {
        println!("{:?}", operators_combinations);
    }

    operators_combinations.par_iter().any(|operators_combination| {
        apply_operator_to_numbers(operators_combination, &operation.numbers) == operation.expected_result
    })
}

pub fn part_one(input: &str) -> Option<i64> {
    let equations = parse_input(input).unwrap().1;

    let possible_operations: i64 = equations.into_par_iter()
        .filter(|equation| equation_is_possible(equation, vec![Operator::Add, Operator::Mul]))
        .map(|o| o.expected_result)
        .sum(); 

    Some(possible_operations)
}

pub fn part_two(input: &str) -> Option<i64> {
    let operations = parse_input(input).unwrap().1;

    let possible_operations : i64 = operations.into_iter()
        .filter(|equation| equation_is_possible(equation, vec![Operator::Add, Operator::Mul, Operator::Concat]))
        .map(|o| o.expected_result)
        .sum(); 

        Some(possible_operations)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn test_parse_input() {
        assert_eq!(parse_input(TEST_INPUT),
            Ok(("", vec![
                Equation { expected_result: 190, numbers: vec![10, 19] },
                Equation { expected_result: 3267, numbers: vec![81, 40, 27] },
                Equation { expected_result: 83, numbers: vec![17, 5] },
                Equation { expected_result: 156, numbers: vec![15, 6] },
                Equation { expected_result: 7290, numbers: vec![6, 8, 6, 15] },
                Equation { expected_result: 161011, numbers: vec![16, 10, 13] },
                Equation { expected_result: 192, numbers: vec![17, 8, 14] },
                Equation { expected_result: 21037, numbers: vec![9, 7, 18, 13] },
                Equation { expected_result: 292, numbers: vec![11, 6, 16, 20] },
            ]))
        );
    }
    #[test]
    fn test_part_one() {
        let operations = parse_input(TEST_INPUT).unwrap().1;

        let possible_operations = operations.into_iter()
            .filter(|equation| equation_is_possible(equation, vec![Operator::Add, Operator::Mul]))
            .map(|o| o.expected_result)
            .collect::<Vec<_>>();

        assert_eq!(possible_operations, vec![190, 3267, 292]);
    }

    #[test]
    fn test_part_two() {
        let operations = parse_input(TEST_INPUT).unwrap().1;

        let possible_operations = operations.into_iter()
            .filter(|equation| equation_is_possible(equation, vec![Operator::Add, Operator::Mul, Operator::Concat]))
            .map(|o| o.expected_result)
            .collect::<Vec<_>>();

        assert_eq!(possible_operations, vec![190, 3267, 156, 7290, 192, 292]);
    }
}