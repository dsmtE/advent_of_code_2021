use std::collections::HashSet;
use std::ops::RangeInclusive;

use nom::bytes::complete::tag;
use nom::character::complete::{digit1, line_ending};
use nom::combinator::{map, map_res, opt, recognize};

use nom::sequence::preceded;
use nom::{
    character::complete,
    multi::separated_list1,
    sequence::{pair, separated_pair},
    IResult,
};

const INPUT: &str = advent_of_code::get_input!();

use advent_of_code::iterator_to_string;

type Coord = (i32, i32);

#[derive(Clone, Copy, Debug)]
struct SensorInfo {
    sensor: Coord,
    beacon: Coord,
}

fn manhattan_distance(a: Coord, b: Coord) -> u32 { a.0.abs_diff(b.0) + a.1.abs_diff(b.1) }

impl SensorInfo {
    pub fn range_at_line(&self, line: i32) -> Result<RangeInclusive<i32>, &'static str> {
        let max_dist = manhattan_distance(self.sensor, self.beacon);
        let shift_y_needed = self.sensor.1.abs_diff(line);
        if shift_y_needed > max_dist {
            return Err("out of reach");
        }

        let deviation = (max_dist - shift_y_needed) as i32;
        Ok((self.sensor.0 - deviation)..=(self.sensor.0 + deviation))
    }
}

impl std::fmt::Display for SensorInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Sensor at x={}, y={}: closest beacon is at x={}, y={}",
            self.sensor.0, self.sensor.1, self.beacon.0, self.beacon.1
        )
    }
}

fn number<T: std::str::FromStr>(input: &str) -> IResult<&str, T> {
    map_res(
        recognize(pair(opt(complete::char('-')), digit1)),
        str::parse::<T>,
    )(input)
}

fn parse_coord(input: &str) -> IResult<&str, Coord> {
    separated_pair(
        preceded(tag("x="), number::<i32>),
        tag(", "),
        preceded(tag("y="), number::<i32>),
    )(input)
}

fn parse_line(input: &str) -> IResult<&str, SensorInfo> {
    map(
        pair(
            preceded(tag("Sensor at "), parse_coord),
            preceded(tag(": closest beacon is at "), parse_coord),
        ),
        |(a, b)| SensorInfo {
            sensor: a,
            beacon: b,
        },
    )(input)
}

fn parse(input: &str) -> IResult<&str, Vec<SensorInfo>> {
    separated_list1(line_ending, parse_line)(input)
}

fn count_empty_positions_at_line(sensor_infos: &[SensorInfo], wanted_line: i32) -> usize {
    let mut result = sensor_infos
        .into_iter()
        .filter_map(|info| info.range_at_line(wanted_line).map(|x| Some(x)).unwrap_or(None))
        .flatten()
        .collect::<HashSet<i32>>();
    
    // remove beacon positions
    sensor_infos.into_iter().for_each(|info| {
        if info.beacon.1 == wanted_line {
            result.remove(&info.beacon.0);
        }
    });

    result.len()
}

fn ranges_at_line(sensor_infos: &[SensorInfo], wanted_line: i32) -> Vec<RangeInclusive<i32>> {
    let mut ranges = sensor_infos
        .iter()
        .flat_map(|info| info.range_at_line(wanted_line))
        .collect::<Vec<_>>();
    ranges.sort_unstable_by_key(|range| *range.start());

    let mut merged_ranges = vec![ranges[0].clone()];
    
    for range in &ranges[1..] {
        let last_idx = merged_ranges.len() - 1;
        let last = &merged_ranges[last_idx];
        // check if the two sorted ranges overlap
        if *range.start() <= last.end() + 1 {
            // replace last with a single bigger range if possible
            if range.end() > last.end() {
                merged_ranges[last_idx] = *last.start()..=*range.end();
            }
        } else {
            // add to the ranges for this row
            merged_ranges.push(range.clone());
        }
    }

    merged_ranges
}

fn find_empty_position_in_range(sensor_infos: &[SensorInfo], range: RangeInclusive<i32>) -> Result<i64, &'static str> {

    let (y, col_ranges) = range.map(|y| (y, ranges_at_line(sensor_infos, y)))
        // if there is more than one range covering the row, there is a gap    
        .find(|(_, ranges)| ranges.len() > 1)
        .expect("Unable to find a line with gaps");

    // find gap location in the given row
    let x = col_ranges.first().unwrap().end() + 1;

    Ok(tuning_frequency(&(x, y)))
}

fn tuning_frequency(pos: &Coord) -> i64 { 4000000 * pos.0 as i64 + pos.1 as i64 }

fn main() {
    let (_, sensor_infos) = parse(INPUT).unwrap();

    println!("count_empty_positions_at_line :{}", count_empty_positions_at_line(&sensor_infos, 2000000));
    println!("distress beacon tuning frequency: {}", find_empty_position_in_range(&sensor_infos, 0..=4000000).unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

    #[test]
    fn parsing_test() {
        let (_, sensor_infos) = parse(TEST_INPUT).unwrap();
        println!("{}", iterator_to_string(&sensor_infos, "\n"));
    }

    #[test]
    fn simple_case() {
        let (_, sensor_infos) = parse(TEST_INPUT).unwrap();

        assert_eq!(count_empty_positions_at_line(&sensor_infos, 10), 26);
        assert_eq!(find_empty_position_in_range(&sensor_infos, 0..=20).unwrap(), 56000011);
    }
}
