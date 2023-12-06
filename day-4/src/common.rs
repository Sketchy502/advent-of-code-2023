use std::{collections::HashSet, str::FromStr};

#[derive(Debug, Clone)]
pub struct ScratchCard {
    winning_numbers: HashSet<usize>,
    card_numbers: Vec<usize>,
    pub count: usize,
}

pub struct ScratchCardError;

impl FromStr for ScratchCard {
    type Err = ScratchCardError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = input.split(" | ").collect();

        let first_parts: Vec<&str> = parts[0].split(':').collect();

        let winning_numbers: HashSet<usize> = first_parts[1]
            .trim()
            .split(' ')
            .map(|p| p.parse::<usize>())
            .filter_map(Result::ok)
            .collect();

        let card_numbers: Vec<usize> = parts[1]
            .split(' ')
            .map(|p| p.parse::<usize>())
            .filter_map(Result::ok)
            .collect();

        Ok(Self {
            winning_numbers,
            card_numbers,
            count: 1,
        })
    }
}

impl ScratchCard {
    pub fn get_matching_numbers(&self) -> usize {
        self.card_numbers
            .iter()
            .filter(|p| self.winning_numbers.contains(p))
            .count()
    }

    pub fn get_points(&self) -> usize {
        let base: usize = 2;
        let matching_numbers = self.get_matching_numbers();
        if matching_numbers == 0 {
            return 0;
        }
        base.pow((matching_numbers - 1) as u32)
    }
}
