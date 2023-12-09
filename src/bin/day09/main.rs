const ACTUAL_INPUT: &str = include_str!("../../../actual_inputs/2023/09/input.txt");

fn get_next_value(history: &[i64]) -> i64 {
    let mut diffs = vec![history.iter().copied().collect::<Vec<_>>()];

    while !diffs.iter().rev().next().unwrap().iter().all(|x| *x == 0) {
        diffs.push(
            diffs
                .iter()
                .rev()
                .next()
                .unwrap()
                .windows(2)
                .map(|vals| vals[1] - vals[0])
                .collect::<Vec<_>>(),
        );
    }

    diffs.into_iter().rev().fold(0, |acc, diff| {
        acc + diff.into_iter().rev().next().unwrap_or(0)
    })
}

fn p1(input: &str) -> String {
    input
        .trim()
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .map(|line| get_next_value(&line))
        .sum::<i64>()
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
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
";

    #[test]
    fn test_p1_sample() {
        assert_eq!(p1(SAMPLE_INPUT), "114");
    }

    #[test]
    fn test_p1_actual() {
        assert_eq!(p1(ACTUAL_INPUT), "1684566095");
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
