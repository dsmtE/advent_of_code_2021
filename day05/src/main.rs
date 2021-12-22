use atoi::atoi;
use nom::*;

// macro for parsing with nom library
named!(usize<&[u8], usize>, map_opt!(nom::character::complete::digit1, atoi));
named!(coord<&[u8], (usize, usize)>, separated_pair!(usize, char!(','), usize));
named!(line<&[u8], ((usize, usize), (usize, usize))>, separated_pair!(coord, tag!(" -> "), coord));

fn main() {
    let input_lines = include_bytes!("../input.txt").split(|b| *b == b'\n');

    let coordinates = input_lines.map(|entry| {
        let ((x, y), (xx, yy)) = line(entry).unwrap().1;
        (x, y, xx, yy)
    }).collect::<Vec<_>>();

    let (max_x, max_y) = coordinates.iter().fold(
        (0usize, 0usize),
        |(max_x, max_y), (x, y, xx, yy)| {
            (max_x.max(*x).max(*xx), max_y.max(*y).max(*yy))
        },
    );

    let (width, height) = (max_x + 1, max_y + 1);
    // println!("map of size: {}x{}", width, height);

    let mut map = vec![0u8; width * height];
    let mut overlaps = 0;

    coordinates.iter().for_each(|&(x, y, xx, yy)| {
        let mut mark = |x, y| {
            if map[(x + y * width) as usize] == 1 {
                // increment overlap 
                overlaps += 1;
            }
            // update board
            map[(x + y * width) as usize] += 1;
        };

        // horizontal or vertical line
        if x == xx {
            (y.min(yy)..=y.max(yy)).for_each(|i| mark(x, i));
        } else if y == yy {
            (x.min(xx)..=x.max(xx)).for_each(|i| mark(i, y));
        }
    });

    println!("result : {}", overlaps);

    let mut map = vec![0u8; width * height];
    let mut overlaps = 0;

    coordinates.iter().for_each(|&(x, y, xx, yy)| {
        let mut mark = |x, y| {
            if map[(x + y * width) as usize] == 1 {
                // increment overlap 
                overlaps += 1;
            }
            // update board
            map[(x + y * width) as usize] += 1;
        };

        // horizontal ,vertical or diagonal line
        if x == xx {
            (y.min(yy)..=y.max(yy)).for_each(|i| mark(x, i));
        } else if y == yy {
            (x.min(xx)..=x.max(xx)).for_each(|i| mark(i, y));
        } else {
            // println!("{},{} -> {},{} : dir: {}_{}", x, y, xx, yy, if xx>x {1} else {-1}, if yy>y {1} else {-1});
            
            let x_forward = xx>x;
            let y_forward = yy>y;
            (0..=(x.max(xx)-x.min(xx))).for_each(|i| mark(if x_forward {x+i} else {x-i}, if y_forward {y+i} else {y-i}));
        }
    });

    println!("result bis : {}", overlaps);
}
