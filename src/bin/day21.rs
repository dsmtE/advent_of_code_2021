const INPUT: &str = aoc_utils::get_input!();
use std::collections::BTreeSet;

use aoc_utils::{parse_char_grid, iterator::IterUtils, Grid, in_bound_cardinal_neighbors_index};
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;

fn reachable_neighbors(position_index: usize, map: &Grid<char>) -> Vec<usize> {
    in_bound_cardinal_neighbors_index(position_index, map.width, map.height)
    .into_iter()
    .filter(|&i| map.data[i] != '#')
    .collect_vec()
}

fn reachable_neighbors_from_pos(starting_pos_index: usize, map: &Grid<char>, step_count: usize) -> Vec<usize> {
    let mut positions_index = vec![starting_pos_index];
    for _ in 0..step_count {
        positions_index = 
            positions_index
            .par_iter()
            .flat_map(|&pos| reachable_neighbors(pos, &map))
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect_vec();
    }
    positions_index    
}

fn main() {
    let map = parse_char_grid(INPUT);
        let starting_pos_index = map.data.iter().enumerate().find(|(_, &c)| c == 'S').unwrap().0;
        let reachable_neighbors_count_at_step = reachable_neighbors_from_pos(starting_pos_index, &map, 64).len();
        println!("{}", reachable_neighbors_count_at_step);
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";

    #[test]
    fn first_start() {
        let map = parse_char_grid(TEST_INPUT);
        let starting_pos_index = map.data.iter().enumerate().find(|(_, &c)| c == 'S').unwrap().0;

        assert_eq!(reachable_neighbors_from_pos(starting_pos_index, &map, 6).len(), 16);
    }
}