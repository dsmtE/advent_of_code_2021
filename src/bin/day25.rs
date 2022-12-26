const INPUT: &str = advent_of_code::get_input!();

fn SNAFU_char_to_multiplier(c: char) -> i64 {
    match c {
        '2' => 2,
        '1' => 1,
        '0' => 0,
        '-' => -1,
        '=' => -2,
        _ => panic!("Invalid SNAFU character"),
    }
}

fn snafu_to_dec(input: &str) -> i64 {
    input.chars().rev().enumerate().fold(
        0,
        |acc, (i, c)|  {
            acc + SNAFU_char_to_multiplier(c) * 5i64.pow(i as u32)
        }
    )
}

fn dec_to_snafu(input: i64) -> String {
    let mut snafu_chars: Vec<char> = Vec::new();
    let mut remainder = input;

    while remainder != 0 {
        let modulo_five = remainder % 5;
        snafu_chars.push(
            match modulo_five {
                4 => '-',
                3 => '=',
                2 => '2',
                1 => '1',
                0 => '0',
                _ => panic!("Invalid remainder"),
            }
        );
        
        let complement = (modulo_five + 2) % 5 - 2;
        remainder = (remainder - complement) / 5;
    }
    snafu_chars.reverse();
    String::from_iter(snafu_chars.iter())
}

fn parse(input: &str) -> Vec<&str> {
    input.lines().collect::<Vec<&str>>()
}

fn main() {
    let input = parse(INPUT);
    
    let part1 = dec_to_snafu(
        input.iter()
            .map(|x| snafu_to_dec(*x))
            .sum::<i64>()
    );

    println!("Part 1: {}", part1);
}

#[cfg(test)]
mod tests {
    use std::fmt::format;

    use advent_of_code::{iterator_to_string, mapped_iterator_to_string};

    use super::*;
    const TEST_INPUT: &str = "1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122";

    #[test]
    fn SNAFU_to_dec_test() {
        let input = parse(TEST_INPUT);

        let dec_input = input.iter()
            .map(|x| snafu_to_dec(*x))
            .collect::<Vec<i64>>();

        println!("  SNAFU  Decimal");
        for (snafu, dec) in input.iter().zip(dec_input.iter()) {
            println!("{:>7}{:>9}", snafu, dec);
        }
        assert_eq!(dec_input, vec![1747, 906, 198, 11, 201, 31, 1257, 32, 353, 107, 7, 3, 37]);
        
        let snafu_from_dec = dec_input.iter()
            .map(|x| dec_to_snafu(*x))
            .collect::<Vec<String>>();

        assert_eq!(snafu_from_dec, input);

    }
}
