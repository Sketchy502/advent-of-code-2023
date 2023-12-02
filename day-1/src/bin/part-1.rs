fn main() {
    let input = include_str!("./input/input-1.txt");
    let output = part1(input);

    dbg!(output);
}

fn get_numbers(input: &str) -> u32 {
    let mut iter = input.chars().filter_map(|c| c.to_digit(10));

    //get the first and last number
    let first = iter.next().expect("valid number");
    let last = iter.last().unwrap_or(first);

    // format the parts to a single digit
    format!("{first}{last}")
        .parse::<u32>()
        .expect("failed to parse")
}

fn part1(input: &str) -> u32 {
    input.lines().map(get_numbers).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("1abc2", 12)]
    #[case("pqr3stu8vwx", 38)]
    #[case("a1b2c3d4e5f", 15)]
    #[case("treb7uchet", 77)]
    fn test_part1(#[case] input: &str, #[case] expected: u32) {
        assert_eq!(get_numbers(input), expected);
    }

    #[test]
    fn test_sum() {
        let input = include_str!("./input/test-1.txt");
        assert_eq!(part1(input), 142);
    }
}
