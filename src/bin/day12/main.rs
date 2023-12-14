use std::collections::HashMap;

const ACTUAL_INPUT: &str = include_str!("../../../actual_inputs/2023/12/input.txt");

fn parse(input: &str) -> (String, Vec<usize>) {
    let (springs, count) = input.trim().split_once(' ').unwrap();
    let springs = springs.trim().to_string();
    let count = count
        .trim()
        .split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    (springs, count)
}

// taken from reddit: https://www.reddit.com/r/adventofcode/comments/18hg99r/2023_day_12_simple_tutorial_with_memoization/
/*
solve(springs, groups, cache, i):
    if num groups is 0:
        if any '#' remaining in springs return 0
        else return 1

    advance i to the next available '?' or '#'

    if i > length of springs return 0

    if (i, num groups) is in cache, return it

    if we can fill the springs at i with the first group in groups:
        recursively call with the groups after that at index: i + groupsize + 1

    if the current spot is '?':
        recursively call with current groups at the next index

    add the result to the cache
    return result
*/
fn solve(input: (String, Vec<usize>)) -> usize {
    fn solve_recur(
        springs: &String,
        groups: &Vec<usize>,
        cache: &mut HashMap<(usize, usize), usize>,
        i: usize,
        i_group: usize,
    ) -> usize {
        if i_group >= groups.len() {
            if springs.chars().skip(i).any(|ch| ch == '#') {
                0
            } else {
                1
            }
        } else {
            let next = springs
                .chars()
                .enumerate()
                .skip(i)
                .find(|(_, ch)| *ch == '?' || *ch == '#');

            if let Some((i, ch)) = next {
                let cache_key = (i, groups.len() - i_group);

                if cache.contains_key(&cache_key) {
                    *cache.get(&cache_key).unwrap()
                } else {
                    let mut result = 0;

                    let first_group_size = groups.get(i_group).unwrap();

                    if (i + *first_group_size <= springs.len())
                        && springs
                            .chars()
                            .skip(i)
                            .take(*first_group_size)
                            .all(|ch| ch == '?' || ch == '#')
                        && springs
                            .chars()
                            .nth(i + *first_group_size)
                            .map(|ch| ch != '#')
                            .unwrap_or(true)
                    {
                        result += solve_recur(
                            springs,
                            groups,
                            cache,
                            i + *first_group_size + 1,
                            i_group + 1,
                        );
                    }

                    if ch == '?' {
                        result += solve_recur(springs, groups, cache, i + 1, i_group);
                    }

                    cache.insert(cache_key, result);
                    result
                }
            } else {
                0
            }
        }
    }

    solve_recur(&input.0, &input.1, &mut HashMap::new(), 0, 0)
}

fn unfold(input: (String, Vec<usize>)) -> (String, Vec<usize>) {
    (
        [input.0]
            .into_iter()
            .cycle()
            .take(5)
            .collect::<Vec<_>>()
            .join("?"),
        [input.1].into_iter().cycle().take(5).flatten().collect(),
    )
}

fn p1(input: &str) -> String {
    input
        .trim()
        .lines()
        .map(parse)
        .map(solve)
        .sum::<usize>()
        .to_string()
}

fn p2(input: &str) -> String {
    input
        .trim()
        .lines()
        .map(parse)
        .map(unfold)
        .map(solve)
        .sum::<usize>()
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
???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
";

    #[test]
    fn test_reddit_example() {
        assert_eq!(solve(("??#???#?????.?".to_string(), vec![5, 1, 1])), 12);
        assert_eq!(solve(("????????#???".to_string(), vec![2, 3])), 15);
    }

    #[test]
    fn test_p1_sample() {
        assert_eq!(p1(SAMPLE_INPUT), "21");
    }

    #[test]
    fn test_p1_actual() {
        assert_eq!(p1(ACTUAL_INPUT), "8180");
    }

    #[test]
    fn test_p2_sample() {
        assert_eq!(p2(SAMPLE_INPUT), "525152");
    }

    #[test]
    fn test_p2_actual() {
        assert_eq!(p2(ACTUAL_INPUT), "620189727003627");
    }
}
