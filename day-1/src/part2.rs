pub fn solution(input: &str) -> u32 {
    input.lines().map(get_numbers).sum()
}

fn pre_process(input: &str) -> String {
    (0..input.len())
        .map(|i| {
            let sub_str = &input[i..];

            if sub_str.starts_with("one") {
                return '1';
            }
            if sub_str.starts_with("two") {
                return '2';
            }
            if sub_str.starts_with("three") {
                return '3';
            }
            if sub_str.starts_with("four") {
                return '4';
            }
            if sub_str.starts_with("five") {
                return '5';
            }
            if sub_str.starts_with("six") {
                return '6';
            }
            if sub_str.starts_with("seven") {
                return '7';
            }
            if sub_str.starts_with("eight") {
                return '8';
            }
            if sub_str.starts_with("nine") {
                return '9';
            }

            sub_str.chars().next().unwrap()
        })
        .collect::<String>()
}

fn get_numbers(input: &str) -> u32 {
    let val = pre_process(input);
    let mut iter = val.chars().filter_map(|c| c.to_digit(10));
    //
    //get the first and last number
    let first = iter.next().expect("valid number");
    let last = iter.last().unwrap_or(first);
    //
    // format the parts to a single digit
    format!("{first}{last}")
        .parse::<u32>()
        .expect("failed to parse")
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("two1nine", 29)]
    #[case("eightwothree", 83)]
    #[case("abcone2threexyz", 13)]
    #[case("xtwone3four", 24)]
    #[case("4nineeightseven2", 42)]
    #[case("zoneight234", 14)]
    #[case("7pqrstsixteen", 76)]
    #[case("7pqrstoneight", 78)]
    fn test_part1(#[case] input: &str, #[case] expected: u32) {
        assert_eq!(get_numbers(input), expected);
    }

    #[test]
    fn test_sum() {
        let input = include_str!("../data/test-2.txt");
        assert_eq!(solution(input), 281);
    }
}
