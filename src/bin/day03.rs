use std::collections::HashMap;

const INPUT: &str = advent_of_code_2023::get_input!();

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Part {
    line_index: usize,
    /// x range within the line
    x_range: core::ops::Range<usize>,
    number: u32,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Symbol {
    map_index: usize,
    symbol: char,
}

fn get_nieghbors_symbols_from_map(map: &[char], part: &Part, width: usize) -> Vec<Symbol> {
    get_nieghbors_index(&part.x_range, part.line_index, width).iter()
            .map(|index| (*index, map[*index])).filter(|(_, c)| *c != '.').map(|(index, c)| Symbol { map_index: index, symbol: c }).collect()
}

fn get_nieghbors_index(x_range_index: &core::ops::Range<usize>, y: usize, width: usize) -> Vec<usize> {
    let mut neighbors = Vec::new();
    let (start_x, end_x) = (x_range_index.start, x_range_index.end);

    let extended_x_range = start_x.saturating_sub(1)..std::cmp::min(end_x+1, width);

    if y > 0 {
        for x in extended_x_range.clone() {
            neighbors.push((y - 1) * width + x);
        }
    }

    if y < width - 1 {
        for x in extended_x_range {
            neighbors.push((y + 1) * width + x);
        }
    }

    if start_x > 0 {
        neighbors.push(y * width + start_x - 1);
    }

    if end_x < width - 1 {
        neighbors.push(y * width + end_x);
    }
    
    neighbors

    // TODO: rewrite using chain and map

}

// return an iterator over the digits of a string and this digit's range index
fn digit_range_iterator(string: &str) -> impl Iterator<Item = (u32, core::ops::Range<usize>)> + '_ {
    let mut index = 0;
    std::iter::from_fn(move || {
        let sub_string = &string[index..];

        sub_string.chars().clone().enumerate().find(|(_, c)| c.is_ascii_digit()).and_then(|(first_index, _)| {
            let (number, count) = sub_string[first_index..].chars().take_while(|c| c.is_ascii_digit())
            .fold((0, 0),
            |(number, count), c| (number * 10 + c.to_digit(10).unwrap(), count + 1));
            index += count + first_index;
            Some((number, index-count..index))
        })
    })
}

fn parse_parts<'a>(input: &'a str) -> impl Iterator<Item = Part> + 'a {
    input
        .lines()
        .enumerate()
        .flat_map(move |(y, line)| {
            digit_range_iterator(line).map(move |(number, x_range_index)|
                Part { line_index: y, x_range: x_range_index, number }
            )
        })
}

fn main() {
    let lines = INPUT.lines();
    let width = lines.clone().next().unwrap().len();
    let map = lines.clone().flat_map(|line| line.chars()).collect::<Vec<_>>();
    let parts = parse_parts(INPUT).collect::<Vec<_>>();

    let part_numbers_sum: u32 = parts.iter().filter(|&part| {
        get_nieghbors_index(&part.x_range, part.line_index, width).iter()
            .map(|index| map[*index])
            .any(|c| c != '.')

    })
    .map(|part| part.number)
    .sum();

    println!("First star: {}", part_numbers_sum);

    let mut gears: HashMap<Symbol, Vec<u32>> = HashMap::new();

    parts
        .iter()
        .map(|part| (part, get_nieghbors_symbols_from_map(&map, part, width)))
        .filter(|(_, neighbors_symbols)| neighbors_symbols.len() > 0)
        .map(|(part, neighbors_symbols)| {
            (
                part,
                neighbors_symbols.into_iter().filter(|symbol| symbol.symbol == '*').collect::<Vec<_>>()
            )
        }).for_each(|(part, neighbors_gears)| {
            for gear in neighbors_gears.into_iter() {
                gears.entry(gear.clone()).or_insert(Vec::new()).push(part.number);
            }
        });
    
    let sum_gear_ratios = gears.values()
        .filter(|numbers| numbers.len() == 2)
        .map(|numbers| numbers.iter().product::<u32>())
        .sum::<u32>();

    println!("Second star: {}", sum_gear_ratios);
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn parsing() {
        let parts = parse_parts(TEST_INPUT).collect::<Vec<_>>();

        for part in parts.iter() {
            println!("{}: {:?} ({})", part.number, part.x_range, part.line_index);
        }

        assert_eq!(parts, vec![
            Part { line_index: 0, x_range: 0..3, number: 467 },
            Part { line_index: 0, x_range: 5..8, number: 114 },
            Part { line_index: 2, x_range: 2..4, number: 35 },
            Part { line_index: 2, x_range: 6..9, number: 633 },
            Part { line_index: 4, x_range: 0..3, number: 617 },
            Part { line_index: 5, x_range: 7..9, number: 58 },
            Part { line_index: 6, x_range: 2..5, number: 592 },
            Part { line_index: 7, x_range: 6..9, number: 755 },
            Part { line_index: 9, x_range: 1..4, number: 664 },
            Part { line_index: 9, x_range: 5..8, number: 598 },
        ]);
    }

    #[test]
    fn first_start() {
        let lines = TEST_INPUT.lines();
        let width = lines.clone().next().unwrap().len();
        let map = lines.clone().flat_map(|line| line.chars()).collect::<Vec<_>>();
        let parts = parse_parts(TEST_INPUT).collect::<Vec<_>>();

        let result: u32 = parts.iter().filter(|&part| {
            get_nieghbors_index(&part.x_range, part.line_index, width).iter()
                .map(|index| map[*index])
                .any(|c| c != '.')

        })
        .map(|part| part.number)
        .sum();

        assert_eq!(result, 4361);
    }

    #[test]
    fn second_star() {
        let lines = TEST_INPUT.lines();
        let width = lines.clone().next().unwrap().len();
        let map = lines.clone().flat_map(|line| line.chars()).collect::<Vec<_>>();
        let parts = parse_parts(TEST_INPUT).collect::<Vec<_>>();

        let mut gears: HashMap<Symbol, Vec<u32>> = HashMap::new();

        parts
            .iter()
            .map(|part| (part, get_nieghbors_symbols_from_map(&map, part, width)))
            .filter(|(_, neighbors_symbols)| neighbors_symbols.len() > 0)
            .map(|(part, neighbors_symbols)| {
                (
                    part,
                    neighbors_symbols.into_iter().filter(|symbol| symbol.symbol == '*').collect::<Vec<_>>()
                )
            }).for_each(|(part, neighbors_gears)| {
                for gear in neighbors_gears.into_iter() {
                    gears.entry(gear.clone()).or_insert(Vec::new()).push(part.number);
                }
            });
        
        let sum_gear_ratios = gears.values()
            .filter(|numbers| numbers.len() == 2)
            .map(|numbers| numbers.iter().product::<u32>())
            .sum::<u32>();

        assert_eq!(sum_gear_ratios, 467835);
    }

}