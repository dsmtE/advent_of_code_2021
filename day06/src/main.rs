use std::env;

fn main() {

    let mut args = env::args();
    args.next();

    let days = args.next().expect("Unable to get days argument").parse::<usize>().expect("Unable to convert argument to number");

    let mut map = include_str!("../input.txt").split(',').fold([0; 9], |mut map, n| {
            map[n.parse::<usize>().unwrap()] += 1;
            map
        });

    (0..days).for_each(|_| {
        map.rotate_left(1); // spawn new automaticaly by rotating
        map[6] += map[8]; // reset older 0 to 6
    });

    println!("final day : {:?}", map);
    println!("{}", map.iter().sum::<usize>());
}
