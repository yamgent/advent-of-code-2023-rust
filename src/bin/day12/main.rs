use std::collections::HashMap;

const ACTUAL_INPUT: &str = include_str!("../../../actual_inputs/2023/12/input.txt");

fn parse(input: &str) -> (String, Vec<u64>) {
    let (springs, count) = input.trim().split_once(' ').unwrap();
    let springs = springs.trim().to_string();
    let count = count
        .trim()
        .split(',')
        .map(|x| x.parse::<u64>().unwrap())
        .rev()
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
fn solve(input: (String, Vec<u64>)) -> u64 {
    fn solve_recur(
        springs: &String,
        groups: Vec<u64>,
        cache: &mut HashMap<(usize, usize), u64>,
        i: usize,
    ) -> u64 {
        if groups.is_empty() {
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
                let cache_key = (i, groups.len());

                if cache.contains_key(&cache_key) {
                    *cache.get(&cache_key).unwrap()
                } else {
                    let mut result = 0;

                    let first_group_size = groups.iter().last().unwrap();

                    if (i + (*first_group_size as usize) <= springs.len())
                        && springs
                            .chars()
                            .skip(i)
                            .take(*first_group_size as usize)
                            .all(|ch| ch == '?' || ch == '#')
                        && springs
                            .chars()
                            .nth(i + *first_group_size as usize)
                            .map(|ch| ch != '#')
                            .unwrap_or(true)
                    {
                        result += solve_recur(
                            springs,
                            groups.iter().take(groups.len() - 1).copied().collect(),
                            cache,
                            i + *first_group_size as usize + 1,
                        );
                    }

                    if ch == '?' {
                        result += solve_recur(springs, groups.clone(), cache, i + 1);
                    }

                    cache.insert(cache_key, result);
                    result
                }
            } else {
                0
            }
        }
    }

    solve_recur(&input.0, input.1, &mut HashMap::new(), 0)
}

fn unfold(input: (String, Vec<u64>)) -> (String, Vec<u64>) {
    (
        [input.0.clone()]
            .into_iter()
            .cycle()
            .take(4)
            .map(|mut x| {
                x.push('?');
                x
            })
            .chain([input.0])
            .collect(),
        [input.1].into_iter().cycle().take(5).flatten().collect(),
    )
}

fn p1(input: &str) -> String {
    input
        .trim()
        .lines()
        .map(parse)
        .map(solve)
        .sum::<u64>()
        .to_string()
}

fn p2(input: &str) -> String {
    input
        .trim()
        .lines()
        .map(parse)
        .map(unfold)
        .map(solve)
        .sum::<u64>()
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
        assert_eq!(solve(("??#???#?????.?".to_string(), vec![1, 1, 5])), 12);
        assert_eq!(solve(("????????#???".to_string(), vec![3, 2])), 15);
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
