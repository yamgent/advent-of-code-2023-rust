use std::collections::HashSet;

const ACTUAL_INPUT: &str = include_str!("../../../actual_inputs/2023/11/input.txt");

struct Universe {
    galaxies: Vec<(usize, usize)>,
}

impl Universe {
    fn parse(input: &str) -> Self {
        Self {
            galaxies: input
                .trim()
                .lines()
                .enumerate()
                .flat_map(|(y, line)| {
                    line.trim()
                        .chars()
                        .enumerate()
                        .filter(|(_, ch)| *ch == '#')
                        .map(|(x, _)| (x, y))
                        .collect::<Vec<_>>()
                })
                .collect(),
        }
    }

    fn expand(&mut self, count: usize) {
        let width = self.galaxies.iter().map(|(x, _)| x).max().unwrap() + 1;
        let height = self.galaxies.iter().map(|(_, y)| y).max().unwrap() + 1;

        (0..width).rev().for_each(|current_x| {
            if self.galaxies.iter().all(|(x, _)| *x != current_x) {
                self.galaxies
                    .iter_mut()
                    .filter(|(x, _)| *x > current_x)
                    .for_each(|coord| coord.0 += count);
            }
        });

        (0..height).rev().for_each(|current_y| {
            if self.galaxies.iter().all(|(_, y)| *y != current_y) {
                self.galaxies
                    .iter_mut()
                    .filter(|(_, y)| *y > current_y)
                    .for_each(|coord| coord.1 += count);
            }
        });
    }

    fn get_galaxies_pairs_steps_sum(&self) -> usize {
        self.galaxies
            .iter()
            .map(|galaxy1| {
                self.galaxies
                    .iter()
                    .map(|galaxy2| {
                        (galaxy2.0.abs_diff(galaxy1.0)) + (galaxy2.1.abs_diff(galaxy1.1))
                    })
                    .sum::<usize>()
            })
            .sum::<usize>()
            / 2
    }

    #[allow(dead_code)]
    fn get_grid_display(&self) -> String {
        let width = self.galaxies.iter().map(|(x, _)| x).max().unwrap() + 1;
        let height = self.galaxies.iter().map(|(_, y)| y).max().unwrap() + 1;

        let galaxies_set: HashSet<(usize, usize)> =
            HashSet::from_iter(self.galaxies.iter().copied());

        (0..height).fold(String::new(), |mut acc, y| {
            (0..width).for_each(|x| {
                let coord = (x, y);

                acc.push(if galaxies_set.contains(&coord) {
                    '#'
                } else {
                    '.'
                });
            });

            if y != height - 1 {
                acc.push('\n');
            }

            acc
        })
    }
}

fn p1(input: &str) -> String {
    let mut universe = Universe::parse(input);
    universe.expand(1);
    universe.get_galaxies_pairs_steps_sum().to_string()
}

fn p2(input: &str) -> String {
    let mut universe = Universe::parse(input);
    universe.expand(999_999);
    universe.get_galaxies_pairs_steps_sum().to_string()
}

fn main() {
    println!("{}", p1(ACTUAL_INPUT));
    println!("{}", p2(ACTUAL_INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = r"
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
";

    #[test]
    fn test_parse() {
        assert_eq!(
            Universe::parse(SAMPLE_INPUT).get_grid_display(),
            SAMPLE_INPUT.trim().to_string()
        );
    }

    #[test]
    fn test_expand() {
        const SAMPLE_INPUT_EXPANDED: &str = r"
....#........
.........#...
#............
.............
.............
........#....
.#...........
............#
.............
.............
.........#...
#....#.......
";
        let mut universe = Universe::parse(SAMPLE_INPUT);
        universe.expand(1);
        assert_eq!(
            universe.get_grid_display(),
            SAMPLE_INPUT_EXPANDED.trim().to_string()
        );
    }

    #[test]
    fn test_p1_sample() {
        assert_eq!(p1(SAMPLE_INPUT), "374");
    }

    #[test]
    fn test_p1_actual() {
        assert_eq!(p1(ACTUAL_INPUT), "9312968");
    }

    #[test]
    fn test_p2_smaller_expand() {
        {
            let mut universe = Universe::parse(SAMPLE_INPUT);
            universe.expand(9);
            assert_eq!(universe.get_galaxies_pairs_steps_sum(), 1030);
        }
        {
            let mut universe = Universe::parse(SAMPLE_INPUT);
            universe.expand(99);
            assert_eq!(universe.get_galaxies_pairs_steps_sum(), 8410);
        }
    }

    #[test]
    fn test_p2_sample() {
        assert_eq!(p2(SAMPLE_INPUT), "82000210");
    }

    #[test]
    fn test_p2_actual() {
        assert_eq!(p2(ACTUAL_INPUT), "597714117556");
    }
}
