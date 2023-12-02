use regex::Regex;

fn main() {
    let data = include_str!("../../data/day2.txt");
    let games = parse_games(data);
    let cubes = Cubes{
        red: 12,
        green: 13,
        blue: 14,
    };
    let possible_ids = possible_games_ids(&games, &cubes);
    let possible_ids_sum : u32 = possible_ids.iter().sum();
    println!("Task 1: {possible_ids_sum}");

    let minimum_cubes = minimum_games_cubes(&games);
    let powers_sum: u32 = minimum_cubes.iter().map(|cubes| cubes.power()).sum();
    println!("Task 2: {powers_sum}");
}

struct Cubes {
    red: u32,
    green: u32,
    blue: u32
}

impl Cubes {
    pub fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }
}

struct Game {
    id: u32,
    reveals: Vec<Cubes>
}

impl Game {
    pub fn from_str(line: &str) -> Self {
        let game_id_re = Regex::new(r"Game (\d+):").unwrap();
        let id = game_id_re.captures(line).unwrap()[1].parse::<u32>().unwrap();

        let reveals_re = Regex::new(r"(?: \d+ \w+,?)+;?").unwrap();
        let reveals: Vec<Cubes> = reveals_re.captures_iter(line).map(|captures| {
            let reveal_str = &captures[0];
            Cubes{
                red: num_colour(reveal_str, "red"),
                green: num_colour(reveal_str, "green"),
                blue: num_colour(reveal_str, "blue"),
            }
        }).collect();

        Game {
            id,
            reveals
        }
    }
}

fn num_colour(reveal_str: &str, colour: &str) -> u32 {
    let re = Regex::new(&*(r"(\d+) ".to_string() + colour)).unwrap();
    re.captures(reveal_str).map_or(0, |captures| {
        captures[1].parse::<u32>().unwrap()
    })
}

fn possible_games_ids(games: &Vec<Game>, cubes: &Cubes) -> Vec<u32> {
    games.iter().filter(|game| game_possible(game, cubes)).map(|game| game.id).collect()
}

fn game_possible(game: &Game, cubes: &Cubes) -> bool {
    for revealed_cubes in &game.reveals {
        if revealed_cubes.red > cubes.red || revealed_cubes.green > cubes.green || revealed_cubes.blue > cubes.blue {
            return false;
        }
    }

    return true;
}

fn parse_games(data: &str) -> Vec<Game> {
    data.lines().map(|line| Game::from_str(line)).collect()
}

fn minimum_games_cubes(games: &Vec<Game>) -> Vec<Cubes> {
    games.iter().map(|game| {
        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;

        for revealed in &game.reveals {
            if revealed.red > max_red {
                max_red = revealed.red;
            }

            if revealed.green > max_green {
                max_green = revealed.green;
            }

            if revealed.blue > max_blue {
                max_blue = revealed.blue;
            }
        }

        Cubes{
            red: max_red,
            green: max_green,
            blue: max_blue
        }
    }).collect()
}

#[cfg(test)]
mod tests {
    use crate::{Cubes, minimum_games_cubes, parse_games, possible_games_ids};

    #[test]
    fn task1() {
        let data = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        let games = parse_games(data);
        let cubes = Cubes{
            red: 12,
            green: 13,
            blue: 14,
        };
        let possibles = possible_games_ids(&games, &cubes);
        let sum_possible_ids: u32 = possibles.iter().sum();
        assert_eq!(sum_possible_ids, 8);
    }

    #[test]
    fn task2() {
        let data = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        let games = parse_games(data);
        let minimum_cubes = minimum_games_cubes(&games);
        let powers_sum : u32 = minimum_cubes.iter().map(|cubes| cubes.power()).sum();
        assert_eq!(powers_sum, 2286);
    }
}