const INPUT: &str = advent_of_code::get_input!();

fn get_score(moves: impl IntoIterator<Item = [i32; 2]>) -> i32 {
    moves.into_iter().map(|moves| {
        // magic append here
        (moves[1] + 1) + (moves[1] - moves[0] + 1 + 3) % 3 * 3
    }).sum::<i32>()
}

fn get_moves(input: &str) -> impl Iterator<Item = [i32; 2]> + '_ + Clone {
    input.lines()
        .map(|line| {
            let mut chars = line.chars()
            .step_by(2)
            .map(|char| (char as u8 - 'A' as u8) % ('X' as u8 - 'A' as u8));
            [chars.next().unwrap(), chars.next().unwrap()]
        })
        .map(|x| x.map(i32::from))
}
fn main() {

    let moves = get_moves(INPUT);
    
    println!("score: {}", get_score(moves.clone()));
    
    let alternate_moves = moves.map(|moves| [moves[0], (moves[0] + (moves[1]-1) + 3) % 3]);

    println!("alternate score: {}", get_score(alternate_moves));
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "A Y
B X
C Z";

    #[test]
    fn simple_case() {
        let moves = get_moves(TEST_INPUT);
    
        assert_eq!(moves.clone().collect::<Vec<_>>(), vec![[0i32, 1i32], [1i32, 0i32], [2i32, 2i32]]);
        assert_eq!(get_score(moves), 2+6 + 1+0 + 3+3);
    }

    #[test]
    fn complexe_case() {
        let alternate_moves = get_moves(TEST_INPUT).map(|moves| [moves[0], (moves[0] + (moves[1]-1) + 3) % 3]);

        assert_eq!(alternate_moves.clone().collect::<Vec<_>>(), vec![[0i32, 0i32], [1i32, 0i32], [2i32, 0i32]]);
        assert_eq!(get_score(alternate_moves), 1+3 + 1+0 + 1+6);
    }
}