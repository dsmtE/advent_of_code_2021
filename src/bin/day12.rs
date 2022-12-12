use std::collections::{BinaryHeap, HashSet};
use std::cmp::Ordering;

const INPUT: &str = advent_of_code::get_input!();

type Map = Vec<Vec<u8>>;

fn display_map(map: &Map, separator: &str){
    let max_spacing = map.iter().map(|l| {
        l.iter().map(|x| (*x as f64).log10().ceil() as usize)
            .fold(std::usize::MIN, |a,b| a.max(b) )
    }).fold(std::usize::MIN, |a,b| a.max(b) );

    for row in map {
        for col in row {
            print!("{:spacing$}{}", col, separator, spacing = max_spacing);
        }
        println!();
    }
}

fn parse(input: &str) -> (Map, (usize, usize), (usize, usize)) {
    let mut start: (usize, usize) = (0, 0);
    let mut end: (usize, usize) = (0, 0);
    let height_map: Vec<Vec<u8>> = input.lines()
        .enumerate()
        .map(|(y, l)| {
            l.as_bytes()
                .iter()
                .enumerate()
                .map(|(x, b)| {
                    (match *b {
                        b'S' => {
                            start = (x, y);
                            b'a'
                        }
                        b'E' => {
                            end = (x, y);
                            b'z'
                        }
                        b => b,
                    }) - b'a'
                })
                .collect()
        }).collect();
    (height_map, start, end)
}


fn neighbours(map: &Map, pos: (usize, usize)) -> Vec<(usize, usize)> {
    let mut candidates: Vec<(usize, usize)> = Vec::new();

    let (x, y) = pos;
    if y > 0 { candidates.push((x, y - 1)); } // up
    if y < map.len() - 1 { candidates.push((x, y + 1)); } // down
    if x > 0 { candidates.push((x - 1, y)); } // left
    if x < map[0].len() - 1 { candidates.push((x + 1, y)); } // right

    // filtering neighbours candidates
    let current_height = map[y][x] as i8;
    candidates.into_iter().filter(|(to_x, to_y)| {
        map[*to_y][*to_x] as i8 - 1 <= current_height
    }).collect()
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Node {
    cost: u32,
    pos: (usize, usize),
}

// Needed for BinaryHeap priority queue
impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn depth_first_search(map: &Map, start: (usize, usize), end: (usize, usize)) -> u32 {
    
    let mut priority_queue = BinaryHeap::new();
    let mut visited = HashSet::new();

    priority_queue.push(Node {
        cost: 0,
        pos: start,
    });
    visited.insert(start);
    
    while let Some(Node { cost, pos }) = priority_queue.pop() {
        if pos == end {
            return cost;
        }

        for candidate in neighbours(map, pos) {
            if visited.insert(candidate) {
                priority_queue.push(Node {
                    cost: cost + 1,
                    pos: candidate,
                })
            }
        }
    }

    panic!("No path found");
    
}

fn depth_first_search_two(map: &Map, starting_height: u8, end: (usize, usize)) -> u32 {
    
    let mut priority_queue = BinaryHeap::new();
    let mut visited = HashSet::new();
    
    for (y, row) in map.iter().enumerate() {
        for (x, height) in row.iter().enumerate() {
            if *height == starting_height {
                priority_queue.push(Node {
                    cost: 0,
                    pos: (x, y),
                });
                visited.insert((x, y));
            }
        }
    }

    while let Some(Node { cost, pos }) = priority_queue.pop() {
        if pos == end {
            return cost;
        }

        for candidate in neighbours(map, pos) {
            if visited.insert(candidate) {
                priority_queue.push(Node {
                    cost: cost + 1,
                    pos: candidate,
                })
            }
        }
    }

    panic!("No path found");
    
}

fn main() {
    let (height_map, start, end) = parse(INPUT);

    let cost = depth_first_search(&height_map, start, end);
    println!("Fewest steps required to move from 'S' to 'E': {}", cost);

    let cost_two = depth_first_search_two(&height_map, 0, end);
    println!("Fewest steps required to move starting from any square with elevation 'a' to 'E'': {}", cost_two);
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    fn simple_case() {
        let (height_map, start, end) = parse(TEST_INPUT);
        display_map(&height_map, ", ");

        assert_eq!(depth_first_search(&height_map, start, end), 31);
        assert_eq!(depth_first_search_two(&height_map, 0, end), 29);
    }
}