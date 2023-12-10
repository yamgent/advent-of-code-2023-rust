use std::collections::{HashMap, HashSet};

const ACTUAL_INPUT: &str = include_str!("../../../actual_inputs/2023/10/input.txt");

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Pipe {
    None,
    Starting,
    NS,
    EW,
    NE,
    SE,
    NW,
    SW,
}

impl Pipe {
    fn parse(ch: char) -> Self {
        match ch {
            '.' => Pipe::None,
            'S' => Pipe::Starting,
            '|' => Pipe::NS,
            '-' => Pipe::EW,
            'L' => Pipe::NE,
            'F' => Pipe::SE,
            'J' => Pipe::NW,
            '7' => Pipe::SW,
            _ => unreachable!(),
        }
    }
}

type Coord = (i32, i32);

struct Map {
    width: i32,
    height: i32,
    content: HashMap<Coord, Pipe>,
    starting_point: Coord,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
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
    fn neighbour(&self, dir: Direction) -> Self;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct CoordHalf {
    x: i32,
    x_half: bool,
    y: i32,
    y_half: bool,
}

impl Neighbour for CoordHalf {
    fn neighbour(&self, dir: Direction) -> CoordHalf {
        let mut result = *self;

        match dir {
            Direction::Up => {
                if !result.y_half {
                    result.y -= 1;
                }
                result.y_half = !result.y_half;
            }
            Direction::Down => {
                if result.y_half {
                    result.y += 1;
                }
                result.y_half = !result.y_half;
            }
            Direction::Left => {
                if !result.x_half {
                    result.x -= 1;
                }
                result.x_half = !result.x_half;
            }
            Direction::Right => {
                if result.x_half {
                    result.x += 1;
                }
                result.x_half = !result.x_half;
            }
        }

        result
    }
}

fn coord_to_half(coord: &Coord) -> CoordHalf {
    CoordHalf {
        x: coord.0,
        x_half: false,
        y: coord.1,
        y_half: false,
    }
}

fn half_to_coord(coord_half: &CoordHalf) -> Option<Coord> {
    if coord_half.x_half || coord_half.y_half {
        None
    } else {
        Some((coord_half.x, coord_half.y))
    }
}

impl Map {
    fn parse_input(input: &str) -> Self {
        let content = HashMap::from_iter(input.trim().lines().enumerate().flat_map(|(y, line)| {
            line.trim()
                .chars()
                .enumerate()
                .map(move |(x, c)| ((x as i32, y as i32), Pipe::parse(c)))
        }));

        let starting_point = *content
            .iter()
            .find(|(_, ch)| **ch == Pipe::Starting)
            .unwrap()
            .0;

        fn fix_starting_point_pipe(mut map: Map) -> Map {
            let starting_point_pipe = {
                let exits: HashMap<Direction, bool> = HashMap::from_iter(
                    [
                        Direction::Up,
                        Direction::Down,
                        Direction::Left,
                        Direction::Right,
                    ]
                    .into_iter()
                    .map(|dir| {
                        (
                            dir,
                            map.has_exit(map.starting_point.neighbour(dir), dir.opposite()),
                        )
                    }),
                );
                let (left, right, up, down) = (
                    *exits.get(&Direction::Left).unwrap(),
                    *exits.get(&Direction::Right).unwrap(),
                    *exits.get(&Direction::Up).unwrap(),
                    *exits.get(&Direction::Down).unwrap(),
                );

                if left && up {
                    Pipe::NW
                } else if left && down {
                    Pipe::SW
                } else if right && up {
                    Pipe::NE
                } else if right && down {
                    Pipe::SE
                } else {
                    unreachable!()
                }
            };
            *map.content.get_mut(&map.starting_point).unwrap() = starting_point_pipe;
            map
        }

        fix_starting_point_pipe(Self {
            width: input.trim().lines().next().unwrap().chars().count() as i32,
            height: input.trim().lines().count() as i32,
            content,
            starting_point,
        })
    }

    fn has_exit(&self, pos: Coord, dir: Direction) -> bool {
        self.content
            .get(&pos)
            .map(|ch| match dir {
                Direction::Up => matches!(*ch, Pipe::NS | Pipe::NE | Pipe::NW),
                Direction::Down => matches!(*ch, Pipe::NS | Pipe::SE | Pipe::SW),
                Direction::Left => matches!(*ch, Pipe::EW | Pipe::NW | Pipe::SW),
                Direction::Right => matches!(*ch, Pipe::EW | Pipe::NE | Pipe::SE),
            })
            .unwrap_or(false)
    }

    fn is_connected(&self, pos: Coord, dir: Direction) -> bool {
        let neighbour = pos.neighbour(dir);
        self.has_exit(pos, dir) && self.has_exit(neighbour, dir.opposite())
    }

    fn clean_up_pipes_not_in_loop(&self) -> (Self, i32) {
        let mut result = Self {
            width: self.width,
            height: self.height,
            content: self
                .content
                .iter()
                .map(|(coord, _)| (*coord, Pipe::None))
                .collect(),
            starting_point: self.starting_point,
        };

        let mut visited: HashSet<Coord> = HashSet::new();
        let mut level = -1;
        let mut current: HashSet<Coord> = HashSet::from_iter([self.starting_point]);

        while !current.is_empty() {
            level += 1;

            visited.extend(&current);
            current = current
                .into_iter()
                .flat_map(|pos| {
                    [
                        Direction::Up,
                        Direction::Down,
                        Direction::Left,
                        Direction::Right,
                    ]
                    .into_iter()
                    .flat_map(|dir| {
                        let neighbour = pos.neighbour(dir);
                        if !visited.contains(&neighbour) && self.is_connected(pos, dir) {
                            Some(neighbour)
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>()
                })
                .collect()
        }

        visited.into_iter().for_each(|coord| {
            *result.content.get_mut(&coord).unwrap() = *self.content.get(&coord).unwrap();
        });

        (result, level)
    }

    fn get_pipe_at_coord_half(&self, coord_half: &CoordHalf) -> Option<Pipe> {
        match half_to_coord(coord_half) {
            None => {
                if coord_half.x_half && coord_half.y_half {
                    if coord_half.x < 0
                        || coord_half.x > self.width
                        || coord_half.y < 0
                        || coord_half.y > self.height
                    {
                        None
                    } else {
                        Some(Pipe::None)
                    }
                } else {
                    if coord_half.x_half {
                        let left = self.has_exit(
                            half_to_coord(&coord_half.neighbour(Direction::Left)).unwrap(),
                            Direction::Left.opposite(),
                        );
                        let right = self.has_exit(
                            half_to_coord(&coord_half.neighbour(Direction::Right)).unwrap(),
                            Direction::Right.opposite(),
                        );

                        if left && right {
                            Some(Pipe::NS)
                        } else {
                            Some(Pipe::None)
                        }
                    } else {
                        let up = self.has_exit(
                            half_to_coord(&coord_half.neighbour(Direction::Up)).unwrap(),
                            Direction::Up.opposite(),
                        );
                        let down = self.has_exit(
                            half_to_coord(&coord_half.neighbour(Direction::Down)).unwrap(),
                            Direction::Down.opposite(),
                        );

                        if up && down {
                            Some(Pipe::NS)
                        } else {
                            Some(Pipe::None)
                        }
                    }
                }
            }
            Some(coord) => self.content.get(&coord).copied(),
        }
    }
}

fn p1(input: &str) -> String {
    Map::parse_input(input)
        .clean_up_pipes_not_in_loop()
        .1
        .to_string()
}

fn p2(input: &str) -> String {
    let clean_map = Map::parse_input(input).clean_up_pipes_not_in_loop().0;

    let outside_points = (0..clean_map.width)
        .flat_map(|x| [(x, 0), (x, clean_map.height - 1)])
        .chain((0..clean_map.height).flat_map(|y| [(0, y), (clean_map.width - 1, y)]))
        .filter(|coord| clean_map.content.get(coord).unwrap() == &Pipe::None)
        .map(|coord| coord_to_half(&coord));

    let mut visited: HashSet<CoordHalf> = HashSet::new();
    let mut current: HashSet<CoordHalf> = HashSet::from_iter(outside_points);

    while !current.is_empty() {
        visited.extend(&current);
        current = current
            .into_iter()
            .flat_map(|pos| {
                [
                    Direction::Up,
                    Direction::Down,
                    Direction::Left,
                    Direction::Right,
                ]
                .into_iter()
                .flat_map(|dir| {
                    let neighbour = pos.neighbour(dir);
                    if !visited.contains(&neighbour)
                        && matches!(
                            clean_map.get_pipe_at_coord_half(&neighbour),
                            Some(Pipe::None)
                        )
                    {
                        Some(neighbour)
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
            })
            .collect()
    }

    let visited: HashSet<Coord> = visited
        .into_iter()
        .flat_map(|half| half_to_coord(&half))
        .collect();

    ((0..clean_map.width)
        .flat_map(|x| (0..clean_map.height).map(move |y| (x, y)))
        .filter(|coord| {
            !visited.contains(coord) && matches!(clean_map.content.get(&coord), Some(&Pipe::None))
        })
        .count() as i32)
        .to_string()
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

    const SAMPLE_INPUT_P2: [&str; 5] = [
        r"
S--7
|..|
|..|
L--J
",
        r"
...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........
",
        r"
..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
..........
",
        r"
.F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...
",
        r"
FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L
",
    ];

    #[test]
    fn test_p2_sample() {
        assert_eq!(p2(SAMPLE_INPUT_P2[0]), "4");
        assert_eq!(p2(SAMPLE_INPUT_P2[1]), "4");
        assert_eq!(p2(SAMPLE_INPUT_P2[2]), "4");
        assert_eq!(p2(SAMPLE_INPUT_P2[3]), "8");
        assert_eq!(p2(SAMPLE_INPUT_P2[4]), "10");
    }

    #[test]
    fn test_p2_actual() {
        assert_eq!(p2(ACTUAL_INPUT), "355");
    }
}
