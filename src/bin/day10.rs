const INPUT: &str = aoc_utils::get_input!();

use num::Integer;

use aoc_utils::{Grid, parse_char_grid};


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Position<T> {
    x: T,
    y: T,
}

impl<T> std::ops::Add for Position<T>
where T: std::ops::Add<Output = T> {
    type Output = Position<T>;

    fn add(self, other: Position<T>) -> Position<T> {
        Position { x: self.x + other.x, y: self.y + other.y }
    }
}

impl<T> std::ops::AddAssign for Position<T>
where T: std::ops::Add<Output = T> + Copy {
    fn add_assign(&mut self, other: Position<T>) {
        *self = Position { x: self.x + other.x, y: self.y + other.y };
    }
}

impl<T> From<Position<T>> for (T, T) {
    fn from(pos: Position<T>) -> Self {
        (pos.x, pos.y)
    }
}

impl<T> Into<Position<T>> for (T, T) {
    fn into(self) -> Position<T> {
        Position { x: self.0, y: self.1 }
    }
}

fn get_neigbours_offset(pipe_type: char) -> Vec<(i64, i64)> {
    match pipe_type {
        '|' => vec![(0, -1), (0, 1)],
        'J' => vec![(-1, 0), (0, -1)],
        'F' => vec![(1, 0), (0, 1)],
        'L' => vec![(1, 0), (0, -1)],
        '7' => vec![(-1, 0), (0, 1)],
        '-' => vec![(-1, 0), (1, 0)],
        '.' => vec![],
        _ => panic!("Invalid pipe type"),
    }
}

fn starting_point_neigbours_position(starting_position: Position<i64>, map: &Grid<char>) -> Vec<Position<i64>> {
    let mut connected_pipes = Vec::new();
    for (offset_x, offset_y) in vec![( 0i64, -1i64 ), ( 0, 1 ), ( -1, 0 ), ( 1, 0 )] {
        // out of bounds
        if  starting_position.x as i64 + offset_x < 0 || 
            starting_position.x as i64 + offset_x >= map.width as i64 ||
            starting_position.y as i64 + offset_y < 0 ||
            starting_position.y as i64 + offset_y >= map.height as i64 {
            continue;
        }

        let neighbor_position = starting_position + Position { x: offset_x, y: offset_y};

        get_neigbours_offset(map[(neighbor_position.x as usize, neighbor_position.y as usize)]).iter().for_each(|&offset| {
            if (neighbor_position + Position { x: offset.0, y: offset.1 }) == starting_position {
                connected_pipes.push(neighbor_position);
            }
        });
    }

    connected_pipes
}

fn get_loop_position(starting_position: Position<i64>, map: &Grid<char>) -> Vec<Position<i64>> {
    let starting_point_neigbours_position = starting_point_neigbours_position(starting_position, &map);

    let mut loop_position = vec![starting_position];

    let mut next_pos = starting_point_neigbours_position.first().unwrap().clone();

    while next_pos != starting_position {
        let last_pos = loop_position.last().unwrap().clone();
        let new_pos = get_neigbours_offset(map[(next_pos.x as usize, next_pos.y as usize)])
            .iter()
            .map(|&offset| next_pos + Position { x: offset.0, y: offset.1 })
            .filter(|&pos| pos != last_pos).next().unwrap();

        loop_position.push(next_pos);
        next_pos = new_pos;
    }

    loop_position
}

fn loop_area(loop_position: &[Position<i64>]) -> isize {

    // Shoelace formula
    let mut area: isize = 0;
    for w in loop_position.windows(2) {
        area += (w[0].y * w[1].x) as isize;
        area -= (w[0].x * w[1].y) as isize;
    }

    // last and first closing the loop
    area += (loop_position.last().unwrap().y * loop_position.first().unwrap().x) as isize;
    area -= (loop_position.last().unwrap().x * loop_position.first().unwrap().y) as isize;

    isize::abs(area) / 2
}

fn main() {
    let map = parse_char_grid(INPUT);

    let starting_position = map.data.iter().enumerate().find(|(_, &c)| c == 'S').map(|(i, _)| Position { x: (i % map.width) as i64, y: (i / map.width) as i64 }).unwrap();
    let loop_position = get_loop_position(starting_position, &map);
    
    println!("Part 1: {}", loop_position.len()/2);

    let tiles_count = loop_area(&loop_position) - (loop_position.len()as isize / 2) + 1;

    println!("Part 2: {}", tiles_count);

}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";

    #[test]
    fn parsing() {
        assert_eq!(parse_char_grid(TEST_INPUT), Grid {
            data: "..F7..FJ|.SJ.L7|F--JLJ...".chars().collect::<Vec<_>>(),
            width: 5,
            height: 5,
        });
    }

    #[test]
    fn first_start() {

        let map = parse_char_grid(TEST_INPUT);

        let starting_position = map.data.iter().enumerate().find(|(_, &c)| c == 'S').map(|(i, _)| Position { x: (i % map.width) as i64, y: (i / map.width) as i64 }).unwrap();
        let loop_position = get_loop_position(starting_position, &map);

        assert_eq!(loop_position.len()/2, 8);
    }

    #[test]
    fn second_start() {


        const TEST_INPUT_2: &str = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";

        let map = parse_char_grid(TEST_INPUT_2);

        let starting_position = map.data.iter().enumerate().find(|(_, &c)| c == 'S').map(|(i, _)| Position { x: (i % map.width) as i64, y: (i / map.width) as i64 }).unwrap();
        let loop_position = get_loop_position(starting_position, &map);

        //find number of tiles inside
        let tiles_count = loop_area(&loop_position) - (loop_position.len()as isize / 2) + 1;

        assert_eq!(tiles_count, 8)
        
    }

}