use std::collections::{HashMap, HashSet, VecDeque, BinaryHeap};

use nom::InputIter;

const INPUT: &str = advent_of_code::get_input!();
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    fn neighbours(&self) -> [Coord; 4] {
        [
            Coord { x: self.x - 1, y: self.y },
            Coord { x: self.x + 1, y: self.y },
            Coord { x: self.x, y: self.y + 1 },
            Coord { x: self.x, y: self.y - 1 },

        ]
    }
}

type Blizzard = (Coord, Direction);

#[derive(Debug)]
struct BlizzardMap {
    size: Coord,
    blizzards_on_row: Vec<Vec<Blizzard>>,
    blizzards_on_col: Vec<Vec<Blizzard>>,
}
impl BlizzardMap {
    fn new(blizzards: &HashMap<Coord, Direction>, size: Coord) -> BlizzardMap {

        let blizzards_on_row = (0..size.y)
            .map(|y| {
                blizzards.iter()
                    .filter(|(coord, _)| coord.y == y)
                    .filter(|(_, direction)|  {
                        **direction == Direction::Right || **direction == Direction::Left
                    })
                    .map(|(coord, direction)| (*coord, *direction))
                    .collect::<Vec<Blizzard>>()
                })
            .collect::<Vec<Vec<Blizzard>>>();

        let blizzards_on_col = (0..size.x)
            .map(|x| {
                blizzards.iter()
                    .filter(|(coord, _)| coord.x == x)
                    .filter(|(_, direction)|  {
                        **direction == Direction::Up || **direction == Direction::Down
                    })
                    .map(|(coord, direction)| (*coord, *direction))
                    .collect::<Vec<Blizzard>>()
                })
            .collect::<Vec<Vec<Blizzard>>>();
        BlizzardMap {
            size,
            blizzards_on_row,
            blizzards_on_col,
        }
    }

    // get all blitzards that are at the given coord at the given time
    fn get_at(&self, coord: Coord, time: usize) -> Vec<&Blizzard> {
        let (width, height) = (self.size.x, self.size.y);
        let time = time as i32;
        self.blizzards_on_row[coord.y as usize].iter()
            .filter(|(initial_coord, direction)| {
                coord.x == match direction {
                    Direction::Left => (initial_coord.x - time + width*height) % width,
                    Direction::Right => (initial_coord.x + time + width*height) % width,
                    _ => panic!("Unexpected direction"),
                }
            })
            .chain(
                self.blizzards_on_col[coord.x as usize].iter()
                    .filter(|(initial_coord, direction)| {
                        coord.y == match direction {
                            Direction::Up => (initial_coord.y - time + width*height) % height,
                            Direction::Down => (initial_coord.y + time + width*height) % height,
                            _ => panic!("Unexpected direction"),
                        }
                    })
            )
            .collect::<Vec<&Blizzard>>()
    }

    fn empty_at(&mut self, coord: Coord, time: usize) -> bool {
        let time = time % (self.size.x * self.size.y) as usize;
        self.get_at(coord, time).is_empty()
    }

    fn display_at(&self, time: usize) {
        for _ in 0..self.size.x + 2 { print!("#"); }
        println!("");
        for y in 0..self.size.y {
            print!("#");
            for x in 0..self.size.x {
                let blizzard_at = self.get_at(Coord { x, y }, time);
                
                if blizzard_at.is_empty() {
                    print!(".");
                    continue;
                }
                let len = blizzard_at.len();

                if len > 1 {
                    print!("{}", len);
                    continue;
                }
                
                print!("{}", match blizzard_at[0].1 {
                    Direction::Up => "^",
                    Direction::Down => "v",
                    Direction::Left => "<",
                    Direction::Right => ">",
                });
            }
            print!("#");
            println!("");
        }
        for _ in 0..self.size.x + 2 { print!("#"); }
        println!("");
    }
}

impl std::fmt::Display for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl std::fmt::Debug for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl From<char> for Direction {
    fn from(input: char) -> Direction {
        match input {
            '^' => Direction::Up,
            'v' => Direction::Down,
            '>' => Direction::Right,
            '<' => Direction::Left,
            _   => panic!("Unable to parse Direction from char"),
        }
    }
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::Up => write!(f, "^"),
            Direction::Down => write!(f, "v"),
            Direction::Left => write!(f, "<"),
            Direction::Right => write!(f, ">"),
        }
    }
}

impl std::fmt::Debug for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}" , self)
    }
}

fn parse(input: &str) -> (HashMap<Coord, Direction>, Coord, Coord, Coord) {
    let lines = input.lines().collect::<Vec<&str>>();
    
    let size = Coord {
        x: lines[0].len() as i32 - 2,
        y: lines.len() as i32 - 2
    };

    let start_x = lines[0].position(|c| c == '.').unwrap() - 1;
    let end_x = lines[size.y as usize + 1].position(|c| c == '.').unwrap() - 1;

    let blizzard = lines[0..=size.y as usize].iter().enumerate()
        .flat_map(|(row, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| "^v<>".contains(*c))
                .map(move |(col, c)| (
                    Coord { y: row as i32 - 1, x: col as i32 - 1 },
                    Direction::from(c)
                ))
        })
        .collect::<HashMap<Coord, Direction>>();

    (
        blizzard,
        Coord { x: start_x as i32, y: - 1},
        Coord { x: end_x as i32, y: size.y},
        size
    )
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    time: usize,
    coord: Coord,
    distance_to_end: usize,
}

impl State {
    fn new (time: usize, coord: Coord, end: Coord) -> State {
        State {
            time,
            coord,
            distance_to_end: (coord.x).abs_diff(end.x) as usize + coord.y.abs_diff(end.y) as usize,
        }
    }
}
impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.time.cmp(&self.time)
            .then_with(|| other.distance_to_end.cmp(&self.distance_to_end))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn solve(blizzard: &HashMap<Coord, Direction>, size: Coord, start: Coord, end: Coord, shift_in_time: usize) -> usize {
    let mut map = BlizzardMap::new(blizzard, size);

    let mut visited: HashSet<(usize, Coord)> = HashSet::new();
    let mut priority_queue: BinaryHeap<State> = BinaryHeap::new();
    priority_queue.push(State::new(shift_in_time, start, end));

    while let Some(State {time, coord, ..}) = priority_queue.pop() {
        for neigbour in coord.neighbours().iter().chain([coord].iter()) {
            if visited.contains(&(time, *neigbour)) { continue; }
            visited.insert((time, *neigbour));

            if *neigbour == end { return time + 1 - shift_in_time; }

            // if we are at the start, we can move to the next time step
            if *neigbour == start {
                priority_queue.push(State::new(time + 1, *neigbour, end));
                continue;
            }
            
            // if we are wihin the map and the next time step is empty, we can move there
            if (neigbour.x >= 0 && neigbour.x < map.size.x 
                && neigbour.y >= 0 && neigbour.y < map.size.y)
                && map.empty_at(*neigbour, time + 1) {
                priority_queue.push(State::new(time + 1, *neigbour, end));
            }
        }
    }
    panic!("No path found");
    
}

fn main() {
    let (blizzard, start, end, size) = parse(INPUT);
    let time_to_end = solve(&blizzard, size, start, end, 0);
    println!("Part 1: {}", time_to_end);

    let time_to_go_back_to_start = solve(&blizzard, size, end, start, time_to_end);
    let time_to_go_back_to_end = solve(&blizzard, size, start, end, time_to_end+time_to_go_back_to_start);

    println!("Part 2: {}", time_to_end+time_to_go_back_to_start+time_to_go_back_to_end);
}

#[cfg(test)]
mod tests {
    use advent_of_code::iterator_to_string;

    use super::*;
    const TEST_INPUT: &str = "#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#";

    #[test]
    fn parsing_test() {
        let (blizzard, start, end, size) = parse(TEST_INPUT);
        assert_eq!(blizzard, HashMap::from([
            (Coord { y: 0, x: 0 }, Direction::Right),
            (Coord { y: 0, x: 1 }, Direction::Right),
            (Coord { y: 0, x: 3 }, Direction::Left),
            (Coord { y: 0, x: 4 }, Direction::Up),
            (Coord { y: 0, x: 5 }, Direction::Left),
            (Coord { y: 1, x: 1 }, Direction::Left),
            (Coord { y: 1, x: 4 }, Direction::Left),
            (Coord { y: 1, x: 5 }, Direction::Left),
            (Coord { y: 2, x: 0 }, Direction::Right),
            (Coord { y: 2, x: 1 }, Direction::Down),
            (Coord { y: 2, x: 3 }, Direction::Right),
            (Coord { y: 2, x: 4 }, Direction::Left),
            (Coord { y: 2, x: 5 }, Direction::Right),
            (Coord { y: 3, x: 0 }, Direction::Left),
            (Coord { y: 3, x: 1 }, Direction::Up),
            (Coord { y: 3, x: 2 }, Direction::Down),
            (Coord { y: 3, x: 3 }, Direction::Up),
            (Coord { y: 3, x: 4 }, Direction::Up),
            (Coord { y: 3, x: 5 }, Direction::Right),
        ]));
        assert_eq!(size, Coord { x: 6, y: 4 });
        assert_eq!(start, Coord { x: 0, y: -1 });
        assert_eq!(end, Coord { x: 5, y: 4 });
    }

    #[test]
    fn test_state_less_blizzard() {
        let (blizzard, start, end, size) = parse(TEST_INPUT);
        let map = BlizzardMap::new(&blizzard, size);
        dbg!(start, end, size);
        for i in 0..18 {
            map.display_at(i);
            println!("i : {}", i+1);
        }
    }

    #[test]
    fn test_part01() {
        let (blizzard, start, end, size) = parse(TEST_INPUT);
        assert_eq!(solve(&blizzard, size, start, end, 0), 18);
        assert_eq!(solve(&blizzard, size, end, start, 18), 23);
        assert_eq!(solve(&blizzard, size, start, end, 18+23), 13);
    }
}
