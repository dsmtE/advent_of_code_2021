use nom::{
    character::complete::{self, line_ending},
    combinator::{iterator, eof},
    sequence::{terminated},
    IResult, branch::alt,
};

use rayon::prelude::{IntoParallelRefIterator, ParallelIterator, IndexedParallelIterator};

const INPUT: &str = advent_of_code::get_input!();

#[derive(Debug, Clone, PartialEq)]
struct InputNumber {
    value: i64,
    original_index: usize    
}

fn parse_numbers(input: &str) -> IResult<&str, Vec<InputNumber>> {
    let mut it = iterator(
        input,
        terminated(
            complete::i64,
            alt((line_ending, eof))
        ),
    );

    let numbers = it.enumerate()
        .map(|(index, value)| { InputNumber { value, original_index: index }})
        .collect::<Vec<_>>();

    let (input, _) = it.finish()?;
    Ok((input, numbers))
}

fn solve(numbers: &Vec<InputNumber>, cycles: i64) -> i64 {
    let numbers_count = numbers.len() as i64 - 1;

    let mut state = (*numbers).clone();
    for _ in 0..cycles {
        for current in 0..state.len() {
            let index = state.iter()
                .position(|x| x.original_index == current )
                .unwrap();
            let mut new_index = index as i64 + state[index].value;
            // new_index = ((new_index % numbers_count) + numbers_count) % numbers_count;
            let new_index = new_index.rem_euclid(numbers_count);
            
            let number = state.remove(index);
            state.insert( new_index as usize, number);
        }
    }

    groove_coordinate(&state)
}

fn groove_coordinate(state: &Vec<InputNumber>) -> i64 {
    let zero_index = state.iter().position(|InputNumber{value, ..}| *value == 0).unwrap();
    (1..=3).map(|i| state[(zero_index + (i * 1000)) % state.len()].value).sum()
}
fn main() {

    let (_, mut numbers) = parse_numbers(INPUT).unwrap();
     
    println!("part 01: {}", solve(&numbers, 1));

    numbers.iter_mut()
            .for_each(|InputNumber { value, .. }| *value *= 811589153);

    println!("part 02: {}", solve(&numbers, 10));
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "1
2
-3
3
-2
0
4";

    #[test]
    fn parsing_test() {
        assert_eq!(
            parse_numbers(TEST_INPUT),
            Ok((
                "",
                vec![
                    InputNumber {original_index: 0, value: 1}, 
                    InputNumber {original_index: 1, value: 2}, 
                    InputNumber {original_index: 2, value: -3}, 
                    InputNumber {original_index: 3, value: 3}, 
                    InputNumber {original_index: 4, value: -2}, 
                    InputNumber {original_index: 5, value: 0}, 
                    InputNumber {original_index: 6, value: 4}
                ]
            ))
        );
    }

    #[test]
    fn part1_test
    () {
        let (_, mut numbers) = parse_numbers(TEST_INPUT).unwrap();
        assert_eq!(solve(&numbers, 1), 3);
        
        numbers.iter_mut()
            .for_each(|InputNumber { value, .. }| *value *= 811589153);

        assert_eq!(solve(&numbers, 10), 1623178306);
    
    }
}
