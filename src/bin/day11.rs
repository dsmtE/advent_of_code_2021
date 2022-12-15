use core::fmt;
use std::{collections::VecDeque};
use std::borrow::BorrowMut;

use advent_of_code::iterator_to_string;

use nom::{
    IResult,
    bytes::complete::tag,
    character::complete::{self, newline, digit1, one_of, multispace0},
    multi::{separated_list1},
    sequence::{preceded, pair, tuple, delimited, separated_pair, terminated},
    combinator::{map_res, recognize},
    branch::alt
};

const INPUT: &str = advent_of_code::get_input!();


#[derive(Debug, Clone, PartialEq, Hash, Eq)]
enum Operator { Mul, Add }

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
enum Operand { Number(u64), Same }

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
struct Operation {
    operator: Operator,
    operand: Operand,
}

impl Operation {
    fn to_function(&self) -> Box<dyn Fn(u64) -> u64> {
        match self.operator {
            Operator::Mul => match self.operand {
                Operand::Number(n) => Box::new(move |x| x * n),
                Operand::Same => Box::new(|x| x * x),
            },
            Operator::Add => match self.operand {
                Operand::Number(n) => Box::new(move |x| x + n),
                Operand::Same => Box::new(|x| x + x),
            },
        }
    }
}

#[derive(Debug, Clone)]
struct MonkeySetup {
    starting_items: Vec<u64>,
    operation: Operation,
    divider_test: u64,
    target_if_true: usize,
    target_if_false: usize,
}

impl fmt::Display for MonkeySetup {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, 
            "MonkeySetup {{
                starting_items: {},
                operation: {:?},
                divider_test: {},
                target_if_true: {},
                target_if_false: {}
            }}", 
            iterator_to_string(&self.starting_items, ", "),
            self.operation,
            self.divider_test,
            self.target_if_true,
            self.target_if_false
        )
    }
}

struct Monkey {
    items: VecDeque<u64>,
    inspection_count: usize,
    func: Box<dyn Fn(u64) -> u64>,
    divider_test: u64,
    target_if_true: usize,
    target_if_false: usize,
}

impl From<&MonkeySetup> for Monkey {
    fn from(setup: &MonkeySetup) -> Self {
        Self {
            items: VecDeque::from(setup.starting_items.clone()),
            inspection_count: 0,
            func: setup.operation.to_function(),
            divider_test: setup.divider_test,
            target_if_true: setup.target_if_true,
            target_if_false: setup.target_if_false,
        }
    }
}

impl std::fmt::Debug for Monkey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, 
            "Monkey {{
                items: {},
                inspection_count: {},
                divider_test: {},
                target_if_true: {},
                target_if_false: {}
            }}",
            iterator_to_string(&self.items, ", "),
            self.inspection_count,
            self.divider_test,
            self.target_if_true,
            self.target_if_false
        )
    }
}

#[inline(always)]
fn parse_number<T>(input: &str) -> IResult<&str, T> 
where
T: std::str::FromStr
{
    map_res(recognize(digit1), str::parse::<T>)(input)
}

fn operation(input: &str) -> IResult<&str, Operation> {
    let (input, (op, right)) = 
        preceded(
            pair(multispace0,tag("Operation: new = old ")),
            separated_pair(
                one_of("*+"),
                multispace0,
                alt((tag("old"), digit1))
            )
        )(input)?;

        let operator: Operator = match op {
            '*' => Operator::Mul,
            '+' => Operator::Add,
            _ => unreachable!()
        };

        let operand: Operand = match right.parse::<u64>() {
            Err(_) => Operand::Same,
            Ok(val) => Operand::Number(val)
        };

        Ok((input, Operation { operator, operand }))
}

fn starting_items(input: &str) -> IResult<&str, Vec<u64>> {
    preceded(
        pair(multispace0,tag("Starting items: ")),
        separated_list1(tag(", "), parse_number::<u64>)
    )(input)
}

fn divider(input: &str) -> IResult<&str, u64> {
    preceded(
        pair(multispace0,tag("Test: divisible by ")),
        parse_number::<u64>
    )(input)
}

fn monkey(input: &str) -> IResult<&str, MonkeySetup> {
    let (input, _) = tuple((tag("Monkey "), digit1, complete::char(':'), newline))(input)?;

    let (input, starting_items) = terminated(starting_items, newline)(input)?;
    let (input, operation) = terminated(operation, newline)(input)?;
    let (input, divider_test) = terminated(divider, newline)(input)?;

    let (input, target_if_true) = delimited(
        pair(multispace0, tag("If true: throw to monkey ")),
        parse_number::<usize>,
        newline
    )(input)?;

    let (input, target_if_false) = preceded(
        pair(multispace0, tag("If false: throw to monkey ")),
        parse_number::<usize>
    )(input)?;

    Ok((input, MonkeySetup {
        starting_items,
        operation,
        divider_test,
        target_if_true,
        target_if_false,
    }))
}

fn monkeys(input: &str) -> IResult<&str, Vec<MonkeySetup>> { 
    separated_list1(multispace0, monkey)(input)
}

fn solve01(monkeys_setup_list: &Vec<MonkeySetup>) -> usize {
    let mut monkeys: Vec<Monkey> = monkeys_setup_list.iter()
        .map(|m| m.into())
        .collect();
    
    for _round in 0..20 {
        for i in 0..monkeys.len() {
            let items: Vec<u64> = monkeys[i].items.drain(..).collect();
            
            for item in items.into_iter().rev() {
                monkeys[i].inspection_count += 1;
                let mut new_item = (monkeys[i].func)(item);
                
                // monkey bored
                new_item /= 3;
                
                let target = if new_item % monkeys[i].divider_test == 0 {
                    monkeys[i].target_if_true
                } else {
                    monkeys[i].target_if_false
                };
                monkeys[target].borrow_mut().items.push_back(new_item)
            }
        }
    }

    let mut inspection_count = monkeys.iter().map(|m| m.inspection_count).collect::<Vec<usize>>();
    inspection_count.sort();
    inspection_count.iter().rev().take(2).product::<usize>()
}

fn solve02(monkeys_setup_list: &Vec<MonkeySetup>) -> usize {
    let mut monkeys: Vec<Monkey> = monkeys_setup_list.iter()
        .map(|m| m.into())
        .collect();
    
    let magic_divider = monkeys_setup_list
        .iter()
        .map(|m| m.divider_test)
        .product::<u64>();

    for _round in 0..10000 {
        for i in 0..monkeys.len() {
            let items: Vec<u64> = monkeys[i].items.drain(..).collect();
            
            for item in items.into_iter().rev() {
                monkeys[i].inspection_count += 1;
                let mut new_item = (monkeys[i].func)(item);
                
                // magic trick to keep the validity of the divisibility and the number lower to be albe to keep up with many iterations
                new_item %= magic_divider;

                let target = if new_item % monkeys[i].divider_test == 0 {
                    monkeys[i].target_if_true
                } else {
                    monkeys[i].target_if_false
                };
                monkeys[target].borrow_mut().items.push_back(new_item)
            }
        }
    }

    let mut inspection_count = monkeys.iter().map(|m| m.inspection_count).collect::<Vec<usize>>();
    inspection_count.sort();
    inspection_count.iter().rev().take(2).product::<usize>()
}

fn main() {
    let (_, monkeys_setup_list) = monkeys(INPUT).unwrap();
        println!("{}", iterator_to_string(&monkeys_setup_list, ", \n"));

        println!("result01: {}", solve01(&monkeys_setup_list));
        println!("result02: {}", solve02(&monkeys_setup_list));
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3
  
Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0
  
Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3
  
Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
";

    #[test]
    fn simple_case() {
        let (_, monkeys_setup_list) = monkeys(TEST_INPUT).unwrap();
        println!("{}", iterator_to_string(&monkeys_setup_list, ", \n"));

        assert_eq!(solve01(&monkeys_setup_list), 10605);

        assert_eq!(solve02(&monkeys_setup_list), 2713310158);

    }

    #[test]
    fn parsing_test() {
        assert_eq!(starting_items("   Starting items: 79, 98"), Ok(("", vec![79, 98])));

        assert_eq!(parse_number::<u64>("15"), Ok(("", 15)));

        assert_eq!(divider("Test: divisible by 23"), Ok(("", 23)));

        assert_eq!(operation("Operation: new = old * 19"), Ok(("", Operation { operator: Operator::Mul, operand: Operand::Number(19) })));

    }
}