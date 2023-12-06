use crate::common::ScratchCard;
use std::str::FromStr;

pub fn solution(input: &str) -> usize {
    let cards_itr = input
        .lines()
        .map(ScratchCard::from_str)
        .filter_map(Result::ok);

    let mut result: Vec<ScratchCard> = cards_itr.clone().collect();

    cards_itr
        .enumerate()
        .filter(|(_, card)| card.get_matching_numbers() != 0)
        .for_each(|(idx, card)| {
            for new_idx in idx..idx + card.get_matching_numbers() {
                let curr = result.get(idx).expect("card to exist").count;
                let target = result.get_mut(new_idx + 1).expect("card to exist");

                target.count += curr;
            }
        });

    result.iter().map(|x| x.count).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_result() {
        let data = include_str!("../data/test.txt");
        let result = solution(data);

        assert_eq!(result, 30);
    }
}
