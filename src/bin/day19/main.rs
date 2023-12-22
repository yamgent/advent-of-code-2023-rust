use std::collections::HashMap;

const ACTUAL_INPUT: &str = include_str!("../../../actual_inputs/2023/19/input.txt");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Interval(u64, u64);

impl Interval {
    fn new(min: u64, max: u64) -> Self {
        if max < min {
            panic!("Forbidden state: {}-{} not a legal interval.", min, max);
        }
        Self(min, max)
    }

    fn inside(&self, val: u64) -> bool {
        val >= self.0 && val <= self.1
    }

    fn sub(&self, other: &Interval) -> Option<Self> {
        if other.1 < self.0 || other.0 > self.1 {
            Some(*self)
        } else if other.0 <= self.0 && other.1 >= self.1 {
            None
        } else if other.0 > self.0 && other.1 < self.1 {
            unimplemented!("Should not happen in this scenario.");
        } else {
            let (min, max) = if other.1 == self.0 {
                (self.0 + 1, self.1)
            } else if other.0 == self.1 {
                (self.0, self.1 - 1)
            } else if other.0 > self.0 && other.0 < self.1 {
                (self.0, other.0 - 1)
            } else {
                // other.1 >= self.0 && other.1 <= self.1
                (other.1 + 1, self.1)
            };

            if min <= max {
                Some(Interval::new(min, max))
            } else {
                None
            }
        }
    }

    fn count(&self) -> u64 {
        self.1 - self.0 + 1
    }
}

#[cfg(test)]
mod tests_interval {
    use super::*;

    #[test]
    fn test_interval_new() {
        assert_eq!(Interval::new(1, 1), Interval(1, 1));
        assert_eq!(Interval::new(1, 2), Interval(1, 2));
    }

    #[test]
    #[should_panic]
    fn test_interval_new_invalid() {
        Interval::new(2, 1);
    }

    #[test]
    fn test_interval_inside() {
        assert_eq!(Interval::new(10, 20).inside(9), false);
        assert_eq!(Interval::new(10, 20).inside(10), true);
        assert_eq!(Interval::new(10, 20).inside(20), true);
        assert_eq!(Interval::new(10, 20).inside(21), false);
    }

    #[test]
    fn test_interval_sub() {
        assert_eq!(
            Interval::new(10, 20).sub(&Interval::new(0, 9)),
            Some(Interval::new(10, 20))
        );
        assert_eq!(
            Interval::new(10, 20).sub(&Interval::new(21, 30)),
            Some(Interval::new(10, 20))
        );

        assert_eq!(
            Interval::new(10, 20).sub(&Interval::new(0, 10)),
            Some(Interval::new(11, 20))
        );
        assert_eq!(
            Interval::new(10, 20).sub(&Interval::new(20, 30)),
            Some(Interval::new(10, 19))
        );

        assert_eq!(
            Interval::new(10, 20).sub(&Interval::new(0, 15)),
            Some(Interval::new(16, 20))
        );
        assert_eq!(
            Interval::new(10, 20).sub(&Interval::new(15, 30)),
            Some(Interval::new(10, 14))
        );

        assert_eq!(Interval::new(10, 20).sub(&Interval::new(0, 20)), None);
        assert_eq!(Interval::new(10, 20).sub(&Interval::new(10, 30)), None);

        assert_eq!(Interval::new(10, 20).sub(&Interval::new(0, 30)), None);
        assert_eq!(Interval::new(10, 20).sub(&Interval::new(10, 20)), None);

        assert_eq!(
            Interval::new(10, 20).sub(&Interval::new(10, 15)),
            Some(Interval::new(16, 20))
        );
        assert_eq!(
            Interval::new(10, 20).sub(&Interval::new(15, 20)),
            Some(Interval::new(10, 14))
        );

        assert_eq!(
            Interval::new(10, 20).sub(&Interval::new(9, 9)),
            Some(Interval::new(10, 20))
        );
        assert_eq!(
            Interval::new(10, 20).sub(&Interval::new(10, 10)),
            Some(Interval::new(11, 20))
        );
        assert_eq!(
            Interval::new(10, 20).sub(&Interval::new(20, 20)),
            Some(Interval::new(10, 19))
        );
        assert_eq!(
            Interval::new(10, 20).sub(&Interval::new(21, 21)),
            Some(Interval::new(10, 20))
        );
    }

    #[test]
    fn test_interval_count() {
        assert_eq!(Interval::new(1, 1).count(), 1);
        assert_eq!(Interval::new(1, 10).count(), 10);
    }
}

const P_X: usize = 0;
const P_M: usize = 1;
const P_A: usize = 2;
const P_S: usize = 3;

fn part_to_idx(part: char) -> usize {
    match part {
        'x' => P_X,
        'm' => P_M,
        'a' => P_A,
        's' => P_S,
        _ => unreachable!(),
    }
}

#[derive(Debug)]
struct Ratings {
    ratings: [u64; 4],
}

impl Ratings {
    fn parse(line: &str) -> Self {
        line.trim().replace(['{', '}'], "").split(',').fold(
            Self {
                ratings: [0, 0, 0, 0],
            },
            |mut acc, rating| {
                let (part, value) = rating.split_once('=').unwrap();
                let part = part_to_idx(part.chars().next().unwrap());
                let value = value.parse().unwrap();
                acc.ratings[part] = value;
                acc
            },
        )
    }
}

#[derive(Debug)]
struct Workflow {
    name: String,
    ifs: Vec<WorkflowIf>,
    else_workflow: String,
}

impl Workflow {
    fn parse(line: &str) -> Self {
        let (name, rest) = line.split_once('{').unwrap();
        rest.replace('}', "").split(',').fold(
            Workflow {
                name: name.to_string(),
                ifs: vec![],
                else_workflow: String::new(),
            },
            |mut acc, current| {
                if current.contains(':') {
                    acc.ifs.push(WorkflowIf::parse(current));
                } else {
                    acc.else_workflow = current.to_string();
                }
                acc
            },
        )
    }

    fn execute(&self, rating: &Ratings) -> String {
        self.ifs
            .iter()
            .find(|workflow_if| workflow_if.execute(&rating))
            .map(|workflow_if| workflow_if.true_workflow.to_string())
            .unwrap_or(self.else_workflow.to_string())
    }
}

#[derive(Debug, Clone, Copy)]
struct Constraints {
    constraints: [Option<Interval>; 4],
}

impl Constraints {
    fn combos(&self) -> u64 {
        if self.constraints.iter().any(|v| v.is_none()) {
            0
        } else {
            self.constraints
                .iter()
                .map(|v| v.unwrap().count())
                .product()
        }
    }

    fn apply_constraint(&self, cond: &WorkflowCond) -> Self {
        let mut result = *self;
        result.constraints[cond.part] = result.constraints[cond.part]
            .map(|x| x.sub(&cond.range))
            .flatten();
        result
    }
}

#[derive(Debug, Clone, Copy)]
struct WorkflowCond {
    part: usize,
    range: Interval,
}

impl WorkflowCond {
    fn parse(condition: &str) -> Self {
        let ((part, value), cmp) = if condition.contains('<') {
            (condition.split_once('<').unwrap(), '<')
        } else {
            (condition.split_once('>').unwrap(), '>')
        };

        let part = part_to_idx(part.chars().next().unwrap());
        let value = value.parse().unwrap();
        let range = if cmp == '<' {
            Interval::new(1, value)
        } else {
            Interval::new(value, 4000)
        };

        Self { part, range }
    }

    fn rev(&self) -> Self {
        let mut result = *self;
        if result.range.0 == 1 {
            result.range = Interval::new((result.range.1 + 1).min(4000), 4000);
        } else {
            result.range = Interval::new(1, (result.range.0 - 1).max(1));
        }
        result
    }

    fn execute(&self, rating: &Ratings) -> bool {
        self.range.inside(rating.ratings[self.part])
    }
}

#[derive(Debug)]
struct WorkflowIf {
    condition: WorkflowCond,
    true_workflow: String,
}

impl WorkflowIf {
    fn parse(if_condition: &str) -> Self {
        let (rest, true_workflow) = if_condition.split_once(':').unwrap();

        Self {
            condition: WorkflowCond::parse(rest),
            true_workflow: true_workflow.to_string(),
        }
    }

    fn execute(&self, rating: &Ratings) -> bool {
        self.condition.execute(rating)
    }
}

fn is_accepted(workflows: &HashMap<String, Workflow>, rating: &Ratings) -> bool {
    let mut current_workflow = "in".to_string();

    loop {
        if current_workflow == "A" {
            return true;
        }
        if current_workflow == "R" {
            return false;
        }

        current_workflow = workflows.get(&current_workflow).unwrap().execute(rating);
    }
}

fn parse_input(input: &str) -> (HashMap<String, Workflow>, Vec<Ratings>) {
    let (workflows, ratings) = input.trim().split_once("\n\n").unwrap();

    let workflows = workflows
        .trim()
        .lines()
        .map(Workflow::parse)
        .map(|workflow| (workflow.name.clone(), workflow))
        .collect::<HashMap<_, _>>();

    let ratings = ratings
        .trim()
        .lines()
        .map(Ratings::parse)
        .collect::<Vec<_>>();

    (workflows, ratings)
}

fn p1(input: &str) -> String {
    let (workflows, ratings) = parse_input(input);

    ratings
        .iter()
        .filter(|rating| is_accepted(&workflows, rating))
        .map(|rating| rating.ratings.iter().sum::<u64>())
        .sum::<u64>()
        .to_string()
}

fn p2(input: &str) -> String {
    let (workflows, _) = parse_input(input);

    fn traverse(
        workflows: &HashMap<String, Workflow>,
        current: &str,
        constraints: &mut Vec<Constraints>,
    ) -> u64 {
        if current == "R" {
            0
        } else if current == "A" {
            constraints.last().unwrap().combos()
        } else {
            let workflow = workflows.get(current).unwrap();
            let mut result = 0;

            workflow.ifs.iter().for_each(|if_workflow| {
                constraints.push(
                    constraints
                        .iter()
                        .last()
                        .unwrap()
                        .apply_constraint(&if_workflow.condition),
                );

                result += traverse(workflows, if_workflow.true_workflow.as_str(), constraints);

                constraints.pop();

                constraints.push(
                    constraints
                        .iter()
                        .last()
                        .unwrap()
                        .apply_constraint(&if_workflow.condition.rev()),
                );
            });

            (0..workflow.ifs.len()).for_each(|_| {
                constraints.pop();
            });

            result += traverse(workflows, workflow.else_workflow.as_str(), constraints);

            result
        }
    }

    traverse(
        &workflows,
        "in",
        &mut vec![Constraints {
            constraints: [
                Some(Interval::new(1, 4000)),
                Some(Interval::new(1, 4000)),
                Some(Interval::new(1, 4000)),
                Some(Interval::new(1, 4000)),
            ],
        }],
    )
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
px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}
    ";

    #[test]
    fn test_p1_sample() {
        assert_eq!(p1(SAMPLE_INPUT), "19114");
    }

    #[test]
    fn test_p1_actual() {
        assert_eq!(p1(ACTUAL_INPUT), "420739");
    }

    #[test]
    fn test_p2_sample() {
        assert_eq!(p2(SAMPLE_INPUT), "167409079868000");
    }

    #[test]
    #[ignore = "not yet implemented"]
    fn test_p2_actual() {
        assert_eq!(p2(ACTUAL_INPUT), "");
    }
}
