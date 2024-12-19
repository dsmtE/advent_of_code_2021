use nom::{bytes::complete::tag, character::complete::newline, multi::separated_list1, sequence::{delimited, pair, preceded, separated_pair, tuple}, IResult, Parser};

use aoc_utils::nom_parsing::number;

advent_of_code::solution!(13);

#[derive(Debug, PartialEq)]
struct PrizeMachine {
    a: (i64, i64),
    b: (i64, i64),
    prize: (i64, i64),
}

fn parse_price_machine(input: &str) -> IResult<&str, PrizeMachine> {
    tuple((
        delimited(tag("Button A: X+"), separated_pair(number, tag(", Y+"), number), newline),
        delimited(tag("Button B: X+"), separated_pair(number, tag(", Y+"), number), newline),
        preceded(tag("Prize: X="), separated_pair(number, tag(", Y="), number)),
    ))(input)
    .map(|(str, (a, b, prize))| 
        (str, PrizeMachine { a, b, prize }))
}

fn parse_input(input: &str) -> IResult<&str, Vec<PrizeMachine>> {
    separated_list1(pair(newline, newline), parse_price_machine)(input)
}


// (Button A X) * (A presses) + (Button B X) * (B presses) = Prize X
// (Button A Y) * (A presses) + (Button B Y) * (B presses) = Prize Y

// this could be represented as a matrix equation
//   [ Ax Bx ][ Ap ] = [ Px ]
//   [ Ay By ][ Bp ] = [ Py ]

fn solve_equation(matrix : [i64; 4], result: [i64; 2]) -> Option<(i64, i64)> {
    let det = matrix[0] * matrix[3] - matrix[1] * matrix[2];

    if det == 0 {
        return None;
    }

    let mut x = matrix[3] * result[0] - matrix[1] * result[1];
    let mut y = matrix[0] * result[1] - matrix[2] * result[0];

    // we search for integer solution only
    if x % det != 0 || y % det != 0 {
        return None;
    }

    x /= det;
    y /= det;

    Some((x, y))
}

pub fn part_one(input: &str) -> Option<i64> {
    let (_, machines) = parse_input(input).ok()?;

    let tokens_by_machines = machines.iter()
        .filter_map(|machine| 
            solve_equation([
                machine.a.0, machine.b.0,
                machine.a.1, machine.b.1,
            ], 
            [
                machine.prize.0,
                machine.prize.1
            ])
        )
        .filter(|(x, y)| *x <= 100 && *y <= 100)
        .collect::<Vec<_>>();

    let total_cost = tokens_by_machines.iter().map(|(x, y)| x * 3 + y).sum();

    Some(total_cost)
}

pub fn part_two(input: &str) -> Option<i64> {
    let (_, machines) = parse_input(input).ok()?;

    Some(
        machines.iter()
        .filter_map(|machine| 
            solve_equation([
                machine.a.0, machine.b.0,
                machine.a.1, machine.b.1,
            ], 
            [
                machine.prize.0 + 10_000_000_000_000,
                machine.prize.1 + 10_000_000_000_000
            ])
        )
        // .filter(|(x, y)| *x <= 100 && *y <= 100)
        .map(|(x, y)| x * 3 + y)
        .sum()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

    #[test]
    fn test_parse_input() {
        let result = parse_input(TEST_INPUT);
        assert_eq!(result, Ok(("", vec![
            PrizeMachine { a: (94, 34), b: (22, 67), prize: (8400, 5400) },
            PrizeMachine { a: (26, 66), b: (67, 21), prize: (12748, 12176) },
            PrizeMachine { a: (17, 86), b: (84, 37), prize: (7870, 6450) },
            PrizeMachine { a: (69, 23), b: (27, 71), prize: (18641, 10279) },
        ])));
    }
    #[test]
    fn test_part_one() {

        let (_, machines) = parse_input(TEST_INPUT).ok().unwrap();

        let tokens_by_machines = machines.iter()
        .filter_map(|machine| 
            solve_equation([
                machine.a.0, machine.b.0,
                machine.a.1, machine.b.1,
            ], [machine.prize.0, machine.prize.1])
        )
        .filter(|(x, y)| *x <= 100 && *y <= 100)
        .collect::<Vec<_>>();

        assert_eq!(tokens_by_machines, vec![
            (80, 40),
            (38, 86),
        ]);

        let total_cost: i64 = tokens_by_machines.iter().map(|(x, y)| x * 3 + y).sum();

        assert_eq!(total_cost, 280 + 200);
    }
}