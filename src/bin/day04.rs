use nom::{
    IResult,
    bytes::complete::tag,
    character::complete::{self, newline},
    multi::separated_list1,
    sequence::separated_pair
};

const INPUT: &str = advent_of_code::get_input!();

// Input parsing using nom
// use tuple(u32, u32) instead of RangeInclusive because we only need the end and start information
fn section_range(input: &str) -> IResult<&str, (u32, u32)> {
    let (input, (start, end)) = separated_pair(complete::u32, tag("-"), complete::u32)(input)?;
    Ok((input, (start, end)))
}

fn section_pair(input: &str) -> IResult<&str, ((u32, u32), (u32, u32))> {
    separated_pair(section_range, tag(","), section_range)(input)
}

fn sections_assigments(input: &str) -> IResult<&str, Vec<((u32, u32), (u32, u32))>> {
    separated_list1(newline, section_pair)(input)
}

fn count_fully_contained_sections<'a>(sections: impl Iterator<Item = &'a ((u32, u32), (u32, u32))>) -> usize {
    sections.filter(|(a, b)| {
            (a.0 >= b.0 && a.1 <= b.1) || (b.0 >= a.0 && b.1 <= a.1)
        })
        .count()
}

fn count_overlap_sections<'a>(sections: impl Iterator<Item = &'a ((u32, u32), (u32, u32))>) -> usize {
    sections.filter(|(a, b)| {
            (a.0 >= b.0 && a.0 <= b.1) || (a.1 >= b.0 && a.1 <= b.1) || (b.0 >= a.0 && b.0 <= a.1) || (b.1 >= a.0 && b.1 <= a.1)
        })
        .count()
}

fn main() {
    let sections_assigments = sections_assigments(INPUT)
        .expect("Unable to parse sections_assigments").1;

    let fully_contained_sections_count = count_fully_contained_sections(sections_assigments.iter());

    let overlap_sections_count = count_overlap_sections(sections_assigments.iter());

    println!("Fully contained sections count: {}", fully_contained_sections_count);
    println!("Overlap sections count: {}", overlap_sections_count);
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "2-4,6-8\n2-3,4-5\n5-7,7-9\n2-8,3-7\n6-6,4-6\n2-6,4-8";

    #[test]
    fn simple_case() {
        let sections_assigments = sections_assigments(TEST_INPUT).unwrap().1;
        let fully_contained_sections_count = count_fully_contained_sections(sections_assigments.iter());
        
        assert_eq!(sections_assigments, vec![((2,4), (6,8)), ((2,3), (4,5)), ((5,7), (7,9)), ((2,8), (3,7)), ((6,6), (4,6)), ((2,6), (4,8))]);
        assert_eq!(fully_contained_sections_count, 2);
    }

    #[test]
    fn complexe_case() {
        let overlap_sections_count = count_overlap_sections(sections_assigments(TEST_INPUT).unwrap().1.iter());

        assert_eq!(overlap_sections_count, 4);
    }
}