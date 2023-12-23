use std::collections::BTreeMap;

use itertools::Itertools;

use aoc_utils::nom_parsing::number;

use nom::{
    multi::{many_m_n, separated_list1}, 
    character::complete::{one_of, newline},
    combinator::map_res, IResult,
};

const INPUT: &str = aoc_utils::get_input!();

trait CardTypeTrait: Clone + PartialEq + Eq + PartialOrd + Ord + From<char> {}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum CardType {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack, // J
    Queen, // Q
    King, // K
    Ace, // A
}

impl CardTypeTrait for CardType {}

impl From<char> for CardType {
    fn from(c: char) -> Self {
        match c {
            'A' => CardType::Ace,
            'K' => CardType::King,
            'Q' => CardType::Queen,
            'J' => CardType::Jack,
            'T' => CardType::Ten,
            '9' => CardType::Nine,
            '8' => CardType::Eight,
            '7' => CardType::Seven,
            '6' => CardType::Six,
            '5' => CardType::Five,
            '4' => CardType::Four,
            '3' => CardType::Three,
            '2' => CardType::Two,
            _ => panic!("Invalid card"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum CardTypeJoker {
    Joker, // J
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Queen, // Q
    King, // K
    Ace, // A
}

impl CardTypeTrait for CardTypeJoker {}

impl From<char> for CardTypeJoker {
    fn from(c: char) -> Self {
        match c {
            'A' => CardTypeJoker::Ace,
            'K' => CardTypeJoker::King,
            'Q' => CardTypeJoker::Queen,
            'T' => CardTypeJoker::Ten,
            '9' => CardTypeJoker::Nine,
            '8' => CardTypeJoker::Eight,
            '7' => CardTypeJoker::Seven,
            '6' => CardTypeJoker::Six,
            '5' => CardTypeJoker::Five,
            '4' => CardTypeJoker::Four,
            '3' => CardTypeJoker::Three,
            '2' => CardTypeJoker::Two,
            'J' => CardTypeJoker::Joker,
            _ => panic!("Invalid card"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPairs,
    ThreeOfKind,
    FullHouse,
    FourOfKind,
    FiveOfKind,
}

impl From<[CardType; 5]> for HandType {
    fn from(cards: [CardType; 5]) -> Self {
        let mut cards_count: BTreeMap<&CardType, usize> = BTreeMap::new();
        cards.iter().for_each(|card|*cards_count.entry(card).or_insert(0) += 1);

        match cards_count.values().count() {
            5 => HandType::HighCard,
            4 => HandType::OnePair,
            3 => {
                if cards_count.values().any(|&x| x == 3) {
                    HandType::ThreeOfKind
                } else {
                    HandType::TwoPairs
                }
            },
            2 => {
                if cards_count.values().any(|&x| x == 4) {
                    HandType::FourOfKind
                } else {
                    HandType::FullHouse
                }
            },
            1 => HandType::FiveOfKind,
            _ => panic!("Invalid hand"),
        }
    }
}

impl From<[CardTypeJoker; 5]> for HandType {
    fn from(cards: [CardTypeJoker; 5]) -> Self {
        let mut cards_count: BTreeMap<&CardTypeJoker, usize> = BTreeMap::new();
        cards.iter().for_each(|card|*cards_count.entry(card).or_insert(0) += 1);
        
        if cards_count.contains_key(&CardTypeJoker::Joker) {
            let joker_count = cards_count[&CardTypeJoker::Joker];
            cards_count.iter_mut().for_each(|(_, count)| *count += joker_count);
            cards_count.remove(&CardTypeJoker::Joker);
        }

        match cards_count.values().count() {
            5 => HandType::HighCard,
            4 => HandType::OnePair,
            3 => {
                if cards_count.values().any(|&x| x == 3) {
                    HandType::ThreeOfKind
                } else {
                    HandType::TwoPairs
                }
            },
            2 => {
                if cards_count.values().any(|&x| x == 4) {
                    HandType::FourOfKind
                } else {
                    HandType::FullHouse
                }
            },
            1 => HandType::FiveOfKind,
            // it was five jokers
            0 => HandType::FiveOfKind,
            _ => panic!("Invalid hand"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Hand<TCardKind> {
    hand_type: HandType,
    cards_type: [TCardKind; 5],
}

fn vec_to_array<T, const N: usize>(v: Vec<T>) -> [T; N] {
    v.try_into()
        .unwrap_or_else(|v: Vec<T>| panic!("Expected a Vec of length {} but it was {}", N, v.len()))
}

fn parse_hand<TCardKind>(input: &str) -> IResult<&str, Hand<TCardKind>> 
where TCardKind: CardTypeTrait, HandType: From<[TCardKind; 5]> {
    map_res(
        many_m_n(5, 5, one_of("AKQJT98765432")),
        |cards| -> Result<Hand<TCardKind>, ()> {
            
            let cards_type = vec_to_array::<TCardKind, 5>(cards.into_iter().map(TCardKind::from).collect::<Vec<_>>());
            
            Ok(Hand {
                hand_type: HandType::from(cards_type.clone()),
                cards_type,
            })
        }
    )(input)
}

fn parse_hands_and_number<TCardKind>(input: &str) -> IResult<&str, Vec<(Hand<TCardKind>, u32)>> 
where TCardKind: CardTypeTrait, HandType: From<[TCardKind; 5]> {
    separated_list1(
        newline,
        map_res(
            nom::sequence::separated_pair(
                parse_hand,
                nom::character::complete::space1,
                number,
            ),
            |(hand, number)| -> Result<(Hand<TCardKind>, u32), ()> {
                Ok((hand, number))
            }
        )
    )(input)
}

fn main() {   
    let hands_and_number = parse_hands_and_number::<CardType>(INPUT).unwrap().1;

    let total_winnings = hands_and_number.into_iter()
    .sorted_by(|(hand1, _), (hand2, _)| hand1.cmp(hand2))
    .enumerate().map(|(i, (_, number))| (i+1) as u32 * number).sum::<u32>();

    println!("Total winnings: {}", total_winnings);

    let total_winnings_joker = parse_hands_and_number::<CardTypeJoker>(INPUT).unwrap().1.into_iter()
    .sorted_by(|(hand1, _), (hand2, _)| hand1.cmp(hand2))
    .enumerate().map(|(i, (_, number))| (i+1) as u32 * number).sum::<u32>();

    println!("Total winnings with joker: {}", total_winnings_joker);
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn parsing() {
        assert_eq!(parse_hand("32T3K").unwrap().1, Hand {
            hand_type: HandType::OnePair,
            cards_type: [CardType::Three, CardType::Two, CardType::Ten, CardType::Three, CardType::King],
        });

        assert_eq!(parse_hands_and_number(TEST_INPUT).unwrap().1, vec![
            (Hand {
                hand_type: HandType::OnePair,
                cards_type: [CardType::Three, CardType::Two, CardType::Ten, CardType::Three, CardType::King],
            }, 765),
            (Hand {
                hand_type: HandType::ThreeOfKind,
                cards_type: [CardType::Ten, CardType::Five, CardType::Five, CardType::Jack, CardType::Five],
            }, 684),
            (Hand {
                hand_type: HandType::TwoPairs,
                cards_type: [CardType::King, CardType::King, CardType::Six, CardType::Seven, CardType::Seven],
            }, 28),
            (Hand {
                hand_type: HandType::TwoPairs,
                cards_type: [CardType::King, CardType::Ten, CardType::Jack, CardType::Jack, CardType::Ten],
            }, 220),
            (Hand {
                hand_type: HandType::ThreeOfKind,
                cards_type: [CardType::Queen, CardType::Queen, CardType::Queen, CardType::Jack, CardType::Ace],
            }, 483),
        ]);

    }

    #[test]
    fn first_start() {
        let hands_and_number = parse_hands_and_number::<CardType>(TEST_INPUT).unwrap().1;

        let total_winnings = hands_and_number.into_iter()
        .sorted_by(|(hand1, _), (hand2, _)| hand1.cmp(hand2))
        .enumerate().map(|(i, (_, number))| (i+1) as u32 * number).sum::<u32>();

        assert_eq!(total_winnings, 6440);

    }
}