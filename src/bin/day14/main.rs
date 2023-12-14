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
    fn test_p2_sample() {
        assert_eq!(p2(SAMPLE_INPUT), "");
    }

    #[test]
    #[ignore = "not yet implemented"]
    fn test_p2_actual() {
        assert_eq!(p2(ACTUAL_INPUT), "");
    }
}
