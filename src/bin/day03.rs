const INPUT: &str = advent_of_code::get_input!();

fn get_items(input: &str) -> impl Iterator<Item = char> + '_ + Clone {
    input.lines().map(|line| {
        let compartments = line.split_at(line.len() / 2);
        compartments.0.chars()
            .find(|c| compartments.1.contains(*c))
            .expect("Unable to find the character in common in both compartments")
    })
}

fn get_badges(input: &str) -> Vec<char> {
    input.lines()
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|chunks| {
            if chunks.len() != 3 { panic!("qdqz"); }
            chunks[0].chars()
                .find(|c| chunks[1].contains(*c) && chunks[2].contains(*c))
                .expect("Unable to find common item in three rucksacks")
        }).collect::<Vec<_>>()
}

fn compute_priorities<'a>(items: impl IntoIterator<Item = char> + 'a) -> impl Iterator<Item = u32> + 'a {
    items.into_iter().map(|c| (1 + c as i8 - ('A' as i8 - 26) - ('a' as i8 - ('A' as i8 - 26)) * c.is_lowercase() as i8) as u32)
}

fn main() {
    let priorities_sum = compute_priorities(get_items(INPUT)).sum::<u32>();
    let badges_priorities_sum = compute_priorities(get_badges(INPUT)).sum::<u32>();

    println!("Priorities sum: {}", priorities_sum);
    println!("badges priorities sum: {}", badges_priorities_sum);
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn simple_case() {
        let items = get_items(TEST_INPUT);
        let priorities = compute_priorities(items.clone());
    
        assert_eq!(items.collect::<Vec<_>>(), vec!['p', 'L', 'P', 'v', 't', 's']);
        assert_eq!(priorities.collect::<Vec<_>>(), vec![16, 38, 42, 22, 20, 19]);
    }

    #[test]
    fn complexe_case() {
        let badges = get_badges(TEST_INPUT);
        let badges_priorities = compute_priorities(badges.clone());

        assert_eq!(badges, vec!['r', 'Z']);
        assert_eq!(badges_priorities.collect::<Vec<_>>(), vec![18, 52]);
    }
}