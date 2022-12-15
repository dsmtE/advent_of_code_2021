use std::cmp::Ordering;

use nom::character::complete::line_ending;
use nom::combinator::map;
use nom::multi::separated_list0;
use nom::{
    IResult,
    character::complete,
    multi::{separated_list1},
    sequence::{pair, delimited, separated_pair},
    branch::alt
};

const INPUT: &str = advent_of_code::get_input!();

use advent_of_code::iterator_to_string;

#[derive(Clone, PartialEq, Eq)]
enum Packet {
    Value(u8),
    List(Vec<Packet>)
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Packet::Value(x), Packet::Value(y))=> x.partial_cmp(y),
            (Packet::List(x), Packet::List(y)) => x.partial_cmp(y),
            (Packet::Value(_), Packet::List(_)) => Packet::List(vec![self.clone()]).partial_cmp(other),
            (Packet::List(_), Packet::Value(_)) => self.partial_cmp(&Packet::List(vec![other.clone()])),
        }
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl std::fmt::Display for Packet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Packet::Value(value) => write!(f, "{}", value),
            Packet::List(list) => write!(f, "{}", iterator_to_string(list, ","))
        }
    }
}

fn parse(input: &str) -> IResult<&str, Vec<(Packet, Packet)>> {
    separated_list1(
        pair(
            line_ending,
            line_ending
        ),
        separated_pair(
            parse_line,
            line_ending,
            parse_line
        )
    )(input)
}

fn parse_line(input: &str) -> IResult<&str, Packet> {
    map(
        delimited(
            complete::char('['),
            separated_list0(
                complete::char(','),
                parse_value
            ),
            complete::char(']'),
        ),
        Packet::List,
    )(input)
}

fn parse_value(input: &str) -> IResult<&str, Packet> {
    alt((parse_line, map(complete::u8, Packet::Value)))(input)
}

fn idx_sum_of_right_ordored_pairs(packets_pairs: &Vec<(Packet, Packet)>) -> usize {
    packets_pairs
        .iter()
        .enumerate()
        .filter(|(_, (left, right))| left <= right)
        .map(|(i, _)| i + 1)
        .sum::<usize>()
}

fn part02(packets_pairs: &Vec<(Packet, Packet)>) -> usize {
    // collect all pairs into a Vec of Packets
    let mut res: Vec<Packet> = packets_pairs.iter()
        .map(|(left, right)| vec![left.clone(), right.clone()])
        .collect::<Vec<Vec<_>>>()
        .concat();

    let divider1 = Packet::List(vec![Packet::List(vec![Packet::Value(2)])]);
    let divider2 = Packet::List(vec![Packet::List(vec![Packet::Value(6)])]);

    res.push(divider1.clone());
    res.push(divider2.clone());
    res.sort();

    let pos_divider_1 = res.iter().position(|x| *x == divider1).unwrap();
    let pos_divider_2 = res.iter().position(|x| *x == divider2).unwrap();

    (pos_divider_1 + 1) * (pos_divider_2 + 1)
}

fn main() {
    let (_, packets_pairs) = parse(INPUT).unwrap();

    let first = idx_sum_of_right_ordored_pairs(&packets_pairs);
    let second = part02(&packets_pairs);

    println!("first: {}", first);
    println!("second: {}", second);
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

    #[test]
    fn simple_case() {
        let (_, packets_pairs) = parse(TEST_INPUT).unwrap();

        for pair in &packets_pairs {
            println!("{}\n{}\n", pair.0, pair.1);
        }

        assert_eq!(idx_sum_of_right_ordored_pairs(&packets_pairs), 13);

        assert_eq!(part02(&packets_pairs), 140);

    }
}