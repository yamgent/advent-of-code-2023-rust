const ACTUAL_INPUT: &str = include_str!("../../../actual_inputs/2023/02/input.txt");

fn p1(input: &str) -> String {
    const LIMIT: (u32, u32, u32) = (12, 13, 14);

    input
        .trim()
        .lines()
        .enumerate()
        .flat_map(|(id, line)| {
            let id = id + 1;
            let (_, records) = line.split_once(':').unwrap();

            let exceed = records
                .trim()
                .split(';')
                .find(|record| {
                    let record = record.trim().split(',').fold((0, 0, 0), |acc, current| {
                        let (count, color) = current.trim().split_once(' ').unwrap();
                        let count = count.parse::<u32>().unwrap();

                        match color {
                            "red" => (count, acc.1, acc.2),
                            "green" => (acc.0, count, acc.2),
                            "blue" => (acc.0, acc.1, count),
                            _ => unreachable!(),
                        }
                    });

                    record.0 > LIMIT.0 || record.1 > LIMIT.1 || record.2 > LIMIT.2
                })
                .is_some();

            if exceed {
                None
            } else {
                Some(id)
            }
        })
        .sum::<usize>()
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
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
"),
            "8"
        );
    }

    #[test]
    fn test_p1_actual() {
        assert_eq!(p1(ACTUAL_INPUT), "2176");
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
