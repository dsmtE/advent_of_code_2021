use std::collections::BTreeSet;

const INPUT: &str = advent_of_code_2023::get_input!();

fn parse_input(input: & str) -> Vec<(Vec<u32>, Vec<u32>)>{
    input
        .lines()
        .map(|line| {
            let (first, second) = line.split_once(" | ").unwrap();
            (
                first.split_once(": ").unwrap().1.split_ascii_whitespace().map(|s| s.parse::<u32>().unwrap()).collect::<Vec<_>>(),
                second.split_ascii_whitespace().map(|s| s.parse::<u32>().unwrap()).collect::<Vec<_>>()
            )
        }).collect::<Vec<_>>()
}

fn main() {   
    let cards_sets = parse_input(INPUT);

    let cards_matching_numbers_count = cards_sets.iter().map(|(winning_number, number)| {
        BTreeSet::from_iter(winning_number.iter().cloned())
        .intersection(&BTreeSet::from_iter(number.iter().cloned()))
        .count()
    }).collect::<Vec<_>>();

    let points = cards_matching_numbers_count.iter().filter(|&n| *n > 0).map(|n| 2u32.pow(*n as u32 - 1)).sum::<u32>();;

    println!("First start: {}", points);

    let mut card_copies_count = vec![1; cards_sets.len()];

    cards_matching_numbers_count.iter().enumerate().for_each(|(index, matching_numbers_count)| {
        for i in 0..*matching_numbers_count {
            card_copies_count[index+i+1] += card_copies_count[index];
        }
    });

    let total_cards_count = card_copies_count.iter().sum::<u32>();

    println!("Second star: {}", total_cards_count);
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn parsing() {
        let cards_sets = parse_input(TEST_INPUT);
        
        assert_eq!(cards_sets, vec![
            (vec![41, 48, 83, 86, 17], vec![83, 86,  6, 31, 17,  9, 48, 53]),
            (vec![13, 32, 20, 16, 61], vec![61, 30, 68, 82, 17, 32, 24, 19]),
            (vec![ 1, 21, 53, 59, 44], vec![69, 82, 63, 72, 16, 21, 14,  1]),
            (vec![41, 92, 73, 84, 69], vec![59, 84, 76, 51, 58,  5, 54, 83]),
            (vec![87, 83, 26, 28, 32], vec![88, 30, 70, 12, 93, 22, 82, 36]),
            (vec![31, 18, 13, 56, 72], vec![74, 77, 10, 23, 35, 67, 36, 11]),
        ]);
    }

    #[test]
    fn first_start() {
        let cards_sets = parse_input(TEST_INPUT);

        let result = cards_sets.iter().map(|(winning_number, number)| {
            BTreeSet::from_iter(winning_number.iter().cloned())
            .intersection(&BTreeSet::from_iter(number.iter().cloned()))
            .count()
        })
        .filter(|n| *n > 0)
        .map(|n| 2u32.pow(n as u32 - 1))
        .sum::<u32>();

        assert_eq!(result, 13);
    }

    #[test]
    fn second_star() {
        let cards_sets = parse_input(TEST_INPUT);

        let cards_matching_numbers_count = cards_sets.iter().map(|(winning_number, number)| {
            BTreeSet::from_iter(winning_number.iter().cloned())
            .intersection(&BTreeSet::from_iter(number.iter().cloned()))
            .count()
        }).collect::<Vec<_>>();

        let mut card_copies_count = vec![1; cards_sets.len()];

        cards_matching_numbers_count.iter().enumerate().for_each(|(index, matching_numbers_count)| {
            for i in 0..*matching_numbers_count {
                card_copies_count[index+i+1] += card_copies_count[index];
            }
        });

        let total_cards_count = card_copies_count.iter().sum::<u32>();

        assert_eq!(total_cards_count, 30);
        
    }

}