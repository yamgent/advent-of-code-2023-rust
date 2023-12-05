const ACTUAL_INPUT: &str = include_str!("../../../actual_inputs/2023/05/input.txt");

#[derive(Debug)]
struct Mapper {
    dst_range_start: u64,
    src_range_start: u64,
    range_len: u64,
}

impl Mapper {
    fn from_line(line: &str) -> Self {
        let mut iter = line.split_whitespace().map(|x| x.parse::<u64>().unwrap());

        let dst_range_start = iter.next().unwrap();
        let src_range_start = iter.next().unwrap();
        let range_len = iter.next().unwrap();

        Self {
            dst_range_start,
            src_range_start,
            range_len,
        }
    }

    fn convert(&self, src_num: u64) -> Option<u64> {
        if src_num >= self.src_range_start && src_num < self.src_range_start + self.range_len {
            Some(src_num - self.src_range_start + self.dst_range_start)
        } else {
            None
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

fn p1(input: &str) -> String {
    let input = Input::parse_input(input);
    input
        .seeds
        .into_iter()
        .map(|seed| {
            input.maps.iter().fold(seed, |src, current| {
                current
                    .iter()
                    .find_map(|mapper| mapper.convert(src))
                    .unwrap_or(src)
            })
        })
        .min()
        .unwrap()
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
        assert_eq!(p2(SAMPLE_INPUT), "");
    }

    #[test]
    #[ignore = "not yet implemented"]
    fn test_p2_actual() {
        assert_eq!(p2(ACTUAL_INPUT), "");
    }
}
