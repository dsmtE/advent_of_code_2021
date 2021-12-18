use std::collections::HashMap;

const ROW_VALIDATION_MASK: u32 = 0b11111;
const COL_VALIDATION_MASK: u32 = 0b100001000010000100001;

fn main() {
    let (nums_input, boards_input) = include_str!("../input.txt").split_once("\n\n").unwrap();

    let nums: Vec<u8> = nums_input.split(',').map(|n| n.parse().unwrap()).collect();

    // let boards : Vec<[[u16; 5]; 5]>

    // HashMap (key: number in bingo board, value: position) with bitmask to keep validate number in bingo
    let boards_and_mask : Vec<(HashMap<u8, usize>, u32)> = boards_input.split("\n\n").map(|board_str| {
        (board_str.split_ascii_whitespace().enumerate().map(|(i, n)| (n.parse().unwrap(), i)).collect(), 0)
    }).collect();

    let mut boards = boards_and_mask.clone();

    let (board, mask, num) = nums.iter().find_map(|n| {
        boards.iter_mut().find_map(|(board, mask)| {
            // try to get number "n" in bingo
            board.get(&n)
            // if exist toggle mask at this mask byte position
            .map(|i| *mask |= 1 << *i)
            // filter with valid boards using mask this and binary mask
            .filter(|_| (0..5).any(|i| (*mask >> i) & COL_VALIDATION_MASK == COL_VALIDATION_MASK || (*mask >> (i * 5)) & ROW_VALIDATION_MASK == ROW_VALIDATION_MASK))
            
            // map for return wanted informations with find_map
            .map(|_| (board.clone(), *mask, n))
        })
    }).unwrap();

    let sum = board.into_iter().map(|(number, position)| ((mask >> position) & 1 ^ 1) * number as u32).sum::<u32>();

    println!("score : {}", (*num as u32) * sum);
}
