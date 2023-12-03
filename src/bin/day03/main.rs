use std::collections::{HashMap, HashSet};

const ACTUAL_INPUT: &str = include_str!("../../../actual_inputs/2023/03/input.txt");

fn get_symbol(map: &Vec<Vec<char>>, y: usize, x: usize) -> char {
    if y >= map.len() || x >= map[y].len() || map[y][x].is_ascii_digit() {
        '.'
    } else {
        map[y][x]
    }
}

fn get_neighbours_coord(map: &Vec<Vec<char>>, y: usize, x: usize) -> Vec<(usize, usize)> {
    let mut neighbours = vec![];

    if y != 0 {
        if x != 0 {
            neighbours.push((y - 1, x - 1));
        }
        neighbours.push((y - 1, x));
        if x + 1 < map[y].len() {
            neighbours.push((y - 1, x + 1));
        }
    }

    if x != 0 {
        neighbours.push((y, x - 1));
    }
    if x + 1 < map[y].len() {
        neighbours.push((y, x + 1));
    }

    if y + 1 < map.len() {
        if x != 0 {
            neighbours.push((y + 1, x - 1));
        }
        neighbours.push((y + 1, x));
        if x + 1 < map[y].len() {
            neighbours.push((y + 1, x + 1));
        }
    }

    neighbours
}

fn solve(input: &str) -> (String, String) {
    let mut map = input
        .trim()
        .lines()
        .map(|c| c.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut final_sum = 0;
    let mut star_graph = HashMap::<(usize, usize), Vec<u32>>::new();

    (0..map.len()).for_each(|y| {
        (0..map[y].len()).for_each(|x| {
            if map[y][x].is_ascii_digit() {
                let mut x = x;
                let mut value = 0;
                let mut is_part_number = false;
                let mut stars = HashSet::new();

                while x < map[y].len() && map[y][x].is_ascii_digit() {
                    value *= 10;
                    value += map[y][x] as u32 - '0' as u32;

                    let neighbours = get_neighbours_coord(&map, y, x);
                    is_part_number |= neighbours
                        .iter()
                        .any(|(y, x)| get_symbol(&map, *y, *x) != '.');
                    neighbours
                        .into_iter()
                        .filter(|(y, x)| get_symbol(&map, *y, *x) == '*')
                        .for_each(|(y, x)| {
                            stars.insert((y, x));
                        });

                    map[y][x] = '.';

                    x += 1;
                }

                if is_part_number {
                    final_sum += value;
                }
                stars.into_iter().for_each(|star| {
                    star_graph
                        .entry(star)
                        .and_modify(|e| e.push(value))
                        .or_insert(vec![value]);
                });
            }
        });
    });

    let final_gear_ratios_sum = star_graph
        .values()
        .filter(|v| v.len() > 1)
        .map(|v| v.iter().product::<u32>())
        .sum::<u32>();

    (final_sum.to_string(), final_gear_ratios_sum.to_string())
}

fn p1(input: &str) -> String {
    solve(input).0
}

fn p2(input: &str) -> String {
    solve(input).1
}

fn main() {
    println!("{}", p1(ACTUAL_INPUT));
    println!("{}", p2(ACTUAL_INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = r"
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
";

    #[test]
    fn test_p1_sample() {
        assert_eq!(p1(SAMPLE_INPUT), "4361");
    }

    #[test]
    fn test_p1_actual() {
        assert_eq!(p1(ACTUAL_INPUT), "556057");
    }

    #[test]
    fn test_p2_sample() {
        assert_eq!(p2(SAMPLE_INPUT), "467835");
    }

    #[test]
    fn test_p2_actual() {
        assert_eq!(p2(ACTUAL_INPUT), "82824352");
    }
}
