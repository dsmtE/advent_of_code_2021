advent_of_code::solution!(10);

use aoc_utils::{cartesian::{Point2, Vector2}, parse_grid, Grid};

use pathfinding::prelude::bfs_reach;

pub fn trailhead_score(height_map: &Grid<u32>, start: &Point2) -> u32 {
    bfs_reach(
        *start,
        |&current_pos| {
            Vector2::ORTHOGONAL
            .into_iter()
            .map(move |offset| offset + current_pos)
            .filter(|new_pos| height_map.in_bound(new_pos))
            .filter(move |new_pos| height_map[new_pos] == height_map[&current_pos] + 1)
            // .inspect(move |new_pos| println!("{:?} -> {:?}", current_pos, new_pos))
    })
    .collect::<Vec<_>>()
    .iter()
    .filter(|&p| height_map[p] == 9)
    .count() as u32
}

pub fn parse_input(input: &str) -> Grid<u32> {
    parse_grid(input, |c| c.to_digit(10).unwrap())
}

pub fn part_one(input: &str) -> Option<u32> {
    let height_map = parse_input(input);

    let trailheads_score_sum: u32 = height_map.data.iter()
        .enumerate()
        .filter(|(_, &v)| v == 0)
        .map(|(i, _)| height_map.index_to_point(i))
        // .inspect(|p| print!("{:?} -> ", p))
        .map(|p| trailhead_score(&height_map, &p))
        // .inspect(|score| println!("{score}"))
        .sum();


    Some(trailheads_score_sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let height_map = parse_input(input);

    let mut count_grid: Grid<u32> = height_map.clone();
    count_grid.data.fill(0);

    let positions_by_height = (0..=9).map(|i| 
            height_map.data.iter()
                .enumerate()
                .filter(|(_, &v)| v == i)
                .map(|(i, _)| height_map.index_to_point(i))
                .collect::<Vec<_>>()
    ).collect::<Vec<_>>();

    positions_by_height[9].iter().for_each(|p| count_grid[p] = 1);

    for height  in (0..=8).rev() {
        for position in &positions_by_height[height] {
            count_grid[position] = Vector2::ORTHOGONAL
                .into_iter()
                .map(|offset| offset + *position)
                .filter(|new_pos| height_map.in_bound(new_pos))
                .filter(|new_pos| height_map[new_pos] == height as u32 + 1)
                .map(|new_pos| count_grid[&new_pos])
                .sum::<u32>();
        }
    }

    Some(positions_by_height[0].iter().map(|p| count_grid[p]).sum::<u32>())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(TEST_INPUT), Some(36));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(TEST_INPUT), Some(81));
    }
}