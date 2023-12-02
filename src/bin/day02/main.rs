const ACTUAL_INPUT: &str = include_str!("../../../actual_inputs/2023/02/input.txt");

type GameRecord = (u32, u32, u32);

struct Game {
    id: usize,
    records: Vec<GameRecord>,
}

fn parse_input(input: &str) -> Vec<Game> {
    input
        .trim()
        .lines()
        .enumerate()
        .map(|(id, line)| {
            let id = id + 1;
            let (_, records) = line.split_once(':').unwrap();

            let records = records
                .trim()
                .split(';')
                .map(|record| {
                    record.trim().split(',').fold((0, 0, 0), |acc, current| {
                        let (count, color) = current.trim().split_once(' ').unwrap();
                        let count = count.parse::<u32>().unwrap();

                        match color {
                            "red" => (count, acc.1, acc.2),
                            "green" => (acc.0, count, acc.2),
                            "blue" => (acc.0, acc.1, count),
                            _ => unreachable!(),
                        }
                    })
                })
                .collect::<Vec<_>>();

            Game { id, records }
        })
        .collect()
}

fn p1(input: &str) -> String {
    const LIMIT: (u32, u32, u32) = (12, 13, 14);

    parse_input(input)
        .into_iter()
        .filter(|game| {
            game.records
                .iter()
                .all(|record| record.0 <= LIMIT.0 && record.1 <= LIMIT.1 && record.2 <= LIMIT.2)
        })
        .map(|game| game.id)
        .sum::<usize>()
        .to_string()
}

fn p2(input: &str) -> String {
    parse_input(input)
        .into_iter()
        .map(|game| {
            let max_each = game.records.iter().fold((0, 0, 0), |acc, current| {
                (
                    acc.0.max(current.0),
                    acc.1.max(current.1),
                    acc.2.max(current.2),
                )
            });
            max_each.0 * max_each.1 * max_each.2
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

    const SAMPLE_INPUT: &'static str = r"
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";

    #[test]
    fn test_p1_sample() {
        assert_eq!(p1(SAMPLE_INPUT), "8");
    }

    #[test]
    fn test_p1_actual() {
        assert_eq!(p1(ACTUAL_INPUT), "2176");
    }

    #[test]
    fn test_p2_sample() {
        assert_eq!(p2(SAMPLE_INPUT), "2286");
    }

    #[test]
    fn test_p2_actual() {
        assert_eq!(p2(ACTUAL_INPUT), "63700");
    }
}
