const INPUT: &str = aoc_utils::get_input!();

use num::Integer;

#[derive(Debug, PartialEq, Eq)]
struct Map <T>{
    data: Vec<T>,
    width: usize,
    height: usize,
}

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


impl<T, I> std::ops::Index<(I, I)> for Map<T>
where I: Integer, usize: From<I> {
    type Output = T;

    fn index(&self, idx: (I, I)) -> &T {
        // or as appropriate for row- or column-major data       
        &self.data[usize::from(idx.1) * self.width + usize::from(idx.0)]
    }
}

impl<T> std::ops::Index<Position<i64>> for Map<T> {
    type Output = T;

    fn index(&self, pos: Position<i64>) -> &T {
        // or as appropriate for row- or column-major data       
        &self.data[pos.y as usize * self.width + pos.x as usize]
    }
}

impl<T> std::ops::Index<usize> for Map<T> {
    type Output = T;

    fn index(&self, idx: usize) -> &T {
        &self.data[idx]
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

fn parse_input(input: &str) -> Map<char> {
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();
    let map = input.lines().flat_map(|line| line.chars()).collect::<Vec<_>>();
    return Map { data: map, width, height };
}


fn main() {
    let map = parse_input(INPUT);

    let starting_position = map.data.iter().enumerate().find(|(_, &c)| c == 'S').map(|(i, _)| Position { x: (i % map.width) as i64, y: (i / map.width) as i64 }).unwrap();
    println!("{:?}", starting_position);

    //find pipes connected to starting point
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

        get_neigbours_offset(map[neighbor_position]).iter().for_each(|&offset| {
            if (neighbor_position + Position { x: offset.0, y: offset.1 }) == starting_position {
                connected_pipes.push(neighbor_position);
            }
        });
    }

    let mut previous_position = starting_position;
    let mut next_position = connected_pipes.first().unwrap().clone();
    let mut count = 1;

    while(next_position != starting_position) {
        let bli = get_neigbours_offset(map[next_position])
            .iter()
            .map(|&offset| next_position + Position { x: offset.0, y: offset.1 })
            .filter(|&pos| pos != previous_position).next().unwrap();

        previous_position = next_position;
        next_position = bli;
        count += 1;
    }
    
    println!("Part 1: {}", count/2);
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
        assert_eq!(parse_input(TEST_INPUT), Map {
            data: "..F7..FJ|.SJ.L7|F--JLJ...".chars().collect::<Vec<_>>(),
            width: 5,
            height: 5,
        });
    }

    #[test]
    fn first_start() {

        let map = parse_input(TEST_INPUT);

        let starting_position = map.data.iter().enumerate().find(|(_, &c)| c == 'S').map(|(i, _)| Position { x: (i % map.width) as i64, y: (i / map.width) as i64 }).unwrap();
        println!("{:?}", starting_position);

        //find pipes connected to starting point
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

            get_neigbours_offset(map[neighbor_position]).iter().for_each(|&offset| {
                if (neighbor_position + Position { x: offset.0, y: offset.1 }) == starting_position {
                    connected_pipes.push(neighbor_position);
                }
            });
        }

        let mut previous_position = starting_position;
        let mut next_position = connected_pipes.first().unwrap().clone();
        let mut count = 1;

        while(next_position != starting_position) {
            let bli = get_neigbours_offset(map[next_position])
                .iter()
                .map(|&offset| next_position + Position { x: offset.0, y: offset.1 })
                .filter(|&pos| pos != previous_position).next().unwrap();

            previous_position = next_position;
            next_position = bli;
            count += 1;
        }
        
        assert_eq!(count/2, 8);
    }

}