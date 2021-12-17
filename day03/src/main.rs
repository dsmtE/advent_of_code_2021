fn main() {
    let input = include_str!("../input.txt");
    
    let size = input.lines().count() as usize;

    // Count bits length from the first lines using chars 
    let bits_width = input.lines().next().unwrap().chars().count();

    let numbers = input.lines().map(|line| u32::from_str_radix(line, 2).unwrap()).collect::<Vec<_>>();

    let bits_count = numbers.iter().fold(
        vec![0u16; bits_width],
        |count_vec, number| {
            count_vec.into_iter().enumerate().map(|(i, count)| count + ((number & (1 << i)) >> i) as u16).collect()
        },
    );

    let gamma = bits_count.into_iter().enumerate().map(|(i, b)| ((b as usize >= size / 2) as u32) << i).sum::<u32>();
    let epsilon = !gamma & ((1 << bits_width) - 1);

    println!("result : {}", gamma * epsilon);

    let mut current_nums_list = numbers.clone();
    let mut shift = (bits_width - 1) as isize;

    while current_nums_list.len() > 1 && shift >= 0 {
        // println!("list :");
        // for i in 0..current_nums_list.len() { println!(" - {:0>5b}", current_nums_list[i]); }

        let ones_count = current_nums_list.iter().filter(|&n| (n & (1 << shift)) > 0).count();
        // println!("ones_count : {}", ones_count);
        let most_common: bool = ones_count >= (current_nums_list.len() + 1) / 2;
        // println!("most_common : {}", if most_common {"1"} else {"0"});

        current_nums_list.retain(|&n| (n & (1 << shift) > 0) == most_common);

        shift -= 1;
    }

    let oxygen_rating = current_nums_list[0];

    let mut current_nums_list = numbers.clone();
    let mut shift = (bits_width - 1) as isize;

    while current_nums_list.len() > 1 && shift >= 0 {
        let most_common: bool = current_nums_list.iter().filter(|&n| (n & (1 << shift)) > 0).count() >= (current_nums_list.len() + 1) / 2;
        current_nums_list.retain(|&n| (n & (1 << shift) > 0) != most_common);
        shift -= 1;
    }

    let co2_rating = current_nums_list[0];

    println!("result : {}", oxygen_rating * co2_rating);
}
