advent_of_code::solution!(5);

use nom::{bytes::complete::tag, character::complete::digit1, combinator::{map_res, recognize}, multi::separated_list1, sequence::separated_pair, IResult};

type OrderingRule = (u32, u32);


pub fn parse_number<T: std::str::FromStr>(input: &str) -> IResult<&str, T> {
    map_res(recognize(digit1), str::parse::<T>)(input)
}

pub fn parse_ordering_rule(input: &str) -> IResult<&str, OrderingRule> {
    separated_pair(parse_number,tag("|"),parse_number)(input)
}

pub fn parse_pages_list(input: &str) -> IResult<&str, Vec<u32>> {
    separated_list1(tag(","),parse_number)(input)
}

pub fn parse_input(input: &str) -> IResult<&str, (Vec<OrderingRule>, Vec<Vec<u32>>)> {
    separated_pair(
        separated_list1(tag("\n"),parse_ordering_rule),
        tag("\n\n"),
        separated_list1(tag("\n"), parse_pages_list)
    )(input)
}

pub fn build_preceding_rules_hash_map(rules: &Vec<OrderingRule>) -> std::collections::HashMap<u32, Vec<u32>> {
    let mut hash_map = std::collections::HashMap::new();
    for (preceding, following) in rules {
        hash_map.entry(*following).or_insert(Vec::new()).push(*preceding);
    }
    hash_map
}

pub fn is_pages_valid(pages: &[u32], hash_map: &std::collections::HashMap<u32, Vec<u32>>) -> bool {
    pages.iter().enumerate().all(|(i, page)| {
        // if this page number isn't in the hash map, then it's valid
        //  else check if all the next pages are not in the preceding pages rules 
        // for this page (i.e. the page number is valid)
        hash_map.get(page).map_or(true,
        |preceding_pages| {
            pages[(i+1)..].iter().all(|next_page| {
                !preceding_pages.contains(next_page)
            })
        })
    })
}

pub fn reorder_pages(pages: &mut [u32], hash_map: &std::collections::HashMap<u32, Vec<u32>>) {
    let mut i = 0;
    while i < pages.len() {
        //swap the current page with the next page if the next page is in the preceding pages rules
        if let Some(preceding_pages) = hash_map.get(&pages[i]) {
            if let Some(j) = pages[(i+1)..].iter().position(|next_page| preceding_pages.contains(next_page)) {
                pages.swap(i, i+j+1);
            } else {
                i += 1;
            }
        } else {
            i += 1;
        }
    }
}

pub fn middle_elements<T>(slice: &[T]) -> &T{
    &slice[slice.len()/2]
}


pub fn part_one(input: &str) -> Option<u32> {
    let (_, (rules, pages)) = parse_input(input).unwrap();
    let hash_map = build_preceding_rules_hash_map(&rules);

    let valid_pages = pages.into_iter().filter(|pages| is_pages_valid(pages, &hash_map));

    // get middle element of each pages list and sum them
    Some(valid_pages.map(|pages| *middle_elements(&pages)).sum())

    //4662
}

pub fn part_two(input: &str) -> Option<u32> {
    let (_, (rules, pages)) = parse_input(input).unwrap();
    let hash_map = build_preceding_rules_hash_map(&rules);

    let result = pages.into_iter()
        .filter(|pages| !is_pages_valid(pages, &hash_map))
        .map(|mut pages| { reorder_pages(&mut pages, &hash_map); pages})
        .map(|x| *middle_elements(&x)).sum();

    Some(result)
    // 5900
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn test_parse() {
        let result = parse_input(TEST_INPUT);
        assert_eq!(result, 
        Ok((
            "",
            (
                vec![
                    (47, 53),(97, 13),(97, 61),(97, 47),(75, 29),
                    (61, 13),(75, 53),(29, 13),(97, 29),(53, 29),
                    (61, 53),(97, 53),(61, 29),(47, 13),(75, 47),
                    (97, 75),(47, 61),(75, 61),(47, 29),(75, 13),(53, 13)
                ],
                vec![
                    vec![75, 47, 61, 53, 29],
                    vec![97, 61, 53, 29, 13],
                    vec![75, 29, 13],
                    vec![75, 97, 47, 61, 53],
                    vec![61, 13, 29],
                    vec![97, 13, 75, 29, 47]
                ]
            )
        )));
    }

    #[test]
    fn test_part_one() {

        let (_, (rules, pages_sections)) = parse_input(TEST_INPUT).unwrap();
        let hash_map = build_preceding_rules_hash_map(&rules);
    
        let valid_pages: Vec<Vec<u32>> = pages_sections.into_iter().filter(|pages| is_pages_valid(pages, &hash_map)).collect();

        assert_eq!(valid_pages, vec![
            vec![75,47,61,53,29],
            vec![97,61,53,29,13],
            vec![75,29,13],
        ]);

        let result: u32 = valid_pages.iter().map(|pages| *middle_elements(&pages)).sum();

        assert_eq!(result, 61+53+29);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(TEST_INPUT);
        assert_eq!(result, None);
    }
}