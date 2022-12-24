use std::{collections::{HashSet, HashMap}};

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::sync::{Arc, Mutex};

const INPUT: &str = advent_of_code::get_input!();
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Coord {
    row: i32,
    col: i32,
}

#[derive(Debug)]
enum Direction {
    North,
    South,
    West,
    East
}

impl Coord {
    fn neighbours(&self) -> [Coord; 8] {
        [
            Coord { row: self.row - 1, col: self.col - 1 },
            Coord { row: self.row - 1, col: self.col + 0 },
            Coord { row: self.row - 1, col: self.col + 1 },
            Coord { row: self.row + 0, col: self.col - 1 },
            Coord { row: self.row + 0, col: self.col + 1 },
            Coord { row: self.row + 1, col: self.col - 1 },
            Coord { row: self.row + 1, col: self.col + 0 },
            Coord { row: self.row + 1, col: self.col + 1 },
        ]
    }
}

fn parse(input: &str) -> HashSet<Coord> {
    input.lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .map(move |(col, _)| Coord {
                    row: row as i32,
                    col: col as i32,
                })
        }).collect::<HashSet<_>>()
}

fn get_map_bounds(elves: &HashSet<Coord>) -> ((i32, i32), (i32, i32)) {
    elves.iter().fold(
        ((i32::MAX, i32::MIN), (i32::MAX, i32::MIN)),
        |(minmax_row, minmax_col), Coord { row, col }| {
            (
                (minmax_row.0.min(*row), minmax_row.1.max(*row)),
                (minmax_col.0.min(*col), minmax_col.1.max(*col)),
            )
        },
    )
}

fn display_elves_positions(elves: &HashSet<Coord>) {
    let (minmax_row, minmax_col) = get_map_bounds(elves);

    for row in minmax_row.0..=minmax_row.1 {
        for col in minmax_col.0..=minmax_col.1 {
            if elves.contains(&Coord { row, col }) {
                print!("#")
            } else {
                print!(".")
            }
        }
        println!();
    }
}

fn compute_proposals<'a>(elves: &'a HashSet<Coord>, checks_direction : &'a [Direction; 4]) -> HashMap<Coord, Vec<Coord>> {
    
    let proposals: Arc<Mutex<HashMap<Coord, Vec<Coord>>>> = Arc::new(Mutex::new(HashMap::new()));

    let t = elves.par_iter().filter_map(|elf| {
        let neighbours_positions = elf.neighbours();
        // if all neighbours are empty, skip
        if neighbours_positions.iter().all(|coord| !elves.contains(coord)) {
            None
        }else {
            let neighbours: [bool; 8] = neighbours_positions
                .iter()
                .map(|neighbour| elves.contains(neighbour))
                .collect::<Vec<bool>>()
                .try_into()
                .unwrap();
            
            // check North, South, East, West using checks_direction order
            checks_direction.iter().find_map(|direction| {
                let (row, col) = match direction {
                    Direction::North => (elf.row - 1, elf.col),
                    Direction::South => (elf.row + 1, elf.col),
                    Direction::West => (elf.row, elf.col - 1),
                    Direction::East => (elf.row, elf.col + 1),
                };

                let neighbours_empty_in_dir = match direction {
                    Direction::North => !neighbours[0] && !neighbours[1] && !neighbours[2],
                    Direction::South => !neighbours[5] && !neighbours[6] && !neighbours[7],
                    Direction::West => !neighbours[0] && !neighbours[3] && !neighbours[5],
                    Direction::East => !neighbours[2] && !neighbours[4] && !neighbours[7],
                };

                if neighbours_empty_in_dir {
                    Some((elf, Coord { row, col }))
                }else {
                    None
                }
            })
        }
    });

    t.for_each(|(elf, coord)| {
        proposals.lock().unwrap().entry(coord).or_default().push(*elf);
    });

    // How avoid copy here ?
    let proposals = proposals.lock().unwrap().clone();
    proposals
}

fn solve(elves: &HashSet<Coord>, stop_at: Option<usize>) -> (usize, usize) {

    let mut elves = elves.clone();

    let mut checks_state = [
            Direction::North,
            Direction::South,
            Direction::West,
            Direction::East,
        ];

    let mut round = 0;
    loop {
        // if round == 0 {
        //     println!("== Initial State ==");
        // } else {
        //     println!("== End of Round {} ==", round);
        // }
        // display_elves_positions(&elves);
        // dbg!(&checks_state);
        
        // compute proposals
        let proposals = compute_proposals(&elves, &checks_state);
        
        let mut moved = false;

        // apply proposals only if there is only one elf proposing a given position
        for (new_coord, old_coords) in proposals.iter()
            .filter(|(_, from_coordinates)| from_coordinates.len() == 1) 
        {
            elves.remove(&old_coords[0]);
            elves.insert(*new_coord);
            moved = true;
        }

        checks_state.rotate_left(1);

        round += 1;

        if !moved || (stop_at.is_some() && round == stop_at.unwrap()) {
            break;
        }
    }
    // println!("== End of Round {} ==", round);
    // display_elves_positions(&elves);
    
    let (minmax_row, minmax_col) = get_map_bounds(&elves);
    let area = ((minmax_row.1-minmax_row.0+1) * (minmax_col.1-minmax_col.0+1)) as usize;
    (area - elves.len(), round)
    
}
fn main() {
    let elves = parse(INPUT);
    println!("Part 1: {}", solve(&elves, Some(10)).0);
    println!("Part 2: {}", solve(&elves, None).1);
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use advent_of_code::iterator_to_string;

    use super::*;
    const TEST_INPUT: &str = "....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#..";

    #[test]
    fn test() {
        let elves = parse(TEST_INPUT);
        assert_eq!(solve(&elves, Some(10)).0, 110);
        assert_eq!(solve(&elves, None).1, 20);
    }
}
