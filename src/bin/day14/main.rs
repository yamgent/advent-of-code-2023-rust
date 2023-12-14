const ACTUAL_INPUT: &str = include_str!("../../../actual_inputs/2023/14/input.txt");

type World = Vec<Vec<Space>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Space {
    RoundRock,
    CubeRock,
    Empty,
}

impl Space {
    fn parse(ch: char) -> Self {
        match ch {
            'O' => Space::RoundRock,
            '#' => Space::CubeRock,
            '.' => Space::Empty,
            _ => unreachable!(),
        }
    }
}

fn parse_input(input: &str) -> World {
    input
        .trim()
        .lines()
        .map(|line| line.trim().chars().map(Space::parse).collect())
        .collect()
}

fn roll_rocks_north(world: &mut World) {
    let mut highest = std::iter::repeat(-1)
        .take(world[0].len())
        .collect::<Vec<_>>();

    (0..world.len()).for_each(|y| {
        (0..world[0].len()).for_each(|x| match world[y][x] {
            Space::RoundRock => {
                highest[x] += 1;
                let final_y = highest[x] as usize;
                world[y][x] = Space::Empty;
                world[final_y][x] = Space::RoundRock;
            }
            Space::CubeRock => {
                highest[x] = y as i32;
            }
            Space::Empty => {}
        });
    });
}

fn roll_rocks_west(world: &mut World) {
    let mut leftest = std::iter::repeat(-1).take(world.len()).collect::<Vec<_>>();

    (0..world[0].len()).for_each(|x| {
        (0..world.len()).for_each(|y| match world[y][x] {
            Space::RoundRock => {
                leftest[y] += 1;
                let final_x = leftest[y] as usize;
                world[y][x] = Space::Empty;
                world[y][final_x] = Space::RoundRock;
            }
            Space::CubeRock => {
                leftest[y] = x as i32;
            }
            Space::Empty => {}
        });
    });
}

fn roll_rocks_south(world: &mut World) {
    let mut lowest = std::iter::repeat(world.len() as i32)
        .take(world[0].len())
        .collect::<Vec<_>>();

    (0..world.len()).rev().for_each(|y| {
        (0..world[0].len()).for_each(|x| match world[y][x] {
            Space::RoundRock => {
                lowest[x] -= 1;
                let final_y = lowest[x] as usize;
                world[y][x] = Space::Empty;
                world[final_y][x] = Space::RoundRock;
            }
            Space::CubeRock => {
                lowest[x] = y as i32;
            }
            Space::Empty => {}
        });
    });
}

fn roll_rocks_east(world: &mut World) {
    let mut rightest = std::iter::repeat(world[0].len() as i32)
        .take(world.len())
        .collect::<Vec<_>>();

    (0..world[0].len()).rev().for_each(|x| {
        (0..world.len()).for_each(|y| match world[y][x] {
            Space::RoundRock => {
                rightest[y] -= 1;
                let final_x = rightest[y] as usize;
                world[y][x] = Space::Empty;
                world[y][final_x] = Space::RoundRock;
            }
            Space::CubeRock => {
                rightest[y] = x as i32;
            }
            Space::Empty => {}
        });
    });
}

fn roll_one_cycle(world: &mut World) {
    roll_rocks_north(world);
    roll_rocks_west(world);
    roll_rocks_south(world);
    roll_rocks_east(world);
}

fn world_to_string(world: &World) -> String {
    world.iter().fold(String::new(), |mut acc, row| {
        row.iter().for_each(|space| {
            acc.push(match space {
                Space::RoundRock => 'O',
                Space::CubeRock => '#',
                Space::Empty => '.',
            });
        });
        acc.push('\n');
        acc
    })
}

fn calculate_load(world: &World) -> usize {
    world
        .iter()
        .rev()
        .enumerate()
        .map(|(row_index, row)| {
            (row_index + 1) * row.iter().filter(|x| **x == Space::RoundRock).count()
        })
        .sum()
}

fn p1(input: &str) -> String {
    let mut world = parse_input(input);
    roll_rocks_north(&mut world);
    calculate_load(&world).to_string()
}

fn p2(input: &str) -> String {
    let mut world = parse_input(input);

    use std::collections::HashMap;
    let mut seen: HashMap<String, usize> = HashMap::new();

    const TOTAL_CYCLES: usize = 1_000_000_000;

    let (cycle_stop, cycle_start) = (1..TOTAL_CYCLES)
        .find_map(|current_cycle| {
            roll_one_cycle(&mut world);

            let current_world = world_to_string(&world);

            let previous_cycle = *seen.entry(current_world).or_insert(current_cycle);
            if previous_cycle != current_cycle {
                Some((current_cycle, previous_cycle))
            } else {
                None
            }
        })
        .unwrap();

    let rem = (TOTAL_CYCLES - cycle_start) % (cycle_stop - cycle_start);

    (0..rem).for_each(|_| {
        roll_one_cycle(&mut world);
    });

    calculate_load(&world).to_string()
}

fn main() {
    println!("{}", p1(ACTUAL_INPUT));
    println!("{}", p2(ACTUAL_INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = r"
O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....
";

    #[test]
    fn test_p1_sample() {
        assert_eq!(p1(SAMPLE_INPUT), "136");
    }

    #[test]
    fn test_p1_actual() {
        assert_eq!(p1(ACTUAL_INPUT), "109939");
    }

    #[test]
    fn test_roll_cycles() {
        let mut world = parse_input(SAMPLE_INPUT);

        roll_one_cycle(&mut world);
        assert_eq!(
            world_to_string(&world).trim(),
            r"
.....#....
....#...O#
...OO##...
.OO#......
.....OOO#.
.O#...O#.#
....O#....
......OOOO
#...O###..
#..OO#....
"
            .trim()
        );

        roll_one_cycle(&mut world);
        assert_eq!(
            world_to_string(&world).trim(),
            r"
.....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#..OO###..
#.OOO#...O
"
            .trim()
        );

        roll_one_cycle(&mut world);
        assert_eq!(
            world_to_string(&world).trim(),
            r"
.....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#...O###.O
#.OOO#...O
"
            .trim()
        );
    }

    #[test]
    fn test_p2_sample() {
        assert_eq!(p2(SAMPLE_INPUT), "64");
    }

    #[test]
    fn test_p2_actual() {
        assert_eq!(p2(ACTUAL_INPUT), "101010");
    }
}
