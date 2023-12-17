use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

const ACTUAL_INPUT: &str = include_str!("../../../actual_inputs/2023/17/input.txt");

type Pos = (i32, i32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Direction {
    Up = 0,
    Down,
    Left,
    Right,
}

impl Direction {
    fn opposite(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

fn advance(pos: &Pos, direction: Direction) -> Pos {
    match direction {
        Direction::Up => (pos.0, pos.1 - 1),
        Direction::Down => (pos.0, pos.1 + 1),
        Direction::Left => (pos.0 - 1, pos.1),
        Direction::Right => (pos.0 + 1, pos.1),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct VirtualNode {
    pos: Pos,
    moves_left: usize,
    moves_left_direction: Direction,
}

impl VirtualNode {
    fn advance(&self, direction: Direction, bounds: &(usize, usize)) -> Option<Self> {
        if self.moves_left == 0 && self.moves_left_direction == direction {
            None
        } else if self.moves_left_direction.opposite() == direction {
            None
        } else {
            let pos = advance(&self.pos, direction);

            if pos.0 >= 0 && pos.0 < bounds.0 as i32 && pos.1 >= 0 && pos.1 < bounds.1 as i32 {
                Some(VirtualNode {
                    pos,
                    moves_left: if self.moves_left_direction == direction {
                        self.moves_left - 1
                    } else {
                        2
                    },
                    moves_left_direction: direction,
                })
            } else {
                None
            }
        }
    }
}

#[derive(Debug)]
struct Map {
    map: Vec<Vec<u32>>,
}

impl Map {
    fn parse_input(input: &str) -> Self {
        Self {
            map: input
                .trim()
                .lines()
                .map(|line| line.chars().map(|x| x.to_digit(10).unwrap()).collect())
                .collect(),
        }
    }

    fn get_heat_loss(&self, pos: Pos) -> u32 {
        self.map[pos.1 as usize][pos.0 as usize]
    }

    fn get_neighbours(&self, node: &VirtualNode) -> Vec<VirtualNode> {
        let bounds = (self.map[0].len(), self.map.len());
        [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ]
        .into_iter()
        .flat_map(|dir| node.advance(dir, &bounds))
        .collect()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct DijkstraQueue {
    weight: Reverse<u32>,
    node: VirtualNode,
}

fn p1(input: &str) -> String {
    let map = Map::parse_input(input);

    let mut queue = BinaryHeap::from_iter([
        DijkstraQueue {
            weight: Reverse(map.get_heat_loss((0, 1))),
            node: VirtualNode {
                pos: (0, 1),
                moves_left: 2,
                moves_left_direction: Direction::Down,
            },
        },
        DijkstraQueue {
            weight: Reverse(map.get_heat_loss((1, 0))),
            node: VirtualNode {
                pos: (1, 0),
                moves_left: 2,
                moves_left_direction: Direction::Right,
            },
        },
    ]);

    let mut visited: HashSet<VirtualNode> = HashSet::new();

    while let Some(next) = queue.pop() {
        let pos = next.node.pos;
        if pos.0 == map.map[0].len() as i32 - 1 && pos.1 == map.map.len() as i32 - 1 {
            return next.weight.0.to_string();
        }

        map.get_neighbours(&next.node)
            .into_iter()
            .filter(|node| !visited.contains(&node))
            .map(|node| DijkstraQueue {
                weight: Reverse(next.weight.0 + map.get_heat_loss(node.pos)),
                node,
            })
            .collect::<Vec<_>>()
            .into_iter()
            .for_each(|queue_node| {
                visited.insert(queue_node.node);
                queue.push(queue_node);
            });
    }

    unreachable!()
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
2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533
";

    #[test]
    fn test_p1_sample() {
        assert_eq!(p1(SAMPLE_INPUT), "102");
    }

    #[test]
    fn test_p1_actual() {
        assert_eq!(p1(ACTUAL_INPUT), "1263");
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
