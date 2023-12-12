const ACTUAL_INPUT: &str = include_str!("../../../actual_inputs/2023/12/input.txt");

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Status {
    // '.'
    Operational,
    // '#'
    Damaged,
    // '?'
    Unknown,
}

impl Status {
    fn parse(ch: char) -> Self {
        match ch {
            '.' => Status::Operational,
            '#' => Status::Damaged,
            '?' => Status::Unknown,
            _ => unreachable!(),
        }
    }

    fn apply(statuses: &[Self], permutation: u64) -> Vec<Self> {
        statuses
            .iter()
            .fold(
                (Vec::new(), permutation),
                |(mut result, permutation), current_status| {
                    if *current_status == Status::Unknown {
                        let assigned = permutation % 2;
                        result.push(if assigned == 0 {
                            Status::Damaged
                        } else {
                            Status::Operational
                        });
                        (result, permutation / 2)
                    } else {
                        result.push(*current_status);
                        (result, permutation)
                    }
                },
            )
            .0
    }

    fn to_counts(statuses: &[Status]) -> Vec<u64> {
        let (damaged, mut acc) = statuses.iter().fold(
            (0, Vec::new()),
            |(damaged, mut acc), current| match current {
                Status::Operational => {
                    if damaged > 0 {
                        acc.push(damaged);
                    }
                    (0, acc)
                }
                Status::Damaged => (damaged + 1, acc),
                Status::Unknown => {
                    panic!("Do not use this method with unknowns!");
                }
            },
        );

        if damaged > 0 {
            acc.push(damaged);
        }

        acc
    }
}

struct Input {
    statuses: Vec<Status>,
    counts: Vec<u64>,
}

fn statuses_match_counts(statuses: &[Status], counts: &[u64]) -> bool {
    Status::to_counts(statuses).as_slice() == counts
}

impl Input {
    fn parse(line: &str) -> Self {
        let (statuses, counts) = line.split_once(' ').unwrap();

        Self {
            statuses: statuses.chars().map(Status::parse).collect::<Vec<_>>(),
            counts: counts
                .split(',')
                .map(|x| x.parse::<u64>().unwrap())
                .collect::<Vec<_>>(),
        }
    }

    fn unknowns_count(&self) -> u32 {
        self.statuses
            .iter()
            .filter(|x| **x == Status::Unknown)
            .count() as u32
    }

    fn is_valid_permutation(&self, permutation: u64) -> bool {
        let new_statuses = Status::apply(&self.statuses, permutation);
        statuses_match_counts(&new_statuses, &self.counts)
    }
}

fn solve_p1(input: Input) -> u64 {
    let unknowns_count = input.unknowns_count();

    if unknowns_count == 0 {
        1
    } else {
        (0..2_u64.pow(unknowns_count))
            .filter(|permutation| input.is_valid_permutation(*permutation))
            .count() as u64
    }
}

fn p1(input: &str) -> String {
    input
        .trim()
        .lines()
        .map(Input::parse)
        .map(solve_p1)
        .sum::<u64>()
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
???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
";

    #[test]
    fn test_p1_sample() {
        assert_eq!(p1(SAMPLE_INPUT), "21");
    }

    #[test]
    fn test_p1_actual() {
        assert_eq!(p1(ACTUAL_INPUT), "8180");
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
