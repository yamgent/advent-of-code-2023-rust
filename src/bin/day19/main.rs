use std::collections::HashMap;

const ACTUAL_INPUT: &str = include_str!("../../../actual_inputs/2023/19/input.txt");

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
    constraints: [(u64, u64); 4],
}

const INVALID_CONSTRAINT: (u64, u64) = (99999, 99999);

impl Constraints {
    fn combos(&self) -> u64 {
        if self.constraints.iter().any(|v| *v == INVALID_CONSTRAINT) {
            0
        } else {
            self.constraints.iter().map(|v| v.1 - v.0 + 1).product()
        }
    }

    fn apply_constraint(&self, cond: &WorkflowCond) -> Self {
        let mut result = *self;
        let current_value = result.constraints[cond.part];

        if current_value == INVALID_CONSTRAINT
            || current_value.0 > cond.range.1
            || current_value.1 < cond.range.0
            || current_value == cond.range
        {
            result.constraints[cond.part] = INVALID_CONSTRAINT;
        } else if cond.range.0 > current_value.0 && cond.range.1 < current_value.1 {
            unimplemented!()
        } else if cond.range.0 > current_value.0 {
            result.constraints[cond.part] = (current_value.0, cond.range.0 - 1);
            if result.constraints[cond.part].1 < result.constraints[cond.part].0 {
                result.constraints[cond.part] = INVALID_CONSTRAINT;
            }
        } else {
            result.constraints[cond.part] = (cond.range.1 + 1, current_value.1);
            if result.constraints[cond.part].1 < result.constraints[cond.part].0 {
                result.constraints[cond.part] = INVALID_CONSTRAINT;
            }
        }
        result
    }
}

#[derive(Debug, Clone, Copy)]
struct WorkflowCond {
    part: usize,
    range: (u64, u64),
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
            (1, value)
        } else {
            (value, 4000)
        };

        Self { part, range }
    }

    fn rev(&self) -> Self {
        let mut result = *self;
        if result.range.0 == 1 {
            result.range = ((result.range.1 + 1).min(4000), 4000);
        } else {
            result.range = (1, (result.range.0 - 1).max(0));
        }
        result
    }

    fn execute(&self, rating: &Ratings) -> bool {
        rating.ratings[self.part] >= self.range.0 && rating.ratings[self.part] <= self.range.1
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
            constraints: [(1, 4000), (1, 4000), (1, 4000), (1, 4000)],
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
