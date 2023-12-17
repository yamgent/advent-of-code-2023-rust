use std::collections::HashSet;

const ACTUAL_INPUT: &str = include_str!("../../../actual_inputs/2023/16/input.txt");

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Ray {
    pos: (i32, i32),
    direction: Direction,
}

impl Ray {
    fn up(&self) -> Self {
        Self {
            pos: (self.pos.0, self.pos.1 - 1),
            direction: Direction::Up,
        }
    }

    fn down(&self) -> Self {
        Self {
            pos: (self.pos.0, self.pos.1 + 1),
            direction: Direction::Down,
        }
    }

    fn left(&self) -> Self {
        Self {
            pos: (self.pos.0 - 1, self.pos.1),
            direction: Direction::Left,
        }
    }

    fn right(&self) -> Self {
        Self {
            pos: (self.pos.0 + 1, self.pos.1),
            direction: Direction::Right,
        }
    }

    fn advance(&self) -> Self {
        match self.direction {
            Direction::Up => self.up(),
            Direction::Down => self.down(),
            Direction::Left => self.left(),
            Direction::Right => self.right(),
        }
    }

    fn move_ray(&self, map: &[Vec<char>]) -> Vec<Ray> {
        fn in_bounds(map: &[Vec<char>], pos: (i32, i32)) -> bool {
            pos.0 >= 0 && pos.0 < map[0].len() as i32 && pos.1 >= 0 && pos.1 < map.len() as i32
        }

        let pos_ch = map[self.pos.1 as usize][self.pos.0 as usize];

        let next_rays = match pos_ch {
            '.' => vec![self.advance()],
            '-' if self.direction == Direction::Left || self.direction == Direction::Right => {
                vec![self.advance()]
            }
            '-' => vec![self.left(), self.right()],
            '|' if self.direction == Direction::Up || self.direction == Direction::Down => {
                vec![self.advance()]
            }
            '|' => vec![self.up(), self.down()],
            '/' => match self.direction {
                Direction::Up => vec![self.right()],
                Direction::Down => vec![self.left()],
                Direction::Left => vec![self.down()],
                Direction::Right => vec![self.up()],
            },
            '\\' => match self.direction {
                Direction::Up => vec![self.left()],
                Direction::Down => vec![self.right()],
                Direction::Left => vec![self.up()],
                Direction::Right => vec![self.down()],
            },
            _ => unreachable!(),
        };

        next_rays
            .into_iter()
            .filter(|ray| in_bounds(map, ray.pos))
            .collect()
    }
}

fn energized_to_string(energized: &HashSet<(i32, i32)>, map: &[Vec<char>]) -> String {
    let mut result = String::new();

    (0..map.len()).for_each(|y| {
        (0..map[y].len()).for_each(|x| {
            result.push(if energized.contains(&(x as i32, y as i32)) {
                '#'
            } else {
                '.'
            });
        });
        result.push('\n');
    });

    result
}

fn get_energized_count(map: &[Vec<char>], start_ray: Ray) -> usize {
    let mut rays = vec![start_ray];
    let mut visited: HashSet<Ray> = HashSet::from_iter(rays.iter().copied());

    while !rays.is_empty() {
        rays = rays
            .iter()
            .flat_map(|ray| ray.move_ray(&map))
            .filter(|ray| !visited.contains(ray))
            .collect();
        rays.iter().for_each(|ray| {
            visited.insert(ray.clone());
        });
    }

    HashSet::<(i32, i32)>::from_iter(visited.iter().map(|ray| ray.pos)).len()
}

fn parse_map(input: &str) -> Vec<Vec<char>> {
    input
        .trim()
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect()
}

fn p1(input: &str) -> String {
    get_energized_count(
        &parse_map(input),
        Ray {
            pos: (0, 0),
            direction: Direction::Right,
        },
    )
    .to_string()
}

fn p2(input: &str) -> String {
    let map = parse_map(input);

    (0..map[0].len())
        .flat_map(|x| {
            vec![
                Ray {
                    pos: (x as i32, 0),
                    direction: Direction::Down,
                },
                Ray {
                    pos: (x as i32, map.len() as i32 - 1),
                    direction: Direction::Up,
                },
            ]
        })
        .chain((0..map.len()).flat_map(|y| {
            vec![
                Ray {
                    pos: (0, y as i32),
                    direction: Direction::Right,
                },
                Ray {
                    pos: (map[0].len() as i32 - 1, y as i32),
                    direction: Direction::Left,
                },
            ]
        }))
        .map(|ray| get_energized_count(&map, ray))
        .max()
        .unwrap()
        .to_string()
}

fn main() {
    println!("{}", p1(ACTUAL_INPUT));
    println!("{}", p2(ACTUAL_INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = r"
.|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....
";

    #[test]
    fn test_p1_sample() {
        assert_eq!(p1(SAMPLE_INPUT), "46");
    }

    #[test]
    fn test_p1_actual() {
        assert_eq!(p1(ACTUAL_INPUT), "7608");
    }

    #[test]
    fn test_p2_sample() {
        assert_eq!(p2(SAMPLE_INPUT), "51");
    }

    #[test]
    fn test_p2_actual() {
        assert_eq!(p2(ACTUAL_INPUT), "8221");
    }
}
