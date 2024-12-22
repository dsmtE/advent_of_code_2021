use aoc_utils::{cartesian::{Direction, Point2, p2}, grid::Grid, graphs::dijkstra::dijkstra};
use itertools::Itertools;

advent_of_code::solution!(16);

fn parse_grid_start_end(input: &str) -> (Grid<bool>, Point2, Point2) {
    let char_grid = input
        .lines()
        .flat_map(|line| line.chars())
        .collect::<Vec<_>>();

    let height = input.lines().count();
    let width = input.lines().next().unwrap().len();

    let start_index = char_grid.iter().position(|&c| c == 'S').unwrap();
    let end_index = char_grid.iter().position(|&c| c == 'E').unwrap();

    let grid = Grid {
        data: char_grid.iter().map(|&c| c == '#').collect(),
        width,
        height,
    };

    let start = grid.index_to_point(start_index);
    let end = grid.index_to_point(end_index);

    (grid, start, end)
}

fn get_sucessors(grid: &Grid<bool>, pos: Point2, dir: Direction) -> impl Iterator<Item = ((Point2, Direction), u32)> {
    let forward_pos = pos + dir.vector();
    [
        Some(((pos, dir.turn_left()), 1000)),
        Some(((pos, dir.turn_right()), 1000)),
        (!grid[&forward_pos]).then_some(((forward_pos, dir), 1)),
    ].into_iter().flatten()
}

fn part_one(input: &str) -> Option<u32> {
    let (grid, start_pos, end_pos) = parse_grid_start_end(input);

    let dijkstra_result = dijkstra(
        [(start_pos, Direction::Right)],
        |&(pos, dir)| {
            get_sucessors(&grid, pos, dir)
        },
        |&(pos, _)| pos == end_pos,
    );

    dijkstra_result.map(|(_, cost)| cost)
}

fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";

    #[test]
    fn test_parse_input() {
        let (grid, start, end) = parse_grid_start_end(TEST_INPUT);

        assert_eq!(grid.width, 15);
        assert_eq!(grid.height, 15);
        assert_eq!(start, p2(1, 13));
        assert_eq!(end, p2(13, 1));
    }

    #[test]
    fn test_part_one() {
        let (grid, start_pos, end_pos) = parse_grid_start_end(TEST_INPUT);

        let dijkstra_result = dijkstra(
            [(start_pos, Direction::Right)],
            |&(pos, dir)| {
                get_sucessors(&grid, pos, dir)
            },
            |&(pos, _)| pos == end_pos,
        );

        assert_eq!(dijkstra_result.unwrap().1, 7036);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(TEST_INPUT);
        assert_eq!(result, None);
    }
}