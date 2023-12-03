
const INPUT: &str = advent_of_code_2023::get_input!();

fn to_decimal(c: char) -> Option<u32> {
    let as_number = (c as u32).wrapping_sub('0' as u32);
    if as_number < 10 { Some(as_number) } else { None }
}

fn first_digit<'a>(str_iter: impl IntoIterator<Item = char>) -> Option<u32> {
    str_iter.into_iter()
        .map(to_decimal)
        .flatten()
        .next()
}

fn digits_iter<'a>(string: &str) -> impl Iterator<Item = u32> + '_ {
    let mut index = 0;
    std::iter::from_fn(move || {
        let reduced_string = &string[index..];

        let next_digit = if reduced_string.starts_with("one") { Some('1') }
        else if reduced_string.starts_with("two") { Some('2') }
        else if reduced_string.starts_with("three") { Some('3') }
        else if reduced_string.starts_with("four") { Some('4') }
        else if reduced_string.starts_with("five") { Some('5') }
        else if reduced_string.starts_with("six") { Some('6') }
        else if reduced_string.starts_with("seven") { Some('7') }
        else if reduced_string.starts_with("eight") { Some('8') }
        else if reduced_string.starts_with("nine") { Some('9') }
        else { reduced_string.chars().next() };
        index += 1;
        next_digit
    })
    .map(to_decimal).flatten()
} 

fn main() {
    let lines = INPUT.split("\n");
    let sum: u32 = lines.clone().map(|line| {
        first_digit(line.chars()).unwrap() * 10 + first_digit(line.chars().rev()).unwrap()
    }).sum();

    println!("First star: {}", sum);

    let sum = lines.clone().map(|line| {
        let digits = digits_iter(line).collect::<Vec<_>>();
        digits.first().unwrap() * 10 + digits.last().unwrap()

    }).sum::<u32>();
    
    println!("Second star: {}", sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_star() {
        const TEST_INPUT: &str = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";

        let sum: u32 = TEST_INPUT.split("\n").map(|line| {
            first_digit(line.chars()).unwrap() * 10 + first_digit(line.chars().rev()).unwrap()
        }).sum();

        assert!(sum == 142);
    }

    #[test]
    fn second_star() {
        const TEST_INPUT: &str = "two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen";

        let sum = TEST_INPUT.split("\n").map(|line| {
            let digits = digits_iter(line).collect::<Vec<_>>();
            digits.first().unwrap() * 10 + digits.last().unwrap()

        }).sum::<u32>();
        
        assert!(sum == 281);
    }
}