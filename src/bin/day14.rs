use itertools::Itertools;

use aoc_utils::{index_to_point, in_bound};

const INPUT: &str = aoc_utils::get_input!();

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Tile {
    Round,
    Square,
    Empty,
}

use aoc_utils::{parse_grid, Grid, cartesian::{Direction, Point2}};

fn weight(grid: &Grid<Tile>) -> usize {
    let  (w, h) = (grid.width, grid.height);
    grid.data.iter()
        .enumerate()
        .filter(|&(_, tile)| *tile == Tile::Round)
        .map(|(i, _)| h - i / w)
        .sum()
}

fn parse_input(input: &str) -> Grid<Tile> {
    parse_grid(input, |c| match c {
            'O' => Tile::Round,
            '#' => Tile::Square,
            '.' => Tile::Empty,
            _ => unreachable!(),
        })
}

fn move_tiles(grid: &mut Grid<Tile>, dir: Direction) {
    let (width, height) = (grid.width, grid.height);
    loop {
        let mut changed = false;

        for pos in grid.data.iter()
            .enumerate()
            .filter(|&(_, tile)| *tile == Tile::Round)
            .map(|(index, _)| index_to_point(index, width))
            .collect_vec() {
                let mut pos = pos;
                let mut next = pos + dir.vector();
                
                while in_bound(&next, width, height) && grid[&next] == Tile::Empty {
                    grid[&pos] = Tile::Empty;
                    grid[&next] = Tile::Round;
                    pos = next;
                    next += dir.vector();
                    changed = true;
                }
            };

        if !changed {
            break;
        }
    }
}

fn main() {

    let grid = parse_input(INPUT);


    {
        let mut grid = grid.clone();
        move_tiles(&mut grid, Direction::Up);

        println!("First star: {}", weight(&grid));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

    #[test]
    fn first_start() {
        let grid = parse_input(TEST_INPUT);

        let mut grid = grid.clone();
        move_tiles(&mut grid, Direction::Up);

        assert_eq!(weight(&grid), 136);
    }
}