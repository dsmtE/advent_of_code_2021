advent_of_code::solution!(18);

use aoc_utils::{
    cartesian::{p2, Cartesian, Cartesian2, Point2, Vector2}, graphs::astar::astar, grid::Grid
};

fn parse_input(input: &str) -> Vec<Point2> {
    input.lines().map(|line| {
        let (x, y) = line.split_once(",").unwrap();
        p2(x.parse().unwrap(), y.parse().unwrap())
    }).collect()
}

fn parse_corruption_grid(coordinates: &Vec<Point2>, size: usize) -> Grid<Option<usize>> {
    let mut grid = Grid {
        data: vec![None; size * size],
        width: size,
        height: size,
    };

    for (i, coord) in coordinates.iter().enumerate() {
        grid[coord] = Some(i);
    }
    
    grid
}

fn compute_path(grid: &Grid<Option<usize>>) -> Option<Vec<Point2>> {
    let target_pos = p2(grid.width as i64 - 1, grid.height as i64 - 1);
    astar(
        [p2(0, 0)],
        |&pos|
        Vector2::ORTHOGONAL.iter()
            .map(|&dir| (pos + dir, 1))
            .filter(|(pos, _)| grid.in_bound(pos) && grid[pos].is_none())
            .collect::<Vec<_>>(),
        |&pos| (pos - target_pos).manhattan_dist(),
    |&pos| pos == target_pos).map(|(path, _)| path)
}

fn part_one(input: &str) -> Option<usize> {
    let coordinates = parse_input(input);
    let grid = parse_corruption_grid(&coordinates.into_iter().take(1024).collect(), 71);

    Some(compute_path(&grid).unwrap().len())
}

fn find_first_blocking_coordinate(coordinates: &Vec<Point2>, grid: &Grid<Option<usize>>) -> Option<Point2> {
    // flood fill from start util all cells possible are reached
    // iterate thought obstacles from reverse order removing them one by one
    // and flood fill again to find the first blocking coordinate

    let target_pos = p2(grid.width as i64 - 1, grid.height as i64 - 1);
    let mut to_visit = vec![p2(0, 0)];
    let mut current_time = coordinates.len();
    let mut seen = grid.same_size_with(false);
    
    loop {
        // println!("current time {}", current_time);
        let target_reached = flood_fill(&mut to_visit, &mut seen, &grid, current_time, target_pos);

        if target_reached {
            break;
        }

        if current_time == 0 {
            panic!("No path found");
        }

        // debug_display_flood_fill(&grid, &seen);

        current_time -= 1;
        let last_removed_coordinate = coordinates[current_time];
        println!("removing last coordinate at {:?} (time {})", last_removed_coordinate, current_time);

        if Vector2::ORTHOGONAL.into_iter()
        .map(|dir| last_removed_coordinate + dir)
        .filter(|pos| grid.in_bound(pos))
        .any(|pos| seen[&pos]) {
            to_visit.push(last_removed_coordinate);
        }
    }

    // debug_display_flood_fill(&grid, &seen, Some(current_time));

    (current_time < coordinates.len()).then(|| coordinates[current_time])
}

// return true if target_pos is reachable from start_pos
fn flood_fill(to_visit: &mut Vec<Point2>, seen: &mut Grid<bool>, grid: &Grid<Option<usize>>, time: usize, target_pos: Point2) -> bool {
    while let Some(pos) = to_visit.pop() {
        if seen[&pos] {
            continue;
        }
        seen[&pos] = true;

        if pos == target_pos {
            return true;
        }

        Vector2::ORTHOGONAL.into_iter()
            .map(|dir| pos + dir)
            .filter(|pos| grid.in_bound(pos) && grid[pos].map_or(true, |x| x > time))
            .for_each(|neighbor_pos| to_visit.push(neighbor_pos));
    }
    
    false
}

#[allow(dead_code)]
fn debug_display_flood_fill(grid: &Grid<Option<usize>>, seen: &Grid<bool>, current_time: Option<usize>) {
    for y in 0..(grid.height as _) {
        for x in 0..(grid.width as _) {
            let pos = p2(x, y);
            if seen[&pos] {
                print!("o");
            } else if let Some(x) = grid[&pos] {
                if current_time.map_or(true, |current_time| x < current_time) {
                    print!("X");
                } else {
                    print!(" ");
                }
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

fn part_two(input: &str) -> Option<String> {
    let coordinates = parse_input(input);
    let grid = parse_corruption_grid(&coordinates, 71);
    find_first_blocking_coordinate(&coordinates, &grid).map(|pos| format!("{},{}", pos.x(), pos.y()))
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "5,4\n4,2\n4,5\n3,0\n2,1\n6,3\n2,4\n1,5\n0,6\n3,3\n2,6\n5,1\n1,2\n5,5\n2,5\n6,5\n1,4\n0,4\n6,4\n1,1\n6,1\n1,0\n0,5\n1,6\n2,0";

    #[test]
    fn test_parse_input() {
        assert_eq!(parse_input(TEST_INPUT), vec![
            p2(5, 4),p2(4, 2),p2(4, 5),p2(3, 0),p2(2, 1),p2(6, 3),p2(2, 4),
            p2(1, 5),p2(0, 6),p2(3, 3),p2(2, 6),p2(5, 1),p2(1, 2),p2(5, 5),
            p2(2, 5),p2(6, 5),p2(1, 4),p2(0, 4),p2(6, 4),p2(1, 1),p2(6, 1),
            p2(1, 0),p2(0, 5),p2(1, 6),p2(2, 0),
        ]);
    }

    #[test]
    fn test_parse_corruption_grid() {
        let coordinates = parse_input(TEST_INPUT);
        let grid = parse_corruption_grid(&coordinates.into_iter().take(12).collect(), 7);
        assert_eq!(grid.data, vec![
            None, None, None, Some(3), None, None, None, 
            None, None, Some(4), None, None, Some(11), None, 
            None, None, None, None, Some(1), None, None, 
            None, None, None, Some(9), None, None, Some(5),
            None, None, Some(6), None, None, Some(0), None, 
            None, Some(7), None, None, Some(2), None, None, 
            Some(8), None, Some(10), None, None, None, None,
        ]);
    }

    #[test]
    fn test_part_one() {
        let coordinates = parse_input(TEST_INPUT);
        let grid = parse_corruption_grid(&coordinates.into_iter().take(12).collect(), 7);

        let path = compute_path(&grid);
        assert!(path.is_some(), "Path not found");
        let path = path.unwrap();

        assert_eq!(path.len(), 22);
        let path_ids = path.iter().map(|pos| pos.x() + pos.y() * grid.width as i64).collect::<Vec<_>>();
        // skip the first 3 elements bas they could be multiple values [0, 7, 14] or [0, 1, 8] or [0, 7, 8]
        assert_eq!(path_ids.into_iter().skip(3).collect::<Vec<_>>(), vec![15, 16, 17, 10, 11, 4, 5, 6, 13, 20, 19, 26, 25, 32, 31, 38, 45, 46, 47]);
    }

    #[test]
    fn test_part_two() {
        let coordinates = parse_input(TEST_INPUT);
        let grid = parse_corruption_grid(&coordinates, 7);
        let result = find_first_blocking_coordinate(&coordinates, &grid);
        assert_eq!(result, Some(p2(6, 1)));
    }
}