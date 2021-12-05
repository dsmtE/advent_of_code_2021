fn main() {
    let input = include_str!("../input.txt");
    
    let size = input.lines().count() as usize;

    // Count bits length from the first lines using chars 
    let bits_width = input.lines().next().unwrap().chars().count();

    let bits_count = input.lines().map(|line| u32::from_str_radix(line, 2).unwrap()).fold(
        vec![0u16; bits_width],
        |count_vec, number| {
            count_vec.into_iter().enumerate().map(|(i, count)| count + ((number & (1 << i)) >> i) as u16).collect()
        },
    );

    let gamma = bits_count.into_iter().enumerate().map(|(i, b)| ((b as usize >= size / 2) as u32) << i).sum::<u32>();
    let epsilon = !gamma & ((1 << bits_width) - 1);

    println!("result : {}", gamma * epsilon);

}
