advent_of_code::solution!(2);

use itertools::Itertools;

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input.lines()
        .map(|line| line
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap()).collect()).collect()
}

pub fn is_valid_level_diff(x: &i32) -> bool {
    return x.abs() >= 1 && x.abs() <= 3
}

pub fn is_level_valid_part_one(level: &Vec<i32>) -> bool {
    // Get the difference between each level
    let diff: Vec<i32> = level.windows(2).map(|x| x[1] - x[0]).collect();

    // Any two adjacent levels differ by at least one and at most three.
    if !diff.iter().all(is_valid_level_diff) {
        return false;
    }

    // check if all element are of the same sign
    if diff.windows(2).any(|x| x[0] * x[1] < 0) {
        return false;
    }

    true
}

pub fn is_level_valid_part_two(level: &Vec<i32>) -> bool {
    // brute force solution using combinations on every subset of the level
    level.iter()
        .combinations(level.len() - 1)
        .any(|l| is_level_valid_part_one(&l.into_iter().map(|x| *x).collect()))

    // todo better iterative solution 
}


pub fn part_one(input: &str) -> Option<usize> {
    let levels = parse_input(input);

    let valid_levels_count = levels.iter().filter(|level| is_level_valid_part_one(level)).count();

    Some(valid_levels_count)
}

pub fn part_two(input: &str) -> Option<usize> {
    let levels = parse_input(input);

    Some(levels.iter().filter(|level| is_level_valid_part_two(level)).count())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn test_parse_input() {
        let result = parse_input(TEST_INPUT);
        assert_eq!(result,
            vec![
                vec![7,6,4,2,1],
                vec![1,2,7,8,9],
                vec![9,7,6,2,1],
                vec![1,3,2,4,5],
                vec![8,6,4,4,1],
                vec![1,3,6,7,9]
        ]);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(TEST_INPUT).unwrap();
        assert_eq!(result, 2);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(TEST_INPUT).unwrap();
        assert_eq!(result, 4);
    }
}