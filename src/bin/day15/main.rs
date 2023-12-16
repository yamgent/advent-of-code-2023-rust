use std::collections::HashMap;

const ACTUAL_INPUT: &str = include_str!("../../../actual_inputs/2023/15/input.txt");

fn hash(text: &str) -> u8 {
    text.chars()
        .fold(0u8, |acc, ch| acc.wrapping_add(ch as u8).wrapping_mul(17))
}

fn p1(input: &str) -> String {
    input
        .trim()
        .split(',')
        .map(hash)
        .map(|val| val as u64)
        .sum::<u64>()
        .to_string()
}

#[derive(Debug)]
enum Step {
    Remove(String),
    Add(String, i32),
}

impl Step {
    fn parse(step: &str) -> Self {
        if step.ends_with('-') {
            Self::Remove(step.chars().take(step.len() - 1).collect())
        } else {
            let (label, focal_length) = step.split_once('=').unwrap();
            Self::Add(label.to_string(), focal_length.parse().unwrap())
        }
    }
}

#[derive(Debug)]
struct BoxContentNode {
    focal_length: i32,
    prev: String,
    next: String,
}

#[derive(Debug)]
struct BoxContent {
    nodes: HashMap<String, BoxContentNode>,
}

impl BoxContent {
    const HEAD_NODE: &'static str = "HEAD";
    const TAIL_NODE: &'static str = "TAIL";

    fn new() -> Self {
        Self {
            nodes: HashMap::from_iter([
                (
                    Self::HEAD_NODE.to_string(),
                    BoxContentNode {
                        focal_length: 0,
                        prev: Self::HEAD_NODE.to_string(),
                        next: Self::TAIL_NODE.to_string(),
                    },
                ),
                (
                    Self::TAIL_NODE.to_string(),
                    BoxContentNode {
                        focal_length: 0,
                        prev: Self::HEAD_NODE.to_string(),
                        next: Self::TAIL_NODE.to_string(),
                    },
                ),
            ]),
        }
    }

    fn remove(&mut self, label: String) {
        if let Some(node) = self.nodes.get(&label) {
            let prev_label = node.prev.clone();
            let next_label = node.next.clone();

            self.nodes.get_mut(&prev_label).unwrap().next = next_label.clone();
            self.nodes.get_mut(&next_label).unwrap().prev = prev_label;
            self.nodes.remove(&label);
        }
    }

    fn set(&mut self, label: String, focal_length: i32) {
        if !self.nodes.contains_key(&label) {
            let tail_node = self.nodes.get_mut(Self::TAIL_NODE).unwrap();
            let last_node_label = tail_node.prev.clone();
            tail_node.prev = label.clone();

            let last_node = self.nodes.get_mut(&last_node_label).unwrap();
            last_node.next = label.clone();

            self.nodes.insert(
                label.clone(),
                BoxContentNode {
                    focal_length: 0,
                    prev: last_node_label,
                    next: Self::TAIL_NODE.to_string(),
                },
            );
        }

        self.nodes.get_mut(&label).unwrap().focal_length = focal_length;
    }

    fn calculate_focusing_power(&self) -> i32 {
        let mut result = 0;
        let mut slot_number = 1;
        let mut next = self.nodes.get(Self::HEAD_NODE).unwrap().next.to_string();

        while next != Self::TAIL_NODE {
            let node = self.nodes.get(&next).unwrap();

            result += slot_number * node.focal_length;
            slot_number += 1;
            next = node.next.to_string();
        }

        result
    }
}

#[derive(Debug)]
struct BoxContentCollection {
    boxes: HashMap<u8, BoxContent>,
}

impl BoxContentCollection {
    fn new() -> Self {
        Self {
            boxes: HashMap::new(),
        }
    }

    fn remove(&mut self, label: &str) {
        let box_number = hash(label);
        self.boxes
            .entry(box_number)
            .or_insert_with(|| BoxContent::new())
            .remove(label.to_string());
    }

    fn set(&mut self, label: &str, focal_length: i32) {
        let box_number = hash(&label);
        self.boxes
            .entry(box_number)
            .or_insert_with(|| BoxContent::new())
            .set(label.to_string(), focal_length);
    }

    fn calculate_focusing_power(&self) -> i32 {
        self.boxes
            .iter()
            .map(|(box_number, box_content)| {
                let box_number = (*box_number as i32) + 1;
                box_number * box_content.calculate_focusing_power()
            })
            .sum()
    }
}

fn p2(input: &str) -> String {
    input
        .trim()
        .split(',')
        .map(Step::parse)
        .fold(BoxContentCollection::new(), |mut acc, step| {
            match step {
                Step::Remove(label) => {
                    acc.remove(&label);
                }
                Step::Add(label, focal_length) => {
                    acc.set(&label, focal_length);
                }
            }
            acc
        })
        .calculate_focusing_power()
        .to_string()
}

fn main() {
    println!("{}", p1(ACTUAL_INPUT));
    println!("{}", p2(ACTUAL_INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn test_p1_sample() {
        assert_eq!(p1("HASH"), "52");
        assert_eq!(p1(SAMPLE_INPUT), "1320");
    }

    #[test]
    fn test_p1_actual() {
        assert_eq!(p1(ACTUAL_INPUT), "507291");
    }

    #[test]
    fn test_p2_sample() {
        assert_eq!(p2(SAMPLE_INPUT), "145");
    }

    #[test]
    fn test_p2_actual() {
        assert_eq!(p2(ACTUAL_INPUT), "296921");
    }
}
