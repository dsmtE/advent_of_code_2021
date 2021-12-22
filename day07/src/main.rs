fn main() {
    let positions = include_str!("../input.txt")
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect::<Vec<i32>>();

    let min_fuel = (0..=*positions.iter().max().unwrap())
        .map(|t| positions.iter().map(|n| (n - t).abs()).sum::<i32>())
        .min()
        .unwrap();

    println!("min fuel : {}", min_fuel);
}
