use crate::common::{get_games_from_input, Game};

pub fn solution(input: &str) -> u32 {
    let games = get_games_from_input(input);
    get_possible_games(games, 12, 13, 14)
        .iter()
        .map(|g| g.id)
        .sum::<u32>()
}

pub fn get_possible_games(games: Vec<Game>, red: u32, green: u32, blue: u32) -> Vec<Game> {
    games
        .iter()
        .filter(|game| {
            game.rounds
                .iter()
                .all(|r| r.is_possible(&red, &green, &blue))
        })
        .cloned()
        .collect::<Vec<Game>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let input = include_str!("../data/test-part-1.txt");
        assert_eq!(solution(input), 8);
    }
}
