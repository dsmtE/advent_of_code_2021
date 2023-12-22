const INPUT: &str = aoc_utils::get_input!();

use std::collections::{BinaryHeap, HashSet};

use aoc_utils::{
    parse_grid,
    Grid,
    to_decimal,
    Direction,
    cartesian::{p2, Point2, Cartesian2},
};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct CrucibleState {
    pos: Point2,
    dir: Direction,
    moves_in_dir: u8,
}

struct StateAndCost<T> {
    state: T,
    cost: u32,
}

impl<T> PartialEq for StateAndCost<T> {
    fn eq(&self, other: &Self) -> bool { self.cost == other.cost }
}

impl<T> Eq for StateAndCost<T> {}

impl<T> PartialOrd for StateAndCost<T> {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> { Some(self.cmp(other)) }
}

impl<T> Ord for StateAndCost<T>  {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering { other.cost.cmp(&self.cost) }
}

fn successors(state_and_cost: &StateAndCost<CrucibleState>, grid: &Grid<u32>, min_straight_line: u8, max_straight_line: u8) -> Vec<StateAndCost<CrucibleState>> {
    let (rows, cols) = (grid.height as i64, grid.width as i64);

    let (state, cost) = (state_and_cost.state, state_and_cost.cost);

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

        let new_cost = cost + grid[(new_pos.x() as usize, new_pos.y() as usize)];

        // increment straight_moves if we went straight, else we moved 1 tile in the current direction
        let moves_in_dir = if state.dir == dir {
            state.moves_in_dir + 1
        } else {
            1
        };
        
        successors.push(StateAndCost {
            state: CrucibleState {
                pos: new_pos,
                dir,
                moves_in_dir,
            },
            cost: new_cost,
        });
    }

    successors
}


fn parse_input(input: &str) -> Grid<u32> {
    parse_grid(input, |c| to_decimal(c).unwrap())
}

fn solve_heat_path_cost(heat_loss_grid: &Grid<u32>, min_straight_line: u8, max_straight_line: u8) -> Option<u32>{
    // Custom dijkstra implementation without path reconstruction
    let (width, height) = (heat_loss_grid.width, heat_loss_grid.height);
    let end_pos = p2(width as i64 - 1, height as i64 - 1);

    let mut pq = BinaryHeap::new();
    let mut seen = HashSet::new();

    pq.push(StateAndCost {
        state: CrucibleState {
            pos: p2(1, 0),
            dir: Direction::Right,
            moves_in_dir: 1,
        },
        cost: heat_loss_grid[(1, 0)],
    });

    pq.push(StateAndCost {
        state: CrucibleState {
            pos: p2(0, 1),
            dir: Direction::Down,
            moves_in_dir: 1,
        },
        cost: heat_loss_grid[(0, 1)],
    });

    while let Some(StateAndCost { state, cost }) = pq.pop() {
        if state.pos == end_pos {
            return Some(cost);
        }
        for successor in successors(&StateAndCost { state, cost }, &heat_loss_grid, min_straight_line, max_straight_line) {
            if seen.insert((successor.state.pos, successor.state.dir, successor.state.moves_in_dir)) {
                pq.push(successor);
            }
        }
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