use std::collections::HashMap;

const ACTUAL_INPUT: &str = include_str!("../../../actual_inputs/2023/08/input.txt");

#[derive(Clone, Copy)]
enum Lookup {
    Left,
    Right,
}

impl Lookup {
    fn parse_all(input: &str) -> Vec<Self> {
        input
            .trim()
            .chars()
            .map(|c| match c {
                'L' => Lookup::Left,
                'R' => Lookup::Right,
                _ => unreachable!(),
            })
            .collect()
    }
}

struct Map {
    instructions: Vec<Lookup>,
    nodes: HashMap<String, (String, String)>,
}

impl Map {
    fn parse_input(input: &str) -> Self {
        let (instructions, nodes) = input.trim().split_once("\n\n").unwrap();
        let instructions = Lookup::parse_all(instructions);
        let nodes = nodes
            .lines()
            .map(|line| {
                let (src, dest) = line.split_once('=').unwrap();
                let src = src.trim().to_string();
                let dest = dest.to_string().replace(['(', ')'], "");

                let (left, right) = dest.split_once(',').unwrap();
                let left = left.trim().to_string();
                let right = right.trim().to_string();

                (src, left, right)
            })
            .fold(HashMap::new(), |mut acc, (src, left, right)| {
                acc.insert(src, (left, right));
                acc
            });

        Self {
            instructions,
            nodes,
        }
    }
}

fn p1(input: &str) -> String {
    let map = Map::parse_input(input);

    let mut count = 0;
    let mut step = 0;
    let mut node = "AAA".to_string();

    while node != "ZZZ" {
        count += 1;

        let direction = map.instructions[step];
        step = (step + 1) % map.instructions.len();

        node = {
            let node_content = map.nodes.get(&node).unwrap();
            match direction {
                Lookup::Left => node_content.0.to_string(),
                Lookup::Right => node_content.1.to_string(),
            }
        };
    }

    count.to_string()
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
RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
";
    const SAMPLE_INPUT_2: &str = r"
LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
";

    #[test]
    fn test_p1_sample() {
        assert_eq!(p1(SAMPLE_INPUT), "2");
        assert_eq!(p1(SAMPLE_INPUT_2), "6");
    }

    #[test]
    fn test_p1_actual() {
        assert_eq!(p1(ACTUAL_INPUT), "20221");
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
