const ACTUAL_INPUT: &str = include_str!("../../../actual_inputs/2023/01/input.txt");

fn p1(input: &str) -> String {
    input
        .trim()
        .lines()
        .map(|line| {
            let first_digit = line.chars().find(|c| c.is_digit(10)).unwrap() as u32 - '0' as u32;
            let last_digit =
                line.chars().rev().find(|c| c.is_digit(10)).unwrap() as u32 - '0' as u32;
            first_digit * 10 + last_digit
        })
        .sum::<u32>()
        .to_string()
}

fn p2(input: &str) -> String {
    fn match_digit(line: &str, start_idx: usize) -> Option<u32> {
        const DIGITS_MAP: [(&str, u32); 19] = [
            ("0", 0),
            ("1", 1),
            ("2", 2),
            ("3", 3),
            ("4", 4),
            ("5", 5),
            ("6", 6),
            ("7", 7),
            ("8", 8),
            ("9", 9),
            ("one", 1),
            ("two", 2),
            ("three", 3),
            ("four", 4),
            ("five", 5),
            ("six", 6),
            ("seven", 7),
            ("eight", 8),
            ("nine", 9),
        ];

        let line = &line[start_idx..];

        DIGITS_MAP
            .iter()
            .find(|(text, _)| line.starts_with(text))
            .map(|(_, digit)| *digit)
    }

    input
        .trim()
        .lines()
        .map(|line| {
            let first_digit = (0..line.len())
                .find_map(|start_idx| match_digit(line, start_idx))
                .unwrap();

            let last_digit = (0..line.len())
                .rev()
                .find_map(|start_idx| match_digit(line, start_idx))
                .unwrap();

            first_digit * 10 + last_digit
        })
        .sum::<u32>()
        .to_string()
}

fn main() {
    println!("{}", p1(ACTUAL_INPUT));
    println!("{}", p2(ACTUAL_INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1_sample() {
        assert_eq!(
            p1(r"
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
"),
            "142"
        );
    }

    #[test]
    fn test_p1_actual() {
        assert_eq!(p1(ACTUAL_INPUT), "54304");
    }

    #[test]
    fn test_p2_sample() {
        assert_eq!(
            p2(r"
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
"),
            "281"
        );
    }

    #[test]
    fn test_p2_actual() {
        assert_eq!(p2(ACTUAL_INPUT), "54418");
    }
}
