use std::cmp::max;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct Round {
    red: u32,
    green: u32,
    blue: u32,
}

#[derive(Debug, Copy, Clone)]
pub struct RoundParsingError;

impl FromStr for Round {
    type Err = RoundParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut red: u32 = 0;
        let mut green: u32 = 0;
        let mut blue: u32 = 0;

        for line in s.split(',') {
            let parts = line.trim().split(' ').collect::<Vec<&str>>();

            match (parts[0], parts[1]) {
                (amount, "red") => red = amount.parse::<u32>().expect("valid number"),
                (amount, "green") => green = amount.parse::<u32>().expect("valid number"),
                (amount, "blue") => blue = amount.parse::<u32>().expect("valid number"),
                _ => return Err(RoundParsingError),
            }
        }

        Ok(Self { red, green, blue })
    }
}

impl Round {
    pub fn is_possible(&self, red: &u32, green: &u32, blue: &u32) -> bool {
        self.red <= *red && self.green <= *green && self.blue <= *blue
    }
}

#[derive(Debug, Clone)]
pub struct Game {
    pub id: u32,
    pub rounds: Vec<Round>,
}

#[derive(Debug, Clone, Copy)]
pub struct GameParsingError;

impl FromStr for Game {
    type Err = GameParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(':').collect();

        let details = parts[0].trim();
        let rounds_str = parts[1].trim();

        let id = details
            .split(' ')
            .last()
            .expect("id should exist")
            .parse::<u32>()
            .expect("valid number");

        let rounds = rounds_str
            .split(';')
            .map(Round::from_str)
            .filter_map(Result::ok)
            .collect::<Vec<Round>>();

        Ok(Self { id, rounds })
    }
}

impl Game {
    pub fn get_required(&self) -> (u32, u32, u32) {
        let mut red: u32 = 0;
        let mut green: u32 = 0;
        let mut blue: u32 = 0;

        for round in &self.rounds {
            red = max(round.red, red);
            green = max(round.green, green);
            blue = max(round.blue, blue);
        }

        (red, green, blue)
    }

    pub fn get_power(&self) -> u32 {
        let (r, g, b) = self.get_required();
        r * g * b
    }
}

pub fn get_games_from_input(input: &str) -> Vec<Game> {
    input
        .lines()
        .map(Game::from_str)
        .filter_map(Result::ok)
        .collect::<Vec<Game>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_round() {
        let input = "3 blue, 4 red".to_string();
        let game = Round::from_str(&input).expect("Game Exists");

        assert_eq!(game.red, 4);
        assert_eq!(game.green, 0);
        assert_eq!(game.blue, 3);
    }

    #[test]
    fn test_game() {
        let input = "Game 6: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green".to_string();

        let game = Game::from_str(&input).expect("Game Exists");

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
}
