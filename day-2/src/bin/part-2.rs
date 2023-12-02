#![allow(dead_code)]

use std::cmp;

fn main() {
    let input = include_str!("./input/part-1.txt");
    let games: Vec<Game> = input.lines().map(Game::from_str).collect();
    let output = get_power_sum(games);

    dbg!(output);
}

#[derive(Debug, Clone)]
struct Round {
    red: u32,
    green: u32,
    blue: u32,
}

impl Round {
    fn from_str(input: &str) -> Self {
        // Input: 1 red, 2 green, 3 blue
        let mut red: u32 = 0;
        let mut green: u32 = 0;
        let mut blue: u32 = 0;

        for line in input.split(',') {
            let parts = line.trim().split(' ').collect::<Vec<&str>>();

            match (parts[0], parts[1]) {
                (amount, "red") => red = amount.parse::<u32>().expect("valid number"),
                (amount, "green") => green = amount.parse::<u32>().expect("valid number"),
                (amount, "blue") => blue = amount.parse::<u32>().expect("valid number"),
                _ => panic!("Invalid input: {}", line),
            }
        }

        Self { red, green, blue }
    }

    fn is_possible(&self, red: &u32, green: &u32, blue: &u32) -> bool {
        self.red <= *red && self.green <= *green && self.blue <= *blue
    }
}

#[derive(Debug, Clone)]
struct Game {
    id: u32,
    rounds: Vec<Round>,
}

impl Game {
    fn from_str(input: &str) -> Self {
        let parts: Vec<&str> = input.split(':').collect();

        let details = parts[0].trim();
        let rounds_str = parts[1].trim();

        let id = details.split(' ').collect::<Vec<&str>>()[1]
            .parse::<u32>()
            .expect("valid number");

        let rounds = rounds_str
            .split(';')
            .map(Round::from_str)
            .collect::<Vec<Round>>();

        Self { id, rounds }
    }

    fn get_required(&self) -> (u32, u32, u32) {
        let mut red: u32 = 0;
        let mut green: u32 = 0;
        let mut blue: u32 = 0;

        for round in &self.rounds {
            red = cmp::max(round.red, red);
            green = cmp::max(round.green, green);
            blue = cmp::max(round.blue, blue);
        }

        (red, green, blue)
    }

    fn get_power(&self) -> u32 {
        let (r, g, b) = self.get_required();
        r * g * b
    }
}

fn get_power_sum(games: Vec<Game>) -> u32 {
    games.iter().map(|g| g.get_power()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_round() {
        let input = "3 blue, 4 red".to_string();
        let game = Round::from_str(&input);

        assert_eq!(game.red, 4);
        assert_eq!(game.green, 0);
        assert_eq!(game.blue, 3);
    }

    #[test]
    fn test_game() {
        let input = "Game 6: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green".to_string();

        let game = Game::from_str(&input);

        assert_eq!(game.id, 6);
        assert_eq!(game.rounds.len(), 3);

        assert_eq!(game.rounds[0].red, 4);
        assert_eq!(game.rounds[0].green, 0);
        assert_eq!(game.rounds[0].blue, 3);

        assert_eq!(game.rounds[1].red, 1);
        assert_eq!(game.rounds[1].green, 2);
        assert_eq!(game.rounds[1].blue, 6);

        assert_eq!(game.rounds[2].red, 0);
        assert_eq!(game.rounds[2].green, 2);
        assert_eq!(game.rounds[2].blue, 0);
    }

    #[test]
    fn test() {
        let input = include_str!("./input/test-part-1.txt");
        let games: Vec<Game> = input.lines().map(Game::from_str).collect();

        let power_sum = get_power_sum(games);

        assert_eq!(power_sum, 2286);
    }
}
