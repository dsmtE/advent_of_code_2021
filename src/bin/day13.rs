use itertools::Itertools;

const INPUT: &str = aoc_utils::get_input!();

use aoc_utils::{parse_char_grid};

// Store each row and column as a binary number, `#.##..##.` becomes `101100110`.
// This way we can easily check two rows or columns for equality using bitwise XOR and count the number of set bits.

fn parse_input(input: &str) -> Vec<(Vec<u32>, Vec<u32>)> {
    input
        .split("\n\n")
        .map(|s| parse_char_grid(s))
        .map(|grid| {
            let mut rows = Vec::with_capacity(grid.height as usize);
            let mut columns = Vec::with_capacity(grid.width as usize);

            for y in 0..grid.height {
                rows.push((0..grid.width).into_iter().fold(0u32, |n, x| (n << 1) | (grid[(x, y)] == '#') as u32));
            }

            for x in 0..grid.width {
                columns.push((0..grid.height).into_iter().fold(0u32, |n, y| (n << 1) | (grid[(x, y)] == '#') as u32));
            }

            (rows, columns)
    }).collect::<Vec<_>>()
}

fn reflect_on_axis_index(axis: &[u32], smudges_target_count: u32) -> Option<usize> {
    let size = axis.len();

    for i in 0..(size-1) {
        let mut smudges = 0;

        // Only consider rows/columns within the boundary of the grid.
        for j in 0..=i.min(size - i - 2) {
            smudges += (axis[i - j] ^ axis[i + j + 1]).count_ones();
        }

        if smudges == smudges_target_count {
            return Some(i);
        }
    }

    None
}

fn reflection_score(rows: &[u32], columns: &[u32], smudges_target_count: u32) -> usize {
    if let Some(x) = reflect_on_axis_index(columns, smudges_target_count) {
        x + 1
    } else if let Some(y) = reflect_on_axis_index(rows, smudges_target_count) {
        100 * (y + 1)
    } else {
        unreachable!()
    }
}

fn main() {
    let input = parse_input(INPUT);

    let first_start = input
    .iter()
    .map(|(rows, columns)| reflection_score(rows, columns, 0))
    .sum::<usize>();

    println!("First star: {}", first_start);

    let second_start = input
    .iter()
    .map(|(rows, columns)| reflection_score(rows, columns, 1))
    .sum::<usize>();

    println!("Second star: {}", second_start);
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

    #[test]
    fn parsing() {
        let input = parse_input(TEST_INPUT);
        assert_eq!(input[0].0, vec![
                0b101100110,
                0b001011010,
                0b110000001,
                0b110000001,
                0b001011010,
                0b001100110,
                0b101011010]
        );

        assert_eq!(input[1].0, vec![
                0b100011001,
                0b100001001,
                0b001100111,
                0b111110110,
                0b111110110,
                0b001100111,
                0b100001001]
        );
    }

    #[test]
    fn first_start() {
        let input = parse_input(TEST_INPUT);

        let result = input
        .iter()
        .map(|(rows, columns)| reflection_score(rows, columns, 0))
        .sum::<usize>();

        assert_eq!(result, 405);
    }

    #[test]
    fn second_start() {
        let input = parse_input(TEST_INPUT);

        let result = input
        .iter()
        .map(|(rows, columns)| reflection_score(rows, columns, 1))
        .sum::<usize>();

        assert_eq!(result, 400);
    }

}