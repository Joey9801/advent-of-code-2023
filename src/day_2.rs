use std::str::FromStr;

use anyhow::anyhow;

#[derive(Debug)]
pub struct Guess {
    red: u32,
    green: u32,
    blue: u32,
}

#[derive(Debug)]
pub struct Game {
    id: u32,
    guesses: Vec<Guess>,
}

impl FromStr for Game {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Game str like "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"
        //  => id = 1, guesses = vec![
        //      Guess { red: 4, blue: 3, green: 0 },
        //      Guess { red: 1, blue: 6, green: 2 },
        //      Guess { red: 0, blue: 0, green: 2 }
        //  ]

        let (game_id_str, guesses_str) = s
            .split_once(": ")
            .ok_or_else(|| anyhow!("Invalid game string"))?;

        let id = game_id_str
            .strip_prefix("Game ")
            .ok_or_else(|| anyhow!("Invalid game string"))?
            .parse::<u32>()?;

        let mut guesses = Vec::new();
        for guess_str in guesses_str.split(';') {
            let mut guess = Guess {
                red: 0,
                green: 0,
                blue: 0,
            };
            for color_count_str in guess_str.split(',') {
                let (count_str, color_str) = color_count_str
                    .trim()
                    .split_once(' ')
                    .ok_or_else(|| anyhow!("Invalid guess string"))?;

                let count = count_str.parse::<u32>()?;
                match color_str {
                    "red" => guess.red = count,
                    "green" => guess.green = count,
                    "blue" => guess.blue = count,
                    _ => return Err(anyhow!("Invalid color string")),
                }
            }
            guesses.push(guess);
        }

        Ok(Game { id, guesses })
    }
}

pub fn parse(input: &str) -> Vec<Game> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

pub fn solve_part_1(input: &[Game]) -> u32 {
    input
        .iter()
        .filter(|game| {
            game.guesses
                .iter()
                .all(|guess| guess.red <= 12 && guess.green <= 13 && guess.blue <= 14)
        })
        .map(|g| g.id)
        .sum()
}

pub fn solve_part_2(input: &[Game]) -> u32 {
    let mut sum = 0;
    for game in input {
        // The maximum red guess, green guess, and blue guess in this game:
        let max_red = game.guesses.iter().map(|g| g.red).max().unwrap_or(0);
        let max_green = game.guesses.iter().map(|g| g.green).max().unwrap_or(0);
        let max_blue = game.guesses.iter().map(|g| g.blue).max().unwrap_or(0);

        sum += max_red * max_green * max_blue;
    }

    sum
}
