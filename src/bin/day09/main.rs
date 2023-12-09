const ACTUAL_INPUT: &str = include_str!("../../../actual_inputs/2023/09/input.txt");

fn extrapolate_once(history: &[i64]) -> (i64, i64) {
    let mut diffs = vec![history.to_vec()];

    while !diffs.iter().next_back().unwrap().iter().all(|x| *x == 0) {
        diffs.push(
            diffs
                .iter()
                .next_back()
                .unwrap()
                .windows(2)
                .map(|vals| vals[1] - vals[0])
                .collect::<Vec<_>>(),
        );
    }

    diffs.iter().rev().fold((0, 0), |acc, diff| {
        (
            acc.0 + diff.iter().next_back().unwrap_or(&0),
            diff.iter().next().unwrap_or(&0) - acc.1,
        )
    })
}

fn solve(input: &str) -> (i64, i64) {
    input
        .trim()
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .map(|line| extrapolate_once(&line))
        .fold((0, 0), |acc, current| {
            (acc.0 + current.0, acc.1 + current.1)
        })
}

fn p1(input: &str) -> String {
    solve(input).0.to_string()
}

fn p2(input: &str) -> String {
    solve(input).1.to_string()
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
        assert_eq!(p2(SAMPLE_INPUT), "2");
    }

    #[test]
    fn test_p2_actual() {
        assert_eq!(p2(ACTUAL_INPUT), "1136");
    }
}
