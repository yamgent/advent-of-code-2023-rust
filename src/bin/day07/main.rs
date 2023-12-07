use std::collections::HashMap;

const ACTUAL_INPUT: &str = include_str!("../../../actual_inputs/2023/07/input.txt");

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Hand {
    hand_type: HandType,
    hand: [u8; 5],
}

impl Hand {
    fn from_input(input: &str) -> Self {
        let hand: [u8; 5] = input
            .trim()
            .chars()
            .map(|c| match c {
                '2' => 2,
                '3' => 3,
                '4' => 4,
                '5' => 5,
                '6' => 6,
                '7' => 7,
                '8' => 8,
                '9' => 9,
                'T' => 10,
                'J' => 11,
                'Q' => 12,
                'K' => 13,
                'A' => 14,
                _ => unreachable!(),
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        let hand_type = {
            let counters = hand
                .iter()
                .fold(HashMap::new(), |mut acc, current| {
                    acc.entry(current).and_modify(|c| *c += 1).or_insert(1);
                    acc
                })
                .into_values()
                .collect::<Vec<_>>();

            if counters.iter().any(|v| *v == 5) {
                HandType::FiveOfAKind
            } else if counters.iter().any(|v| *v == 4) {
                HandType::FourOfAKind
            } else if counters.iter().any(|v| *v == 3) && counters.iter().any(|v| *v == 2) {
                HandType::FullHouse
            } else if counters.iter().any(|v| *v == 3) {
                HandType::ThreeOfAKind
            } else if counters.iter().filter(|v| **v == 2).count() == 2 {
                HandType::TwoPair
            } else if counters.iter().any(|v| *v == 2) {
                HandType::OnePair
            } else {
                HandType::HighCard
            }
        };

        Self { hand, hand_type }
    }
}

fn p1(input: &str) -> String {
    let mut hands = input
        .trim()
        .lines()
        .map(|line| line.split_once(' ').unwrap())
        .map(|line| (Hand::from_input(line.0), line.1.parse::<u32>().unwrap()))
        .collect::<Vec<_>>();

    hands.sort();

    hands
        .into_iter()
        .enumerate()
        .map(|(rank, (_, bid))| (rank as u32 + 1) * bid)
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
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
";

    #[test]
    fn test_p1_sample() {
        assert_eq!(p1(SAMPLE_INPUT), "6440");
    }

    #[test]
    fn test_p1_actual() {
        assert_eq!(p1(ACTUAL_INPUT), "253866470");
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
