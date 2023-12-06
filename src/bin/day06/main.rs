const ACTUAL_INPUT: &str = include_str!("../../../actual_inputs/2023/06/input.txt");

fn p1(input: &str) -> String {
    let mut iter = input.trim().lines().map(|line| {
        line.split(':')
            .nth(1)
            .unwrap()
            .trim()
            .split_whitespace()
            .map(|x| x.trim().parse::<u32>().unwrap())
            .collect::<Vec<_>>()
    });

    let time = iter.next().unwrap();
    let distances = iter.next().unwrap();

    time.into_iter()
        .zip(distances.into_iter())
        .map(|(time, distance)| {
            (0..=time)
                .map(|hold| hold * (time - hold))
                .filter(|x| *x > distance)
                .count() as u32
        })
        .product::<u32>()
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
Time:      7  15   30
Distance:  9  40  200
";

    #[test]
    fn test_p1_sample() {
        assert_eq!(p1(SAMPLE_INPUT), "288");
    }

    #[test]
    fn test_p1_actual() {
        assert_eq!(p1(ACTUAL_INPUT), "114400");
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
