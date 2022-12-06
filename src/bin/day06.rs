const INPUT: &str = advent_of_code::get_input!();

// fn has_unique_elements_using_hashset<T>(iter: impl Iterator<Item = T>) -> bool
// where
//     T: Eq + std::hash::Hash,
// {
//     let mut uniq = std::collections::HashSet::new();
//     iter.into_iter().all(move |x| uniq.insert(x))
// }

fn has_unique_elements<T>(iter: impl Iterator<Item = T> + Clone) -> bool
where
    T: Eq + std::hash::Hash,
{
    !iter.clone().enumerate().any(|(i, val)| {
        iter.clone().skip(i+1).any(|x| x == val)
    })
}

fn first_uniq_window_index<T>(vec: &[T], window_size: usize) -> usize
where
    T: Eq + std::hash::Hash,
{
    vec.windows(window_size)
    .position(|chars_window| {
        has_unique_elements(chars_window.iter())
    })
    .expect("No unique set found")
}

fn main() {

    let chars = INPUT.chars().collect::<Vec<_>>();
    
    let first4 = first_uniq_window_index(&chars, 4);
    // we can utilise that we already know that before this there cannot be 14 equal ones
    let first14 = first_uniq_window_index(&chars[first4..], 14);
    println!("part01: {}\npart02: {}", 4 + first4, 14 + first4 + first14)
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb
bvwbjplbgvbhsrlpgdmjqwftvncz
nppdvjthqldpwncqszvftbrmjlhg
nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg
zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";

    #[test]
    fn simple_case() {

        let res = TEST_INPUT.lines().map(|line| {
            4 + first_uniq_window_index(&line.chars().collect::<Vec<_>>(), 4)
        }).collect::<Vec<_>>();

        assert_eq!(res, vec![7, 5, 6, 10, 11]);
    }
}