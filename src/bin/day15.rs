const INPUT: &str = aoc_utils::get_input!();

fn parse_input(input: &str) -> Vec<&[u8]> {
    input.trim().as_bytes().split(|&b| b == b',').collect()
}

#[inline]
fn hash(slice: &[u8]) -> usize {
    slice.iter().fold(0, |acc, &b| ((acc + b as usize) * 17) & 0xff)
}

fn main() {
    let input = parse_input(INPUT);

    let first_start = input.iter().map(|s| hash(s)).sum::<usize>();

    println!("First star: {}", first_start);
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

}