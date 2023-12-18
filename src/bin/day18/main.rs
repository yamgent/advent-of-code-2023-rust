use std::collections::{HashSet, VecDeque};

const ACTUAL_INPUT: &str = include_str!("../../../actual_inputs/2023/18/input.txt");

#[derive(Debug, Clone, Copy)]
enum ProblemPart {
    Part1,
    Part2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn parse_direction(ch: char, problem_part: ProblemPart) -> Self {
        match problem_part {
            ProblemPart::Part1 => match ch {
                'U' => Direction::Up,
                'D' => Direction::Down,
                'L' => Direction::Left,
                'R' => Direction::Right,
                _ => unreachable!(),
            },
            ProblemPart::Part2 => match ch {
                '0' => Direction::Right,
                '1' => Direction::Down,
                '2' => Direction::Left,
                '3' => Direction::Up,
                _ => unreachable!(),
            },
        }
    }
}

#[derive(Debug, Clone)]
struct Input {
    dir: Direction,
    steps: i64,
}

impl Input {
    fn parse_input(line: &str, problem_part: ProblemPart) -> Self {
        let mut iter = line.split(' ');

        match problem_part {
            ProblemPart::Part1 => {
                let dir = Direction::parse_direction(
                    iter.next().unwrap().chars().next().unwrap(),
                    problem_part,
                );
                let steps = iter.next().unwrap().parse().unwrap();
                Self { dir, steps }
            }
            ProblemPart::Part2 => {
                let color = iter.nth(2).unwrap().replace("(", "").replace(")", "");

                let dir = Direction::parse_direction(color.chars().last().unwrap(), problem_part);
                let steps =
                    i64::from_str_radix(&color.chars().skip(1).take(5).collect::<String>(), 16)
                        .unwrap();

                Self { dir, steps }
            }
        }
    }
}

fn solve(input: &str, problem_part: ProblemPart) -> String {
    let input = input
        .trim()
        .lines()
        .map(|line| Input::parse_input(line, problem_part))
        .collect::<Vec<_>>();

    let area = input
        .iter()
        .fold(vec![(0, 0)], |mut acc, current| {
            let pos = acc.last().unwrap();

            let new_pos = match current.dir {
                Direction::Up => (pos.0, pos.1 - current.steps),
                Direction::Down => (pos.0, pos.1 + current.steps),
                Direction::Left => (pos.0 - current.steps, pos.1),
                Direction::Right => (pos.0 + current.steps, pos.1),
            };

            acc.push(new_pos);
            acc
        })
        .windows(2)
        .into_iter()
        .map(|points| points[0].0 * points[1].1 - points[1].0 * points[0].1)
        .sum::<i64>()
        // reddit says use both shoelace formula & pick's theorem. Above was
        // shoelace, below is part of the pick's theorem
        + input.iter().map(|x| x.steps).sum::<i64>()
        + 2;

    // impossible for final area to end with 0.5. There is no 0.5 hex area.
    (area / 2).to_string()
}

fn p1(input: &str) -> String {
    solve(input, ProblemPart::Part1)
}

fn p2(input: &str) -> String {
    solve(input, ProblemPart::Part2)
}

fn main() {
    println!("{}", p1(ACTUAL_INPUT));
    println!("{}", p2(ACTUAL_INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = r"
R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)
    ";

    #[test]
    fn test_p1_sample() {
        assert_eq!(p1(SAMPLE_INPUT), "62");
    }

    #[test]
    fn test_p1_actual() {
        assert_eq!(p1(ACTUAL_INPUT), "50603");
    }

    #[test]
    fn test_p2_sample() {
        assert_eq!(p2(SAMPLE_INPUT), "952408144115");
    }

    #[test]
    fn test_p2_actual() {
        assert_eq!(p2(ACTUAL_INPUT), "96556251590677");
    }
}
