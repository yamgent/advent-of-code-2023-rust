const ACTUAL_INPUT: &str = include_str!("../../../actual_inputs/2023/03/input.txt");

fn p1(input: &str) -> String {
    let mut map = input
        .trim()
        .lines()
        .map(|c| c.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut final_sum = 0;

    (0..map.len()).for_each(|y| {
        (0..map[y].len()).for_each(|x| {
            if map[y][x].is_ascii_digit() {
                let mut x = x;
                let mut value = 0;
                let mut is_part_number = false;

                while x < map[y].len() && map[y][x].is_ascii_digit() {
                    value *= 10;
                    value += map[y][x] as u32 - '0' as u32;

                    fn is_symbol(map: &Vec<Vec<char>>, y: usize, x: usize) -> bool {
                        y < map.len()
                            && x < map[y].len()
                            && map[y][x] != '.'
                            && !map[y][x].is_ascii_digit()
                    }

                    is_part_number |= x != 0 && y != 0 && is_symbol(&map, y - 1, x - 1);
                    is_part_number |= y != 0 && is_symbol(&map, y - 1, x);
                    is_part_number |= y != 0 && is_symbol(&map, y - 1, x + 1);
                    is_part_number |= x != 0 && is_symbol(&map, y, x - 1);
                    is_part_number |= is_symbol(&map, y, x + 1);
                    is_part_number |= x != 0 && is_symbol(&map, y + 1, x - 1);
                    is_part_number |= is_symbol(&map, y + 1, x);
                    is_part_number |= is_symbol(&map, y + 1, x + 1);

                    map[y][x] = '.';

                    x += 1;
                }

                if is_part_number {
                    final_sum += value;
                }
            }
        });
    });

    final_sum.to_string()
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
        assert_eq!(p2(SAMPLE_INPUT), "");
    }

    #[test]
    #[ignore = "not yet implemented"]
    fn test_p2_actual() {
        assert_eq!(p2(ACTUAL_INPUT), "");
    }
}
