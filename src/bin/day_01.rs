advent_of_code::solution!(1);

const INPUT: &str = advent_of_code::get_input!();

fn parse_input(input: &str) -> (Vec<i32>, Vec<i32>) {
    let mut v1 = Vec::new();
    let mut v2 = Vec::new();
    for line in input.lines() {
        let mut parts = line.split_whitespace();
        v1.push(parts.next().unwrap().parse().unwrap());
        v2.push(parts.next().unwrap().parse().unwrap());
    }
    (v1, v2)
}

pub fn part_one(input: &str) -> Option<i32> {
    let (mut v1, mut v2) = parse_input(input);

    v1.sort();
    v2.sort();

    let sum_abs_diff = v1.iter().zip(v2.iter()).map(|(a, b)| a.abs_diff(*b) as i32).sum();
    Some(sum_abs_diff)
}

pub fn part_two(input: &str) -> Option<i32> {
    let (mut v1, mut v2) = parse_input(input);

    // build map of frequencies for v2
    let mut freqs = std::collections::HashMap::new();
    for &n in &v2 {
        *freqs.entry(n).or_insert(0) += 1;
    }

    let result: i32 = v1.iter().map(|&n| n * freqs.get(&n).unwrap_or(&0)).sum();

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn test_parse_input() {
        let result = parse_input(TEST_INPUT);
        assert_eq!(result, (vec![3, 4, 2, 1, 3, 3], vec![4, 3, 5, 3, 9, 3]));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(TEST_INPUT).unwrap();
        assert_eq!(result, 11);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(TEST_INPUT).unwrap();
        assert_eq!(result, 31);
    }
}