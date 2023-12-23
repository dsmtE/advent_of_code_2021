use nom::{
    sequence::{preceded, tuple},
    multi::separated_list1, 
    bytes::complete::tag,
    character::complete::{multispace1, digit1, newline},
    combinator::map_res, IResult,
};

use aoc_utils::nom_parsing::numbers_list;

const INPUT: &str = aoc_utils::get_input!();

#[derive(Debug, Clone, PartialEq, Eq)]
struct Race {
    distance: u64,
    time: u64,
}

fn get_ways_count(race: Race) -> usize {
    // TODO: optim not compute all possibility but only from bounds (as it will follow a triangle of possibility)
    (0..race.time).map(move |i| i * (race.time-i) ).filter(|x| x > &race.distance).count()
}

fn number_with_whitespace<T: std::str::FromStr>(input: &str) -> IResult<&str, T>
    where T::Err: std::fmt::Debug {
    map_res(
        map_res(
            separated_list1(multispace1, digit1),
            |x: Vec<&str>| -> Result<String, ()> { Ok(x.join("")) }
        ),
        |x: String| -> Result<T, T::Err> { Ok(x.parse::<T>().unwrap()) }
    )(input)
}

fn parse_input(input: & str) -> Vec<Race> {
    let (_, (times, distances)) = 
        tuple((
            preceded(tuple((tag("Time:"), multispace1)), numbers_list::<u64>),
            preceded(tuple((newline, tag("Distance:"), multispace1)), numbers_list::<u64>),
        ))(input).unwrap();

    std::iter::zip(times.into_iter(), distances.into_iter()).map(|(time, distance)| Race { time, distance }).collect()
}

fn parse_input_as_one_race(input: & str) -> Race {
    let (_, (time, distance)) = 
        tuple((
            preceded(tuple((tag("Time:"), multispace1)), number_with_whitespace::<u64>),
            preceded(tuple((newline, tag("Distance:"), multispace1)), number_with_whitespace::<u64>),
        ))(input).unwrap();

    Race { time, distance }
}

fn main() {   
    let races = parse_input(INPUT);

    let ways_of_beating_races = races.into_iter().map(get_ways_count).collect::<Vec<_>>();

    println!("product: {}", ways_of_beating_races.iter().product::<usize>());
    
    let unique_race = parse_input_as_one_race(INPUT);

    println!("ways_count of unique race: {}", get_ways_count(unique_race));
}

#[cfg(test)]
mod tests {

    use super::*;

    const TEST_INPUT: &str = "Time:      7  15   30\nDistance:  9  40  200";

    #[test]
    fn parsing() {
        let races = parse_input(TEST_INPUT);

        assert_eq!(races, vec![
            Race { time: 7, distance: 9 },
            Race { time: 15, distance: 40 },
            Race { time: 30, distance: 200 },
        ]);

        assert_eq!(number_with_whitespace::<u64>("42  123  5"),  Ok(("", 421235)));

        assert_eq!(parse_input_as_one_race(TEST_INPUT), Race { time: 71530, distance: 940200 });
    }

    #[test]
    fn first_start() {
        let races = parse_input(TEST_INPUT);

        let ways_of_beating_races = races.into_iter().map(get_ways_count).collect::<Vec<_>>();

        assert_eq!(ways_of_beating_races, vec![4, 8, 9]);
    }

    #[test]
    fn second_star() {

        let race = parse_input_as_one_race(TEST_INPUT);

        println!("race: {:?}", race);

        assert_eq!(get_ways_count(race), 71503);

    }

}