const INPUT: &str = aoc_utils::get_input!();

use aoc_utils::{
    parse_grid,
    Grid,
    to_decimal,
    Direction,
    cartesian::{p2, Point2, Cartesian2},
    dijkstra::dijkstra,
};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct CrucibleState {
    pos: Point2,
    dir: Direction,
    moves_in_dir: u8,
}

fn successors(state: &CrucibleState, grid: &Grid<u32>, min_straight_line: u8, max_straight_line: u8) -> Vec<(CrucibleState, u32)> {
    let (rows, cols) = (grid.height as i64, grid.width as i64);

    let mut successors = Vec::new();
    for dir in [
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ] {

        // We must move at least min_straight_line tiles in the same direction
        if state.moves_in_dir < min_straight_line && dir != state.dir {
            continue;
        }

        if dir == state.dir && state.moves_in_dir >= max_straight_line {
            // if we moved more than max_straight_line tiles in a straight line, we can't move further
            continue;
        }

        if state.dir.opposite() == dir {
            // can't move in opposite direction
            continue;
        }

        let new_pos = state.pos + dir.vector();

        if new_pos.x() < 0 || new_pos.x() >= cols || new_pos.y() < 0 || new_pos.y() >= rows {
            // out of bounds
            continue;
        }

        // increment straight_moves if we went straight, else we moved 1 tile in the current direction
        let moves_in_dir = if state.dir == dir {
            state.moves_in_dir + 1
        } else {
            1
        };
        
        successors.push((
            CrucibleState {
                pos: new_pos,
                dir,
                moves_in_dir,
            },
            grid[(new_pos.x() as usize, new_pos.y() as usize)],
        ));
    }

    successors
}


fn parse_input(input: &str) -> Grid<u32> {
    parse_grid(input, |c| to_decimal(c).unwrap())
}

fn solve_heat_path_cost(heat_loss_grid: &Grid<u32>, min_straight_line: u8, max_straight_line: u8) -> Option<u32>{
    let end_pos = p2(heat_loss_grid.width as i64 - 1, heat_loss_grid.height as i64 - 1);

    let result = dijkstra(
        vec![
            CrucibleState {
                pos: p2(0, 0),
                dir: Direction::Right,
                moves_in_dir: 1,
            },
            CrucibleState {
                pos: p2(0, 0),
                dir: Direction::Down,
                moves_in_dir: 1,
            },
        ],
        |state| successors(state, &heat_loss_grid, min_straight_line, max_straight_line),
        |state| state.pos == end_pos,
    );

    if let Some((_, cost)) = result {
        return Some(cost);
    }
    return None;
}

fn main() {
    let heat_loss_grid = parse_input(INPUT);

    let first_start = solve_heat_path_cost(&heat_loss_grid, 0, 3);

    println!("Part 1: {:?}", first_start);

    let second_start = solve_heat_path_cost(&heat_loss_grid, 4, 10);

    println!("Part 2: {:?}", second_start);
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533
";

    #[test]
    fn first_start() {
        let heat_loss_grid = parse_input(TEST_INPUT);
        assert_eq!(solve_heat_path_cost(&heat_loss_grid, 0, 3), Some(102));
    }

    #[test]
    fn second_start() {
        let heat_loss_grid = parse_input(TEST_INPUT);
        assert_eq!(solve_heat_path_cost(&heat_loss_grid, 4, 10), Some(94));
    }
    
    
}