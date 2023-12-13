use std::collections::HashSet;

const ACTUAL_INPUT: &str = include_str!("../../../actual_inputs/2023/13/input.txt");

fn reflect_x(rock: &(i64, i64), x_line: i64) -> (i64, i64) {
    (2 * x_line - rock.0 + 1, rock.1)
}

fn reflect_y(rock: &(i64, i64), y_line: i64) -> (i64, i64) {
    (rock.0, 2 * y_line - rock.1 + 1)
}

fn solve_p1(pattern: &str) -> u64 {
    let height = pattern.lines().count() as i64;
    let width = pattern.lines().next().unwrap().chars().count() as i64;

    let rocks: HashSet<(i64, i64)> =
        HashSet::from_iter(pattern.lines().enumerate().flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, ch)| *ch == '#')
                .map(|(x, _)| (x as i64, y as i64))
                .collect::<Vec<(i64, i64)>>()
        }));

    (0..(width - 1))
        .into_iter()
        .find(|x| {
            rocks.iter().all(|rock| {
                let reflect = reflect_x(rock, *x);
                reflect.0 < 0 || reflect.0 >= width || rocks.contains(&reflect)
            })
        })
        .map(|x| x + 1)
        .unwrap_or_else(|| {
            (0..(height - 1))
                .into_iter()
                .find(|y| {
                    rocks.iter().all(|rock| {
                        let reflect = reflect_y(rock, *y);
                        reflect.1 < 0 || reflect.1 >= height || rocks.contains(&reflect)
                    })
                })
                .map(|y| (y + 1) * 100)
                .unwrap()
        }) as u64
}

fn p1(input: &str) -> String {
    input
        .trim()
        .split("\n\n")
        .map(|line| solve_p1(line.trim()))
        .sum::<u64>()
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
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
";

    #[test]
    fn test_reflects() {
        assert_eq!(reflect_x(&(3, 0), 3), (4, 0));
        assert_eq!(reflect_x(&(4, 0), 3), (3, 0));
        assert_eq!(reflect_x(&(5, 0), 3), (2, 0));

        assert_eq!(reflect_y(&(0, 3), 3), (0, 4));
        assert_eq!(reflect_y(&(0, 4), 3), (0, 3));
        assert_eq!(reflect_y(&(0, 5), 3), (0, 2));
    }

    #[test]
    fn test_p1_sample() {
        assert_eq!(p1(SAMPLE_INPUT), "405");
    }

    const EDGE_CASE_1: &str = r"
##..###
##.#.##
##.##.#
#######
";
    const EDGE_CASE_2: &str = r"
#####
#####
#.#.#
.####
...##
";
    const EDGE_CASE_3: &str = r"
#..####
#.#.###
#.##.##
#######
";
    const EDGE_CASE_4: &str = r"
#..###
#.#.##
#.##.#
######
######
";

    #[test]
    fn test_p1_edge_case() {
        assert_eq!(p1(EDGE_CASE_1), "1");
        assert_eq!(p1(EDGE_CASE_2), "100");
        assert_eq!(p1(EDGE_CASE_3), "6");
        assert_eq!(p1(EDGE_CASE_4), "400");
    }

    #[test]
    fn test_p1_actual() {
        assert_eq!(p1(ACTUAL_INPUT), "33780");
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
