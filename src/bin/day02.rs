const INPUT: &str = advent_of_code_2023::get_input!();

const COUNT_SET: Set = Set { blue: 14, green: 13, red: 12 };

#[derive(PartialEq, Clone, Copy, Debug, Default)]
struct Set {
    blue: u32,
    green: u32,
    red: u32,
}

// TODO: how write derive macro to forward operator based on fields if all fields implement the operator ?
impl std::ops::Add<Set> for Set {
    type Output = Set;
    
    fn add(self, rhs: Set) -> Set {
        Set {
            blue: self.blue + rhs.blue,
            green: self.green + rhs.green,
            red: self.red + rhs.red,
        }
    }
}

impl Set {
    fn max(&self, other: &Set) -> Set {
        Set {
            blue: self.blue.max(other.blue),
            green: self.green.max(other.green),
            red: self.red.max(other.red),
        }
    }

    fn power(&self) -> u32 {
        self.blue * self.green * self.red
    }
}

#[derive(PartialEq, Debug)]
struct Game (Vec<Set>);

impl Game {
    fn max(&self) -> Set {
        self.0.iter().fold(Set::default(),|a, b| a.max(b))
    }
}

fn games(input: &str) -> Vec<Game> {
    input.split("\n")
        .map(|line| {
            Game(
                line.split_once(":")
                .unwrap().1
                .split(";")
                .map(|set_string| {
                    let mut set = Set { blue: 0, green: 0, red: 0 };
                    set_string.split(",").for_each(|color_count| {
                        let (count, color) = color_count.trim().split_once(" ").unwrap();
                        match color {
                            "blue" => set.blue = count.parse().unwrap(),
                            "green" => set.green = count.parse().unwrap(),
                            "red" => set.red = count.parse().unwrap(),
                            _ => panic!("unknown color"),
                        }
                    });
                    set
                })
                .collect::<Vec<_>>()
            )
        })
        .collect()
}

fn main() {
    let games: Vec<Game> = games(INPUT);
    let games_minimum_playable_set = games.iter().map(|game| game.max());

    let first_start = games_minimum_playable_set.clone().enumerate().filter(|(_, set)| {
        set.blue <= COUNT_SET.blue && set.green <= COUNT_SET.green && set.red <= COUNT_SET.red
    }).map(|(index, _)| index + 1).sum::<usize>();

    println!("First star: {}", first_start);

    let second_star = games_minimum_playable_set.map(|set| set.power()).sum::<u32>();

    println!("Second star: {}", second_star);
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn parsing() {
        assert_eq!(
            games(TEST_INPUT), 
            vec![
                Game(vec![
                    Set { blue: 3, green: 0, red: 4 },
                    Set { blue: 6, green: 2, red: 1 },
                    Set { blue: 0, green: 2, red: 0 },
                ]),
                Game(vec![
                    Set { blue: 1, green: 2, red: 0 },
                    Set { blue: 4, green: 3, red: 1 },
                    Set { blue: 1, green: 1, red: 0 },
                ]),
                Game(vec![
                    Set { blue: 6, green: 8, red: 20 },
                    Set { blue: 5, green: 13, red: 4 },
                    Set { blue: 0, green: 5, red: 1 },
                ]),
                Game(vec![
                    Set { blue: 6, green: 1, red: 3 },
                    Set { blue: 0, green: 3, red: 6 },
                    Set { blue: 15, green: 3, red: 14 },
                ]),
                Game(vec![
                    Set { blue: 1, green: 3, red: 6 },
                    Set { blue: 2, green: 2, red: 1 },
                ]),
            ]
        );
    }

    #[test]
    fn first_start() {
        let games: Vec<Game> = games(TEST_INPUT);
        let games_minimum_playable_set = games.iter().map(|game| game.max());
    
        assert_eq!(games_minimum_playable_set.enumerate().filter(|(_, set)| {
            set.blue <= COUNT_SET.blue && set.green <= COUNT_SET.green && set.red <= COUNT_SET.red
        }).map(|(index, _)| index + 1).sum::<usize>(), 8);
    }

    #[test]
    fn second_star() {
        let games: Vec<Game> = games(TEST_INPUT);
        let games_minimum_playable_set = games.iter().map(|game| game.max());

        assert_eq!(games_minimum_playable_set.map(|set| set.power()).sum::<u32>(), 2286);
    }
}