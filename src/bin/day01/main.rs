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
        assert_eq!(p2(""), "");
    }

    #[test]
    #[ignore = "not yet implemented"]
    fn test_p2_actual() {
        assert_eq!(p2(ACTUAL_INPUT), "");
    }
}
