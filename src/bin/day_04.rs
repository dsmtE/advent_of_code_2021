advent_of_code::solution!(4);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Position(i32, i32);

type Id = i32;

impl std::ops::Add for Position {
    type Output = Position;

    fn add(self, other: Position) -> Position {
        Position(self.0 + other.0, self.1 + other.1)
    }
}

impl std::ops::Mul<i32> for Position {
    type Output = Position;

    fn mul(self, other: i32) -> Position {
        Position(self.0 * other, self.1 * other)
    }
}

pub fn parse_input(input: &str) -> ((usize, usize),Vec<char>) {
    // return width and height of the grid and the grid as a vector of chars
    let height = input.lines().count();
    let width = input.lines().next().unwrap().chars().count();
    let grid = input.lines().flat_map(|line| line.chars()).collect();
    ((width, height), grid)
}

impl Position {
    fn in_bounds(&self, width: i32, height: i32) -> bool {
        self.0 >= 0 && self.0 < width && self.1 >= 0 && self.1 < height
    }
}

fn id_to_position(id: Id, width: i32) -> Position {
    Position(id % width, id / width)
}

fn position_to_id(position: Position, width: i32) -> Id {
    position.0 + position.1 * width
}

pub fn part_one(input: &str) -> Option<usize> {
    let ((width, height), grid) = parse_input(input);

    let width_as_i32 = width as i32;
    let height_as_i32 = height as i32;
    // Find all XMAS world in the grid
    let mut count: usize = 0;
    grid.iter().enumerate().filter_map(|(id, &c)| if c == 'X' { Some(id_to_position(id as _, width_as_i32)) } else {None} ).for_each(|x_pos| {
        for pos_offset in [(-1, 0), (1, 0), (0, -1), (0, 1), (-1, -1), (1, -1), (-1, 1), (1, 1)].map(|(x, y)| Position(x,y)).iter() {
            // map to if and value in the (None if out of bounds)
            let found: bool = "MAS".chars()
                .enumerate()
                .map(|(i, c)| (x_pos + *pos_offset * (i as i32 + 1), c))
                .all(|(pos,c)| pos.in_bounds(width_as_i32, height_as_i32) && grid[position_to_id(pos, width_as_i32) as usize] == c);

            if found {
                count += 1;
            }
        }
    });

    Some(count)
    // 2483
}

pub fn part_two(input: &str) -> Option<usize> {
    let ((width, height), grid) = parse_input(input);
    let width_as_i32 = width as i32;
    let height_as_i32 = height as i32;

    let mut count: usize = 0;
    grid.iter().enumerate().filter_map(|(id, &c)| if c == 'A' { Some(id_to_position(id as _, width_as_i32)) } else {None} ).for_each(|a_pos| {
        // corner positions in clockwise order
        let corners_positions = [(-1, -1), (1, -1), (1, 1), (-1, 1)]
            .map(|(x, y)| a_pos + Position(x,y));
        
        if corners_positions.iter().any(|p| !p.in_bounds(width_as_i32, height_as_i32)) {
            return;
        }
        
        let corners_characters = corners_positions
            .iter()
            .map(|p| grid[position_to_id(*p, width_as_i32) as usize])
            .collect::<Vec<char>>();

        for i in 0..4 {
            if corners_characters.iter().cycle().skip(i).take(4).collect::<String>() == "MMSS" {
                count += 1;
                break;
            }
        }
    });

    Some(count)
    // 1925
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn test_parse_input() {
        let ((width, height), grid) = parse_input(TEST_INPUT);
        assert_eq!((width, height), (10, 10));
        assert_eq!(grid.len(), 100);
        assert_eq!(grid, "MMMSXXMASMMSAMXMSMSAAMXSXMAAMMMSAMASMSMXXMASAMXAMMXXAMMXXAMASMSMSASXSSSAXAMASAAAMAMMMXMMMMMXMXAXMASX".chars().collect::<Vec<char>>());
    }

    #[test]
    fn test_part_one() {
        let ((_, _), grid) = parse_input(TEST_INPUT);

        let x_id: Vec<Id> = grid.iter().enumerate().filter_map(|(id, &c)| if c == 'X' { Some(id as Id) } else {None} ).collect();
        assert_eq!(x_id, vec![4, 5, 14, 22, 24, 39, 40, 46, 50, 51, 55, 56, 67, 72, 85, 91, 93, 95, 99]);

        assert_eq!(part_one(TEST_INPUT), Some(18));
    }
    

    #[test]
    fn test_part_two() {
        assert_eq!(part_one(TEST_INPUT), Some(9));
    }
}