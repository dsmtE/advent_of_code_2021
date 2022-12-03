const INPUT: &str = advent_of_code::get_input!();

fn main() {
    let first_three_max = INPUT.split("\n\n")
    .map(|group| group.lines().map(|line| line.parse::<usize>().unwrap()).sum::<usize>()).into_iter().fold([0usize; 3], 
        |mut acc, value| {
            acc.sort_by(|a, b| b.cmp(a));
            [acc[0], acc[1], std::cmp::max(acc[2], value)]
        }
    );

    println!("max: {}", first_three_max[0]);
    println!("Sum of three max: {}", first_three_max.iter().sum::<usize>());
}