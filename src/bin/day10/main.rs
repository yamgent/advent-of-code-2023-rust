use std::collections::{HashMap, HashSet};

const ACTUAL_INPUT: &str = include_str!("../../../actual_inputs/2023/10/input.txt");

type Coord = (i32, i32);

struct Map {
    content: HashMap<Coord, char>,
    starting_point: Coord,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn opposite(&self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

trait Neighbour {
    fn neighbour(&self, dir: Direction) -> Coord;
}

impl Neighbour for Coord {
    fn neighbour(&self, dir: Direction) -> Coord {
        match dir {
            Direction::Up => (self.0, self.1 - 1),
            Direction::Down => (self.0, self.1 + 1),
            Direction::Left => (self.0 - 1, self.1),
            Direction::Right => (self.0 + 1, self.1),
        }
    }
}

impl Map {
    fn parse_input(input: &str) -> Self {
        let content = HashMap::from_iter(input.trim().lines().enumerate().flat_map(|(y, line)| {
            line.trim()
                .chars()
                .enumerate()
                .map(move |(x, c)| ((x as i32, y as i32), c))
        }));

        let starting_point = content
            .iter()
            .find(|(_, ch)| **ch == 'S')
            .unwrap()
            .0
            .clone();

        fn fix_starting_point_pipe(mut map: Map) -> Map {
            let starting_point_pipe = {
                let (left, right, up, down) = (
                    map.has_exit(
                        map.starting_point.neighbour(Direction::Left),
                        Direction::Right,
                    ),
                    map.has_exit(
                        map.starting_point.neighbour(Direction::Right),
                        Direction::Left,
                    ),
                    map.has_exit(map.starting_point.neighbour(Direction::Up), Direction::Down),
                    map.has_exit(map.starting_point.neighbour(Direction::Down), Direction::Up),
                );

                if left && up {
                    'J'
                } else if left && down {
                    '7'
                } else if right && up {
                    'L'
                } else if right && down {
                    'F'
                } else {
                    unreachable!()
                }
            };
            *map.content.get_mut(&map.starting_point).unwrap() = starting_point_pipe;
            map
        }

        fix_starting_point_pipe(Self {
            content,
            starting_point,
        })
    }

    fn has_exit(&self, pos: Coord, dir: Direction) -> bool {
        self.content
            .get(&pos)
            .map(|ch| match dir {
                Direction::Up => *ch == '|' || *ch == 'L' || *ch == 'J',
                Direction::Down => *ch == '|' || *ch == '7' || *ch == 'F',
                Direction::Left => *ch == '-' || *ch == 'J' || *ch == '7',
                Direction::Right => *ch == '-' || *ch == 'L' || *ch == 'F',
            })
            .unwrap_or(false)
    }

    fn is_connected(&self, pos: Coord, dir: Direction) -> bool {
        let neighbour = pos.neighbour(dir);
        self.has_exit(pos, dir) && self.has_exit(neighbour, dir.opposite())
    }
}

fn p1(input: &str) -> String {
    let map = Map::parse_input(input);

    let mut visited: HashSet<Coord> = HashSet::from_iter([map.starting_point]);
    let mut level = -1;
    let mut current = vec![map.starting_point];

    while !current.is_empty() {
        level += 1;

        let mut next = vec![];

        current.iter().for_each(|pos| {
            visited.insert(*pos);

            [
                Direction::Up,
                Direction::Down,
                Direction::Left,
                Direction::Right,
            ]
            .into_iter()
            .for_each(|dir| {
                let neighbour = pos.neighbour(dir);
                if !visited.contains(&neighbour) && map.is_connected(*pos, dir) {
                    next.push(neighbour);
                }
            });
        });

        current = next;
    }

    level.to_string()
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

    const SAMPLE_INPUT_P1: [&str; 4] = [
        r"
.....
.S-7.
.|.|.
.L-J.
.....
",
        r"
-L|F7
7S-7|
L|7||
-L-J|
L|-JF
",
        r"
..F7.
.FJ|.
SJ.L7
|F--J
LJ...
",
        r"
7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ
",
    ];

    #[test]
    fn test_p1_sample() {
        assert_eq!(p1(SAMPLE_INPUT_P1[0]), "4");
        assert_eq!(p1(SAMPLE_INPUT_P1[1]), "4");
        assert_eq!(p1(SAMPLE_INPUT_P1[2]), "8");
        assert_eq!(p1(SAMPLE_INPUT_P1[3]), "8");
    }

    #[test]
    fn test_p1_actual() {
        assert_eq!(p1(ACTUAL_INPUT), "7097");
    }

    #[test]
    fn test_p2_sample() {
        assert_eq!(p2(SAMPLE_INPUT_P1[0]), "");
    }

    #[test]
    #[ignore = "not yet implemented"]
    fn test_p2_actual() {
        assert_eq!(p2(ACTUAL_INPUT), "");
    }
}
