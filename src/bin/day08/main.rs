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

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    a
}

fn lcm(a: u64, b: u64) -> u64 {
    (a * b) / gcd(a, b)
}

fn p2(input: &str) -> String {
    let map = Map::parse_input(input);

    let all_cycles = map
        .nodes
        .keys()
        .filter(|k| k.ends_with('A'))
        .cloned()
        .map(|node| {
            let mut node = node.clone();
            let mut count = 0;
            let mut step = 0;

            while !node.ends_with('Z') {
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

            count
        })
        .collect::<Vec<_>>();

    all_cycles.iter().fold(1, |acc, x| lcm(acc, *x)).to_string()
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
    const SAMPLE_INPUT_3: &str = r"
LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
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
        assert_eq!(p2(SAMPLE_INPUT_3), "6");
    }

    #[test]
    fn test_p2_actual() {
        assert_eq!(p2(ACTUAL_INPUT), "14616363770447");
    }
}
