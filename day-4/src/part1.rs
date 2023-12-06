use crate::common::ScratchCard;
use std::str::FromStr;

pub fn solution(input: &str) -> usize {
    let cards = input
        .lines()
        .map(ScratchCard::from_str)
        .filter_map(Result::ok);

    cards.map(|card| card.get_points()).sum::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_result() {
        let data = include_str!("../data/test.txt");
        let result = solution(data);

        assert_eq!(result, 13);
    }
}
