advent_of_code::solution!(11);

use std::collections::HashMap;

use aoc_utils::num::integer::digits_count;

fn parse_input(input: &str) -> Vec<u64> {
    input.split_whitespace().map(|s| s.parse().unwrap()).collect()
}

fn split_digits_in_half(n: u64) -> Option<(u64, u64)> {
    let nb_of_digits = digits_count(&n);

    if nb_of_digits % 2 == 0 {
        let base_10_split = 10u64.pow((nb_of_digits/2) as _);
        let left = n/base_10_split;
        let right = n - left*base_10_split;
        Some((left, right))
    }else {
        None
    }   
}

fn process_stone_blink(
    stone: u64,
    blinks_count: usize,
    cache: &mut HashMap<(u64, usize), usize>
) -> usize {
    if let Some(&count) = cache.get(&(stone, blinks_count)) {
        return count;
    }

    if blinks_count == 0 {
        return 1;
    }

    let stone_count;

    if stone == 0 {
        stone_count = process_stone_blink(1, blinks_count - 1, cache);
    }
    else if let Some((left, right)) = split_digits_in_half(stone) {
        stone_count = process_stone_blink(left, blinks_count - 1, cache)
                + process_stone_blink(right, blinks_count - 1, cache);
    } else {
        stone_count = process_stone_blink(stone*2024, blinks_count - 1, cache);
    }

    cache.insert((stone, blinks_count), stone_count);
    stone_count
}

fn process_blink(stones: &Vec<u64>) -> Vec<u64> {
    stones.iter()
    .flat_map(blink_stone)
    .collect()
}

fn blink_stone(stone: &u64) -> Vec<u64> {
    if *stone == 0 {
        return vec![1];
    }
    let nb_of_digits = digits_count(stone);

    if nb_of_digits % 2 == 0 {
        let base_10_split = 10u64.pow((nb_of_digits/2) as _);
        let left = stone/base_10_split;
        vec![left, stone - left*base_10_split]
    }else {
        vec![stone*2024]
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let stones = parse_input(input);

    let mut blinks_iterator = std::iter::successors(
        Some(stones), |stones| Some(process_blink(stones))
    );

    Some(
        blinks_iterator.nth(25)
        .unwrap()
        .iter()
        .count() as _
    )
}

pub fn part_two(input: &str) -> Option<u64> {

    let stones = parse_input(input);

    const BLINKS_COUNT: usize = 75;
    let mut cache = HashMap::new();

    let stones_total_count = stones.iter().map(|stone| {
        process_stone_blink(*stone, BLINKS_COUNT, &mut cache)
    }).sum::<usize>();

    Some(stones_total_count as _)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "125 17";

    #[test]
    fn test_parse_input() {
        assert_eq!(parse_input(TEST_INPUT), vec![125, 17]);
    }

    #[test]
    fn test_process_blink() {
        let stones = parse_input("125 17");

        let mut blinks_iterator = std::iter::successors(
            Some(stones), |stones| Some(process_blink(stones))
        );
        
        assert_eq!(blinks_iterator.next().unwrap(), vec![125, 17]);
        assert_eq!(blinks_iterator.next().unwrap(), vec![253000, 1, 7]);
        assert_eq!(blinks_iterator.next().unwrap(), vec![253, 0, 2024, 14168]);
        assert_eq!(blinks_iterator.next().unwrap(), vec![512072, 1, 20, 24, 28676032]);
        assert_eq!(blinks_iterator.next().unwrap(), vec![512, 72, 2024, 2, 0, 2, 4, 2867, 6032]);
        assert_eq!(blinks_iterator.next().unwrap(), vec![1036288, 7, 2, 20, 24, 4048, 1, 4048, 8096, 28, 67, 60, 32]);
        assert_eq!(blinks_iterator.next().unwrap(), vec![2097446912, 14168, 4048, 2, 0, 2, 4, 40, 48, 2024, 40, 48, 80, 96, 2, 8, 6, 7, 6, 0, 3, 2]);
    }

    #[test]
    fn test_part_two() {
        let stones = parse_input("125 17");

        let mut blinks_iterator = std::iter::successors(
            Some(stones.clone()), |stones| Some(process_blink(stones))
        );

        let mut cache = HashMap::new();

        const BLINKS_COUNT_TEST: usize = 25;

        let stones_total_count = stones.iter().map(|stone| {
            process_stone_blink(*stone, BLINKS_COUNT_TEST, &mut cache)
        }).sum::<usize>();

        assert_eq!(blinks_iterator.nth(BLINKS_COUNT_TEST).unwrap().iter().count(), stones_total_count);
    }
}