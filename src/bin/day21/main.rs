use std::collections::HashSet;

const ACTUAL_INPUT: &str = include_str!("../../../actual_inputs/2023/21/input.txt");

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Vec2u {
    x: usize,
    y: usize,
}

impl Vec2u {
    fn left(&self) -> Option<Self> {
        if self.x == 0 {
            None
        } else {
            Some(Vec2u {
                x: self.x - 1,
                y: self.y,
            })
        }
    }

    fn right(&self) -> Option<Self> {
        Some(Vec2u {
            x: self.x + 1,
            y: self.y,
        })
    }

    fn up(&self) -> Option<Self> {
        if self.y == 0 {
            None
        } else {
            Some(Vec2u {
                x: self.x,
                y: self.y - 1,
            })
        }
    }

    fn down(&self) -> Option<Self> {
        Some(Vec2u {
            x: self.x,
            y: self.y + 1,
        })
    }
}

#[derive(Debug)]
struct Input {
    size: Vec2u,
    start: Vec2u,
    rocks: HashSet<Vec2u>,
}

impl Input {
    fn parse(input: &str) -> Self {
        let size = Vec2u {
            x: input.trim().lines().next().unwrap().len(),
            y: input.trim().lines().count(),
        };
        let start = input
            .trim()
            .lines()
            .enumerate()
            .find_map(|(y, line)| {
                line.trim()
                    .chars()
                    .enumerate()
                    .find(|(_, ch)| *ch == 'S')
                    .map(|(x, _)| Vec2u { x, y })
            })
            .unwrap();
        let rocks = input
            .trim()
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.trim()
                    .chars()
                    .enumerate()
                    .filter(|(_, ch)| *ch == '#')
                    .map(|(x, _)| Vec2u { x, y })
                    .collect::<Vec<_>>()
            })
            .collect::<HashSet<_>>();

        Self { size, start, rocks }
    }
}

fn execute_p1(input: &str, steps: usize) -> String {
    let input = Input::parse(input);

    (0..steps)
        .fold(HashSet::<Vec2u>::from_iter([input.start]), |acc, _| {
            HashSet::from_iter(
                acc.iter()
                    .flat_map(|point| {
                        [point.left(), point.right(), point.up(), point.down()]
                            .iter()
                            .flatten()
                            .copied()
                            .collect::<Vec<_>>()
                    })
                    .filter(|pos| pos.x < input.size.x && pos.y < input.size.y)
                    .filter(|pos| !input.rocks.contains(&pos)),
            )
        })
        .len()
        .to_string()
}

fn p1(input: &str) -> String {
    execute_p1(input, 64)
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
...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........
    ";

    #[test]
    fn test_p1_sample() {
        assert_eq!(execute_p1(SAMPLE_INPUT, 6), "16");
        assert_eq!(p1(SAMPLE_INPUT), "42");
    }

    #[test]
    fn test_p1_actual() {
        assert_eq!(p1(ACTUAL_INPUT), "3689");
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
