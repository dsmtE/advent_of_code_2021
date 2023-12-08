use nom::{
    sequence::{preceded, tuple, terminated},
    multi::separated_list1, 
    bytes::complete::{tag, take_until},
    character::complete::newline,
    combinator::map_res, IResult,
};

use aoc_utils::nom_helpers::{number, numbers_list};

use itertools::Itertools;
use std::collections::VecDeque;

const INPUT: &str = aoc_utils::get_input!();

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct RangeMapping {
    to: u64,
    from: u64,
    length: u64,
}

impl std::fmt::Display for RangeMapping {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}..{} -> {}..{}", self.from, self.from + self.length, self.to, self.to + self.length)

        // write!(f,"{} {} {}", self.to, self.from, self.length)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Almanac {
    seeds: Vec<u64>,
    conversion_maps: Vec<Vec<RangeMapping>>,
}

impl std::fmt::Display for Almanac {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "seeds: {}", self.seeds.iter().map(|s| s.to_string()).collect::<Vec<_>>().join(" "))?;
        for (i, conversion_map) in self.conversion_maps.iter().enumerate() {
            writeln!(f, "{} map:", match i {
                0 => "seed-to-soil",
                1 => "soil-to-fertilizer",
                2 => "fertilizer-to-water",
                3 => "water-to-light",
                4 => "light-to-temperature",
                5 => "temperature-to-humidity",
                6 => "humidity-to-location",
                _ => panic!("Unknown conversion map")
            })?;
            for mapping in conversion_map {
                writeln!(f, "{}", mapping)?;
            }
        }
        Ok(())
    }
}

fn parse_seeds(input: &str) -> IResult<&str, Vec<u64>> {
    preceded(tag("seeds: "),  numbers_list)(input)
}

fn parse_range_mapping_list(input: &str) -> IResult<&str, Vec<RangeMapping>> {
    preceded(
        tuple((
            take_until("map:"),
            tag("map:"),
            newline
        )),
        separated_list1(
                newline, 
                parse_range_mapping
            )
    )(input)

}

fn parse_range_mapping(input: &str) -> IResult<&str,RangeMapping> {
    map_res(
        tuple((
            number,
            preceded(tag(" "), number),
            preceded(tag(" "), number),
        )),
        |(to, from, length)| -> Result<RangeMapping, ()> { Ok(RangeMapping { length, from, to }) }
    )(input)
}

fn parse_input(input: & str) -> Almanac {
    map_res(
        tuple((
            terminated(parse_seeds, newline),
            separated_list1(newline, preceded(newline, parse_range_mapping_list))
        )),
        |(seeds, conversion_maps)| -> Result<Almanac, ()> { Ok(Almanac { seeds, conversion_maps }) }
    )(input).unwrap().1
}

fn main() {   
    let almanac = parse_input(INPUT);

    let min_locations_on_seed = almanac.conversion_maps.iter().fold(almanac.seeds.clone(), |mut seeds, conversion_map| {
        seeds.iter_mut().for_each(|seed| {
            for mapping in conversion_map {
                if (mapping.from..(mapping.from + mapping.length)).contains(seed) {
                    *seed = *seed + mapping.to - mapping.from;
                    break;
                }
            }
        });
        seeds
    }).into_iter().min().unwrap();

    println!("min_locations_on_seed: {:?}", min_locations_on_seed);

    let min_locations_on_seeds_range = almanac.conversion_maps.iter().fold(
        almanac.seeds.clone().into_iter().tuples().map(|(a, b)| a..(a+b)).collect::<VecDeque<_>>(),
        |mut seeds, conversion_map| {
            let mut new_seeds = VecDeque::new();
            while let Some(seed) = seeds.pop_front() {
                let mut mapping_found = false;
                for mapping in conversion_map {
                    if (mapping.from..mapping.from + mapping.length).contains(&seed.start) {
                        if seed.end <= mapping.from + mapping.length {
                            new_seeds.push_back((mapping.to + seed.start - mapping.from)..(mapping.to + seed.end - mapping.from));
                        }else {
                            new_seeds.push_back((mapping.to + seed.start - mapping.from)..(mapping.to + mapping.length));
                            // push back the rest of the seed
                            seeds.push_back((mapping.from + mapping.length)..seed.end);
                        }
                        mapping_found = true;
                        break;
                    } else if(mapping.from..mapping.from + mapping.length).contains(&(seed.end-1)) {
                        // Mean that seed.start is before mapping.from
                        new_seeds.push_back(mapping.to..(mapping.to + seed.end - mapping.from));
                        // push back the rest of the seed
                        seeds.push_back(seed.start..mapping.from);
                        mapping_found = true;
                        break;
                    }
                }
                // if we didn't find a mapping for that range, keep it unchanged
                if !mapping_found {
                    new_seeds.push_back(seed);
                }
            }
        new_seeds
    }).iter().map(|range| range.start).min().unwrap();

    println!("min_locations_on_seeds_range: {:?}", min_locations_on_seeds_range);

}

#[cfg(test)]
mod tests {

    use super::*;

    const TEST_INPUT: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn parsing() {

        assert_eq!(number("123"), Ok(("", 123)));
        assert_eq!(parse_seeds("seeds: 79 14 55 13"), Ok(("", vec![79, 14, 55, 13])));
        assert_eq!(parse_range_mapping("50 98 2"), Ok(("", RangeMapping { length: 2, from: 98, to: 50 })));
        assert_eq!(parse_range_mapping_list("seed-to-soil map:\n50 98 2\n52 50 48"), Ok(("", vec![
            RangeMapping { length: 2, from: 98, to: 50 },
            RangeMapping { length: 48, from: 50, to: 52 },
        ])));

        let almanac = parse_input(TEST_INPUT);

        assert_eq!(almanac, Almanac{
            seeds: vec![79, 14, 55, 13],
            conversion_maps: vec![
                vec![
                    RangeMapping { to: 50, from: 98, length: 2 },
                    RangeMapping { to: 52, from: 50, length: 48 },
                ],
                vec![
                    RangeMapping { to: 0, from: 15, length: 37 },
                    RangeMapping { to: 37, from: 52, length: 2 },
                    RangeMapping { to: 39, from: 0, length: 15 },
                ],
                vec![
                    RangeMapping { to: 49, from: 53, length: 8 },
                    RangeMapping { to: 0, from: 11, length: 42 },
                    RangeMapping { to: 42, from: 0, length: 7 },
                    RangeMapping { to: 57, from: 7, length: 4 },
                ],
                vec![
                    RangeMapping { to: 88, from: 18, length: 7 },
                    RangeMapping { to: 18, from: 25, length: 70 },
                ],
                vec![
                    RangeMapping { to: 45, from: 77, length: 23 },
                    RangeMapping { to: 81, from: 45, length: 19 },
                    RangeMapping { to: 68, from: 64, length: 13 },
                ],
                vec![
                    RangeMapping { to: 0, from: 69, length: 1 },
                    RangeMapping { to: 1, from: 0, length: 69 },
                ],
                vec![
                    RangeMapping { to: 60, from: 56, length: 37 },
                    RangeMapping { to: 56, from: 93, length: 4 },
                ],
            ],
        });
    }

    #[test]
    fn first_start() {
        let almanac = parse_input(TEST_INPUT);

        let locations = almanac.conversion_maps.iter().fold(almanac.seeds.clone(), |mut seeds, conversion_map| {
            seeds.iter_mut().for_each(|seed| {
                for mapping in conversion_map {
                    if (mapping.from..(mapping.from + mapping.length)).contains(seed) {
                        *seed = *seed + mapping.to - mapping.from;
                        break;
                    }
                }
            });
            println!("seeds: {:?}", seeds);
            seeds
        });

        assert_eq!(locations, vec![82, 43, 86, 35]);

        println!("min locations: {:?}", locations.iter().min().unwrap());

    }

    #[test]
    fn second_star() {
        let almanac = parse_input(TEST_INPUT);

        let min_locations = almanac.conversion_maps.iter().fold(
            almanac.seeds.clone().into_iter().tuples().map(|(a, b)| a..(a+b)).collect::<VecDeque<_>>(),
            |mut seeds, conversion_map| {
                let mut new_seeds = VecDeque::new();
                while let Some(seed) = seeds.pop_front() {
                    let mut mapping_found = false;
                    for mapping in conversion_map {
                        if (mapping.from..mapping.from + mapping.length).contains(&seed.start) {
                            if seed.end <= mapping.from + mapping.length {
                                new_seeds.push_back((mapping.to + seed.start - mapping.from)..(mapping.to + seed.end - mapping.from));
                            }else {
                                new_seeds.push_back((mapping.to + seed.start - mapping.from)..(mapping.to + mapping.length));
                                // push back the rest of the seed
                                seeds.push_back((mapping.from + mapping.length)..seed.end);
                            }
                            mapping_found = true;
                            break;
                        } else if(mapping.from..mapping.from + mapping.length).contains(&(seed.end-1)) {
                            // Mean that seed.start is before mapping.from
                            new_seeds.push_back(mapping.to..(mapping.to + seed.end - mapping.from));
                            // push back the rest of the seed
                            seeds.push_back(seed.start..mapping.from);
                            mapping_found = true;
                            break;
                        }
                    }
                    // if we didn't find a mapping for that range, keep it unchanged
                    if !mapping_found {
                        new_seeds.push_back(seed);
                    }
                }
            new_seeds
        }).iter().map(|range| range.start).min().unwrap();
        
        println!("min locations: {:?}", min_locations);
        assert_eq!(min_locations, 46);

    }

}