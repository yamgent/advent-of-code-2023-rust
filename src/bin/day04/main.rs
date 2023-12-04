use std::collections::HashSet;

const ACTUAL_INPUT: &str = include_str!("../../../actual_inputs/2023/04/input.txt");

fn p1(input: &str) -> String {
    input
        .trim()
        .lines()
        .map(|line| {
            let (winning_numbers, numbers) = line.split_once('|').unwrap();
            let winning_numbers = winning_numbers
                .split(':')
                .nth(1)
                .unwrap()
                .trim()
                .split_whitespace()
                .map(|v| v.parse::<u32>().unwrap())
                .collect::<HashSet<_>>();
            let numbers = numbers
                .split_whitespace()
                .map(|v| v.parse::<u32>().unwrap())
                .collect::<HashSet<_>>();
            let matches = numbers.intersection(&winning_numbers).count() as u32;

            if matches == 0 {
                0
            } else {
                2u32.pow(matches - 1)
            }
        })
        .sum::<u32>()
        .to_string()
}

fn p2(input: &str) -> String {
    let _input = input.trim();
    "".to_string()
}

fn main() {
    println!("{}", p1(ACTUAL_INPUT));
    println!("{}", p2(ACTUAL_INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = r"
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
";

    #[test]
    fn test_p1_sample() {
        assert_eq!(p1(SAMPLE_INPUT), "13");
    }

    #[test]
    fn test_p1_actual() {
        assert_eq!(p1(ACTUAL_INPUT), "25651");
    }

    #[test]
    fn test_p2_sample() {
        assert_eq!(p2(SAMPLE_INPUT), "");
    }

    #[test]
    #[ignore = "not yet implemented"]
    fn test_p2_actual() {
        assert_eq!(p2(ACTUAL_INPUT), "");
    }
}
