advent_of_code::solution!(12);

use std::collections::HashSet;

use aoc_utils::{cartesian::{p2, Point2, Vector2, Cartesian2}, parse_char_grid, Grid};

pub fn parse_input(input: &str) -> Grid<char> {
    parse_char_grid(input)
}

#[derive(Debug, PartialEq)]
struct GardenPlot {
    label: char,
    points: HashSet<Point2>,
    edges: Vec<(Point2, Vector2)>,
}

impl GardenPlot {
    pub fn area (&self) -> usize {
        self.points.len()
    }
    pub fn perimeter (&self) -> usize {
        self.edges.len()
    }


    // To get the number of sides of a garden area,
    // we attribute weight to each edge of the plot.
    // The total number of side is this weight sum divided by 2.

    // A edge contribute to zero if it has two edges neighbors in the same direction.
    // A edge contribute to one if it has one neighbor and two if it has none.

    // exemple up direction

    // 1 0 1    _ 1 1  _ 2 _
    // ^ ^ ^      ^ ^    ^   	
    // . . .    . . .  . . .
    // # # #    . # #  . # .

    // We must also count twice if the edge is in a corner

    // . _ . . . In this case, there is 8 edges, 4 and the two 
    // | # | _ . upper right edges must be count as two and not one 
    // | #   # | (as if i only check left and right neighbors) 
    // . _ . _ . for that reason, in the case the edge has a neighbor on the left
    // . . . . . i must also check for the upper left diagonal is also in the plot

    // Complexe Test case with right 
    // +-+
    // |C|
    // + +-+
    // |C C|
    // +-+ +
    //   |C|
    //   +-+

    // +2+   -> - Total weight 12
    // 1C1      - 12 / 2 = 6 sides
    // + +1+    - it's wrong because edges in corners only weight one
    // 1C C1
    // +1+ +
    //   1C1
    //   +2+

    // +2+   -> - Total weight 16
    // 1C2      - 16 / 2 = 8 sides
    // + +2+    - it's correct because edges check for diagonal
    // 1C C1
    // +2+ +
    //   2C1
    //   +2+


    pub fn sides_count(&self) -> usize {
        let mut sides_weight = 0;
        for (point, direction) in self.edges.iter() {
            let left = *point + direction.rotate_left();
            let right = *point + direction.rotate_right();
            let left_diagonal = left + *direction;
            let right_diagonal = right + *direction;

            if !self.points.contains(&left) || self.points.contains(&left_diagonal) {
                sides_weight += 1;
            }
            if !self.points.contains(&right) || self.points.contains(&right_diagonal) {
                sides_weight += 1;
            }
        }

        sides_weight / 2
    }
}

fn plots_from_grid(grid : &Grid<char>) -> Vec<GardenPlot> {

    let mut seen = grid.same_size_with(false);

    let mut plots = Vec::new();

    for y in 0..grid.height {
        for x in 0..grid.width {
            // Skip already seen points
            let point = p2(x as _, y as _);
            if seen[&point] {
                continue;
            }
            
            let mut plot = GardenPlot {
                label: grid[&point],
                points: HashSet::new(),
                edges: Vec::new(),
            };

            let mut flood_fill_to_visit = vec![point];
            seen[&point] = true;

            while let Some(current_point) = flood_fill_to_visit.pop() {
                plot.points.insert(current_point);
                
                for direction in Vector2::ORTHOGONAL {
                    let orthogonal_neighbor = current_point + direction;
                    if grid.in_bound(&orthogonal_neighbor) && grid[&orthogonal_neighbor] == plot.label {
                        if !seen[&orthogonal_neighbor] {
                            flood_fill_to_visit.push(orthogonal_neighbor);
                            seen[&orthogonal_neighbor] = true;
                        }
                    }
                    else {
                        plot.edges.push((current_point, direction));
                    }
                }
            }

            plots.push(plot);
        }
    }

    plots
}

pub fn part_one(input: &str) -> Option<usize> {
    let grid = parse_input(input);

    let plots = plots_from_grid(&grid);

    let total_price = plots.iter()
        .map(|plot| plot.area() * plot.perimeter())
        .sum::<usize>();
    Some(total_price)
}

pub fn part_two(input: &str) -> Option<usize> {
    let grid = parse_input(input);

    let plots = plots_from_grid(&grid);

    let total_price = plots.iter()
        .map(|plot| plot.area() * plot.sides_count())
        .sum::<usize>();
    Some(total_price)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "AAAA
BBCD
BBCC
EEEC";

    #[test]
    fn test_parse_input() {
        let result = parse_input(TEST_INPUT);
        assert_eq!(result, Grid {
            data: "AAAABBCDBBCCEEEC".chars().collect(),
            width: 4,
            height: 4,
        });
    }

    #[test]
    fn test_flood_fill_areas() {
        let grid = parse_input(TEST_INPUT);
        let areas = plots_from_grid(&grid);

        assert_eq!(areas.len(), 5);
        assert_eq!((areas[0].label, areas[0].area(), areas[0].perimeter()), ('A', 4, 10));
        assert_eq!((areas[1].label, areas[1].area(), areas[1].perimeter()), ('B', 4, 8));
        assert_eq!((areas[2].label, areas[2].area(), areas[2].perimeter()), ('C', 4, 10));
        assert_eq!((areas[3].label, areas[3].area(), areas[3].perimeter()), ('D', 1, 4));
        assert_eq!((areas[4].label, areas[4].area(), areas[4].perimeter()), ('E', 3, 8));
    }

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(TEST_INPUT), Some(140));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(TEST_INPUT), Some(80));
    }
}