const INPUT: &str = aoc_utils::get_input!();

use itertools::Itertools;

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input.lines().map(|line| line.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>()).collect::<Vec<Vec<i32>>>()
}

fn previous_and_next_history_value(history: &Vec<i32>) -> (i32, i32) {

    if history.len() < 2 {
        panic!("History too short");
    }

    let differences = history.windows(2).map(|w| w[1] - w[0]).collect::<Vec<i32>>();

    if differences.iter().all_equal() {
        (
            history[0] - differences[0],
            history.last().unwrap() + differences[0],
        )
    }else {
        let (previous, next) = previous_and_next_history_value(&differences);
        (
            history[0] - previous,
            history.last().unwrap() + next,
        )
    }
}

fn main() {
    let inputs = parse_input(INPUT);

    let result = inputs.iter().map(previous_and_next_history_value).fold((0, 0), |acc, (p, f)| (acc.0 + p, acc.1 + f));
    println!("First star: {}", result.1);
    println!("Second star: {}", result.0);
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    #[test]
    fn parsing() {
        assert_eq!(parse_input(TEST_INPUT), vec![
            vec![0, 3, 6, 9, 12, 15],
            vec![1, 3, 6, 10, 15, 21],
            vec![10, 13, 16, 21, 30, 45],
        ]);
    }

    #[test]
    fn first_start() {
        let inputs = parse_input(TEST_INPUT);

        let result = inputs.iter().map(previous_and_next_history_value).map(|(_, x)| x) .collect::<Vec<_>>();
        assert_eq!(result, vec![18, 28, 68]);
    }

}