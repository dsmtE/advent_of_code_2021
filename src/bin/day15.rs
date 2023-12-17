use std::path::Display;

const INPUT: &str = aoc_utils::get_input!();

fn parse_input(input: &str) -> Vec<&[u8]> {
    input.trim().as_bytes().split(|&b| b == b',').collect()
}

#[inline]
fn hash(slice: &[u8]) -> usize {
    slice.iter().fold(0, |acc, &b| ((acc + b as usize) * 17) & 0xff)
}

fn process_lens_boxes<'a>(input: Vec<&'a[u8]>) -> Vec<Vec<Lens<'a>>> {
    let mut boxes: Vec<Vec<Lens<'_>>> = (0..256).map(|_| Vec::new()).collect();

    for slice in input {
        if let Some(lens) = Lens::from(slice) {
            let slot = &mut boxes[hash(lens.label)];

            // Replace or append new lens.
            if let Some(i) = slot.iter().position(|item| item.label == lens.label) {
                // replace with the new lens (the label is the same).
                slot[i] = lens;
            } else {
                slot.push(lens);
            }
        }else {
            let label = &slice[..(slice.len()-1)];
            let hash = hash(label);
            let slot = &mut boxes[hash];

            // If one lens with the same label exists, remove it.
            if let Some(i) = slot.iter().position(|item| item.label == label) {
                slot.remove(i);
            }
        }
    }

    boxes
}

fn compute_focusing_power_sum(boxes: Vec<Vec<Lens<'_>>>) -> usize {
    boxes
        .iter()
        .enumerate()
        .filter(|(_, list_of_lens)| !list_of_lens.is_empty())
        // .inspect(|(box_number, list_of_lens)| {
        //     println!("Box {}: {}", box_number, aoc_utils::iterator_helpers::iterator_to_string(list_of_lens.iter(), ", "));
        // })
        .fold(0, |acc, (box_number, list_of_lens)| {
            list_of_lens.iter().enumerate().fold(acc, |acc, (lens_number, item)| {
                acc + (box_number + 1) * (lens_number + 1) * item.focal_length
            })
        })
}

fn main() {
    let input = parse_input(INPUT);

    let first_start = input.iter().map(|s| hash(s)).sum::<usize>();

    println!("First star: {}", first_start);

    let boxes: Vec<Vec<Lens<'_>>> = process_lens_boxes(input);
    let focusing_power_sum = compute_focusing_power_sum(boxes);

    println!("Second star: {}", focusing_power_sum);
    
}

#[derive(Debug)]
struct Lens<'a> {
    label: &'a [u8],
    focal_length: usize,
}

impl std::fmt::Display for Lens<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}", String::from_utf8_lossy(self.label), self.focal_length)
    }
}

impl Lens<'_> {
    fn from(slice: &[u8]) -> Option<Lens<'_>> {
        let size = slice.len();
        if slice[size - 2] == b'=' {
            Some(Lens {
                label: &slice[..(size - 2)],
                focal_length: slice[size-1].wrapping_sub(b'0') as usize
            })
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn hash_test() {
        assert_eq!(hash(b"HASH"), 52);
        assert_eq!(hash(b"rn=1"), 30);
    }

    #[test]
    fn first_start() {
        let input = parse_input(TEST_INPUT);
        let result = input.iter().map(|s| hash(s)).collect::<Vec<_>>();
        assert_eq!(result, vec![30, 253, 97, 47, 14, 180, 9, 197, 48, 214, 231]);
        assert_eq!(result.iter().sum::<usize>(), 1320);
    }
    
    #[test]
    fn second_start() {
        let input = parse_input(TEST_INPUT);

        let boxes: Vec<Vec<Lens<'_>>> = process_lens_boxes(input);
        let focusing_power_sum = compute_focusing_power_sum(boxes);
        
        assert_eq!(focusing_power_sum, 145);

    }

}