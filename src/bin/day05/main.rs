const ACTUAL_INPUT: &str = include_str!("../../../actual_inputs/2023/05/input.txt");

#[derive(Debug, Clone)]
struct Range {
    start: u64,
    len: u64,
}

struct Offset(u64);

impl Range {
    fn offset(&self, num: u64) -> Option<Offset> {
        if num >= self.start && num < self.start + self.len {
            Some(Offset(num - self.start))
        } else {
            None
        }
    }

    fn num_from_offset(&self, offset: Offset) -> Option<u64> {
        if offset.0 < self.len {
            Some(self.start + offset.0)
        } else {
            None
        }
    }

    fn interval_inclusive(&self) -> (u64, u64) {
        (self.start, self.start + self.len - 1)
    }

    fn overlap(&self, other: &Range) -> Option<Range> {
        let self_interval = self.interval_inclusive();
        let other_interval = other.interval_inclusive();

        if self_interval.0 > other_interval.1 || self_interval.1 < other_interval.0 {
            None
        } else {
            let overlap_interval = (
                self_interval.0.max(other_interval.0),
                self_interval.1.min(other_interval.1),
            );
            Some(Range {
                start: overlap_interval.0,
                len: overlap_interval.1 - overlap_interval.0 + 1,
            })
        }
    }
}

#[derive(Debug)]
struct Mapper {
    dst: Range,
    src: Range,
}

#[derive(Debug)]
struct ConvertResult {
    success: Option<Range>,
    failures: Vec<Range>,
}

impl Mapper {
    fn from_line(line: &str) -> Self {
        let mut iter = line.split_whitespace().map(|x| x.parse::<u64>().unwrap());

        let dst_range_start = iter.next().unwrap();
        let src_range_start = iter.next().unwrap();
        let range_len = iter.next().unwrap();

        Self {
            dst: Range {
                start: dst_range_start,
                len: range_len,
            },
            src: Range {
                start: src_range_start,
                len: range_len,
            },
        }
    }

    fn convert(&self, input: &Range) -> ConvertResult {
        match input.overlap(&self.src) {
            None => ConvertResult {
                success: None,
                failures: vec![input.clone()],
            },
            Some(overlap) => {
                let mut failures = vec![];

                if input.start < overlap.start {
                    failures.push(Range {
                        start: input.start,
                        len: overlap.start - input.start,
                    });
                }

                if input.start + input.len > overlap.start + overlap.len {
                    failures.push(Range {
                        start: overlap.start + overlap.len,
                        len: input.start + input.len - (overlap.start + overlap.len),
                    });
                }

                ConvertResult {
                    success: Some(Range {
                        start: self
                            .dst
                            .num_from_offset(self.src.offset(overlap.start).unwrap())
                            .unwrap(),
                        len: overlap.len,
                    }),
                    failures,
                }
            }
        }
    }
}

#[derive(Debug)]
struct Input {
    seeds: Vec<u64>,
    maps: Vec<Vec<Mapper>>,
}

impl Input {
    fn parse_input(input: &str) -> Self {
        let seeds = input
            .trim()
            .split("\n\n")
            .next()
            .unwrap()
            .split(':')
            .nth(1)
            .unwrap()
            .trim()
            .split(' ')
            .map(|x| x.parse::<u64>().unwrap())
            .collect();

        let maps = input
            .trim()
            .split("\n\n")
            .skip(1)
            .map(|description| description.lines().skip(1).map(Mapper::from_line).collect())
            .collect();

        Self { seeds, maps }
    }
}

fn handle_seed_range(seed_range: &Range, maps: &Vec<Vec<Mapper>>) -> u64 {
    maps.iter()
        .fold(vec![seed_range.clone()], |ranges, current_level| {
            ranges
                .iter()
                .flat_map(|range| {
                    struct RangeCollection {
                        successes: Vec<Range>,
                        failures: Vec<Range>,
                    }

                    let final_ans = current_level.iter().fold(
                        RangeCollection {
                            successes: vec![],
                            failures: vec![range.clone()],
                        },
                        |acc, map| {
                            let mut new_acc = RangeCollection {
                                successes: acc.successes,
                                failures: vec![],
                            };

                            acc.failures.iter().for_each(|failure| {
                                let result = map.convert(failure);
                                if let Some(success) = result.success {
                                    new_acc.successes.push(success);
                                }
                                new_acc.failures.extend(result.failures);
                            });

                            new_acc
                        },
                    );

                    final_ans
                        .successes
                        .iter()
                        .cloned()
                        .chain(final_ans.failures.iter().cloned())
                        .collect::<Vec<_>>()
                })
                .collect()
        })
        .into_iter()
        .map(|range| range.start)
        .min()
        .unwrap()
}

fn p1(input: &str) -> String {
    let input = Input::parse_input(input);
    input
        .seeds
        .into_iter()
        .map(|seed| {
            handle_seed_range(
                &Range {
                    start: seed,
                    len: 1,
                },
                &input.maps,
            )
        })
        .min()
        .unwrap()
        .to_string()
}

fn p2(input: &str) -> String {
    let input = Input::parse_input(input);
    input
        .seeds
        .chunks(2)
        .map(|seed_range| {
            handle_seed_range(
                &Range {
                    start: seed_range[0],
                    len: seed_range[1],
                },
                &input.maps,
            )
        })
        .min()
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
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
";

    #[test]
    fn test_p1_sample() {
        assert_eq!(p1(SAMPLE_INPUT), "35");
    }

    #[test]
    fn test_p1_actual() {
        assert_eq!(p1(ACTUAL_INPUT), "214922730");
    }

    #[test]
    fn test_p2_sample() {
        assert_eq!(p2(SAMPLE_INPUT), "46");
    }

    #[test]
    fn test_p2_actual() {
        assert_eq!(p2(ACTUAL_INPUT), "148041808");
    }
}
