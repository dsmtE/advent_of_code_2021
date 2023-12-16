use itertools::Itertools;

const INPUT: &str = aoc_utils::get_input!();

fn parse_input(input: &str) -> Vec<(usize, usize)> {
    let mut coordinate_set = Vec::new();
    
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                coordinate_set.push((x as usize, y as usize));
            }
        }
    }
    coordinate_set
}

fn universe_size(universe: &Vec<(usize, usize)>) -> (usize, usize) {
    universe.iter().fold((0, 0), |(max_x, max_y), (x, y)| {
        (max_x.max(*x), max_y.max(*y))
    })
}

fn expended_row_and_cols(universe: &Vec<(usize, usize)>) -> (Vec<usize>, Vec<usize>) {
    let (width, height) = universe_size(universe);

    let mut rows = Vec::new();
    let mut cols = Vec::new();

    for x in 0..=width {
        if universe.iter().all(|(x2, _)| x != *x2) {
            cols.push(x);
        }
    }
    for y in 0..=height {
        if universe.iter().all(|(_, y2)| y != *y2) {
            rows.push(y);
        }
    }

    (rows, cols)
}

fn sum_galaxies_shortest_path_between_pairs(universe: &Vec<(usize, usize)>, universe_expansion: usize) -> usize {
    let (rows, cols) = expended_row_and_cols(universe);
    
    universe.iter()
        .combinations(2)
        // pair as vec to tuple
        .map(|vec| (vec[0], vec[1]))
        // map distance between pairs
        .map(|((x1, y1), (x2, y2))| {
        x1.max(x2) - x1.min(x2) + 
        y1.max(y2) - y1.min(y2) + 
        (cols.iter().filter(|c| (x1.min(x2)..=x1.max(x2)).contains(c) ).count() +
        rows.iter().filter(|r| (y1.min(y2)..=y1.max(y2)).contains(r) ).count()) * universe_expansion.saturating_sub(1).max(1)
    }).sum()
}

fn main() {
    let input = parse_input(INPUT);

    let first_star = sum_galaxies_shortest_path_between_pairs(&input, 1);

    println!("First star: {}", first_star);

    let second_star = sum_galaxies_shortest_path_between_pairs(&input, 1000000);

    println!("Second star: {}", second_star);
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    #[test]
    fn first_start() {

        let galaxies_coordinates: Vec<(usize, usize)> = parse_input(TEST_INPUT);
        assert_eq!(sum_galaxies_shortest_path_between_pairs(&galaxies_coordinates, 1), 374);
    }

    #[test]
    fn second_start() {
        let galaxies_coordinates: Vec<(usize, usize)> = parse_input(TEST_INPUT);
        assert_eq!(sum_galaxies_shortest_path_between_pairs(&galaxies_coordinates, 10), 1030);
        assert_eq!(sum_galaxies_shortest_path_between_pairs(&galaxies_coordinates, 100), 8410);
    }

}