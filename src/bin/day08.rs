use rayon::iter::{ParallelIterator, IntoParallelRefIterator, IndexedParallelIterator};

const INPUT: &str = advent_of_code::get_input!();

type Map = Vec<Vec<u32>>;

fn parse(input: &str) -> Map {
    input.lines().map(
        |l| l.chars().map(|c| c.to_digit(10).unwrap()).collect()
        )
        .collect()
}

fn tree_iter(map: &Map) -> impl ParallelIterator<Item = (u32, (usize, usize))> + '_ {
    map.par_iter()
        .enumerate()
        .flat_map(
            |(row, line)| {
                line.par_iter()
                    .enumerate()
                    .map(move |(col, tree_height)| (*tree_height, (col, row)))
            }
    )
}

fn line_col_iter<'a>( map: &'a Map, (x, y): (usize, usize)) -> impl Iterator<Item = impl Iterator<Item = u32> + 'a> {
    let up = Box::new((0..y).rev().map(move |i| map[i][x]));
    let down = Box::new((y + 1..map.len()).map(move |i| map[i][x]));
    let left = Box::new((0..x).rev().map(move |i| map[y][i]));
    let right = Box::new((x + 1..map[0].len()).map(move |i| map[y][i]));

    let a: [Box<dyn Iterator<Item = u32> + Send + 'a>; 4] = [up, down, left, right];
    a.into_iter()
}

fn visible_trees_count(map: &Map) -> usize {
    tree_iter(&map)
        .filter(|(current, (x, y))| {
            line_col_iter(&map, (*x, *y)).any(|mut line| line.all(|h| h < *current))
        })
        .count()
}

fn max_scenic_score(map: &Map)-> usize {
    tree_iter(&map)
            .map(|(current_height, (x, y))| {
                line_col_iter(&map, (x, y))
                    .map(|line| {
                        let mut count: usize = 0;
                        for height in line {
                            count += 1;
                            if height >= current_height { break; }
                        }
                        count
                    })
                    .product::<usize>()
            })
            .max()
            .unwrap()
}

fn main() {
    let map = parse(INPUT);

    let visible_trees = visible_trees_count(&map);
    let max_scenic_score = max_scenic_score(&map);

    println!("visible trees: {}", visible_trees);
    println!("max scenic score: {}", max_scenic_score);
}


#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "30373
25512
65332
33549
35390";


    #[test]
    fn parse_input() {
        let map = parse(TEST_INPUT);

        assert_eq!(map, vec![
                vec![3,0,3,7,3],
                vec![2,5,5,1,2],
                vec![6,5,3,3,2],
                vec![3,3,5,4,9],
                vec![3,5,3,9,0]
            ]
        );
    }

    #[test]
    fn visible_trees() {
        let map = parse(TEST_INPUT);

        let visible_trees = visible_trees_count(&map);
        let max_scenic_score = max_scenic_score(&map);
        assert_eq!(visible_trees, 21);
        dbg!(max_scenic_score);
        // assert_eq!(max_scenic_score, 8);

        // dbg!(max_scenic_score(&map));
    }
        
}