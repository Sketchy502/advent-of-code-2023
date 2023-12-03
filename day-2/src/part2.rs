use crate::common::get_games_from_input;

pub fn solution(input: &str) -> u32 {
    let games = get_games_from_input(input);
    games.iter().map(|g| g.get_power()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = include_str!("../data/test-part-1.txt");
        let power_sum = solution(input);

        assert_eq!(power_sum, 2286);
    }
}
