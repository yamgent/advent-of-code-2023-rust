use std::collections::{HashSet, VecDeque};

const ACTUAL_INPUT: &str = include_str!("../../../actual_inputs/2023/18/input.txt");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn parse_direction(ch: char) -> Self {
        match ch {
            'U' => Direction::Up,
            'D' => Direction::Down,
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone)]
struct Input {
    dir: Direction,
    steps: i32,
    color: String,
}

impl Input {
    fn parse_input(line: &str) -> Self {
        let mut iter = line.split(' ');

        let dir = Direction::parse_direction(iter.next().unwrap().chars().next().unwrap());
        let steps = iter.next().unwrap().parse().unwrap();
        let color = iter.next().unwrap().replace("(", "").replace(")", "");

        Self { dir, steps, color }
    }
}

#[allow(dead_code)]
fn dug_to_string(dug: &HashSet<(i32, i32)>, min: &(i32, i32), max: &(i32, i32)) -> String {
    (min.1..=max.1).fold(String::new(), |mut acc, y| {
        (min.0..=max.0).for_each(|x| {
            acc.push(if dug.contains(&(x, y)) { '#' } else { '.' });
        });
        acc.push('\n');
        acc
    })
}

fn p1(input: &str) -> String {
    let (_, dug, min, max) = input.trim().lines().map(Input::parse_input).fold(
        ((0, 0), HashSet::<(i32, i32)>::new(), (-1, -1), (1, 1)),
        |(mut pos, mut dug, mut min, mut max), current| {
            (0..current.steps).for_each(|_| {
                pos = match current.dir {
                    Direction::Up => (pos.0, pos.1 - 1),
                    Direction::Down => (pos.0, pos.1 + 1),
                    Direction::Left => (pos.0 - 1, pos.1),
                    Direction::Right => (pos.0 + 1, pos.1),
                };
                dug.insert(pos);
                min = (min.0.min(pos.0 - 1), min.1.min(pos.1 - 1));
                max = (max.0.max(pos.0 + 1), max.1.max(pos.1 + 1));
            });

            (pos, dug, min, max)
        },
    );

    fn in_bounds(pos: &(i32, i32), min: &(i32, i32), max: &(i32, i32)) -> bool {
        pos.0 >= min.0 && pos.1 >= min.1 && pos.0 <= max.0 && pos.1 <= max.1
    }

    let mut outside = HashSet::<(i32, i32)>::new();
    let mut queue = VecDeque::from_iter([min]);

    while let Some(next) = queue.pop_front() {
        if !in_bounds(&next, &min, &max) || outside.contains(&next) || dug.contains(&next) {
            continue;
        }

        outside.insert(next);
        queue.push_back((next.0 - 1, next.1));
        queue.push_back((next.0 + 1, next.1));
        queue.push_back((next.0, next.1 - 1));
        queue.push_back((next.0, next.1 + 1));
    }

    (min.0..=max.0)
        .flat_map(|x| (min.1..=max.1).map(move |y| (x, y)))
        .filter(|coord| !outside.contains(coord))
        .count()
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
        assert_eq!(p2(SAMPLE_INPUT), "");
    }

    #[test]
    #[ignore = "not yet implemented"]
    fn test_p2_actual() {
        assert_eq!(p2(ACTUAL_INPUT), "");
    }
}
