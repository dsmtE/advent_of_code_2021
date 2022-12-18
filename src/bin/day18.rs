use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::collections::{HashSet, VecDeque};

use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending},
    multi::separated_list1,
    IResult,
};

const INPUT: &str = advent_of_code::get_input!();

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos(i32, i32, i32);

impl std::ops::Add for &Pos {
    type Output = Pos;
    fn add(self, rhs: Self) -> Self::Output {
        Pos(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

fn parse(input: &str) -> IResult<&str, Vec<Pos>> {
    separated_list1(line_ending, parse_coord)(input)
}

fn parse_coord(input: &str) -> IResult<&str, Pos> {
    let (input, x) = complete::i32(input)?;
    let (input, _) = tag(",")(input)?;
    let (input, y) = complete::i32(input)?;
    let (input, _) = tag(",")(input)?;
    let (input, z) = complete::i32(input)?;
    Ok((input, Pos(x, y, z)))
}

fn neighbours_iter(voxel: &Pos) -> impl Iterator<Item = Pos> {
    let Pos(x, y, z) = voxel;
    vec![
        Pos(x - 1, *y, *z),
        Pos(x + 1, *y, *z),
        Pos(*x, y - 1, *z),
        Pos(*x, y + 1, *z),
        Pos(*x, *y, z - 1),
        Pos(*x, *y, z + 1),
    ]
    .into_iter()
}

fn surface_area(voxels: &HashSet<Pos>) -> usize {
    voxels
        .par_iter()
        .map(|voxel| {
            neighbours_iter(voxel)
                .filter(|ivec| voxels.get(ivec).is_none())
                .count()
        })
        .sum::<usize>()
}

fn get_bounds(voxels: &HashSet<Pos>) -> (i32, i32, i32, i32, i32, i32) {
    (
        voxels.par_iter().map(|pos| pos.0).min().unwrap(),
        voxels.par_iter().map(|pos| pos.0).max().unwrap(),
        voxels.par_iter().map(|pos| pos.1).min().unwrap(),
        voxels.par_iter().map(|pos| pos.1).max().unwrap(),
        voxels.par_iter().map(|pos| pos.2).min().unwrap(),
        voxels.par_iter().map(|pos| pos.2).max().unwrap(),
    )
}

fn is_in_bounds(voxel: &Pos, bounds: (i32, i32, i32, i32, i32, i32), margin: i32) -> bool {
    let (min_x, max_x, min_y, max_y, min_z, max_z) = bounds;
    voxel.0 >= min_x - margin
        && voxel.0 <= max_x + margin
        && voxel.1 >= min_y - margin
        && voxel.1 <= max_y + margin
        && voxel.2 >= min_z - margin
        && voxel.2 <= max_z + margin
}

fn exterior_flood_fill_count_faces(voxels: &HashSet<Pos>) -> usize {
    let bound = get_bounds(voxels);

    let mut visited = HashSet::new();
    let mut priority_queue = VecDeque::new();
    let start = Pos(bound.0 - 1, bound.2 - 1, bound.4 - 1);

    visited.insert(start);
    priority_queue.push_back(start);

    let mut exterior_faces = 0;

    while let Some(voxel) = priority_queue.pop_back() {
        for neighbour in neighbours_iter(&voxel) {
            if voxels.get(&neighbour).is_some() {
                exterior_faces += 1;
            } else if !visited.contains(&neighbour) && is_in_bounds(&neighbour, bound, 1) {
                priority_queue.push_back(neighbour);
                visited.insert(neighbour);
            }
        }
    }
    exterior_faces
}

fn main() {
    let (_, voxels_coord) = parse(INPUT).unwrap();
    let voxels: HashSet<Pos> = HashSet::from_iter(voxels_coord.into_iter());

    println!("surface area: {}", surface_area(&voxels));
    println!(
        "exterior surface area: {}",
        exterior_flood_fill_count_faces(&voxels)
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";

    #[test]
    fn parsing_test() {
        let (_, voxels_coord) = parse(TEST_INPUT).unwrap();
        assert_eq!(
            voxels_coord,
            vec![
                Pos(2, 2, 2),
                Pos(1, 2, 2),
                Pos(3, 2, 2),
                Pos(2, 1, 2),
                Pos(2, 3, 2),
                Pos(2, 2, 1),
                Pos(2, 2, 3),
                Pos(2, 2, 4),
                Pos(2, 2, 6),
                Pos(1, 2, 5),
                Pos(3, 2, 5),
                Pos(2, 1, 5),
                Pos(2, 3, 5),
            ]
        );
    }

    #[test]
    fn part1_test() {
        let (_, voxels_coord) = parse(TEST_INPUT).unwrap();

        let voxels: HashSet<Pos> = HashSet::from_iter(voxels_coord.into_iter());

        assert_eq!(surface_area(&voxels), 64);

        assert_eq!(exterior_flood_fill_count_faces(&voxels), 58);
    }
}
