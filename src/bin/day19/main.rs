use std::collections::HashMap;

const ACTUAL_INPUT: &str = include_str!("../../../actual_inputs/2023/19/input.txt");

#[derive(Debug)]
struct Ratings {
    x: u64,
    m: u64,
    a: u64,
    s: u64,
}

impl Ratings {
    fn parse(line: &str) -> Self {
        line.trim().replace(['{', '}'], "").split(',').fold(
            Self {
                x: 0,
                m: 0,
                a: 0,
                s: 0,
            },
            |mut acc, rating| {
                let (part, value) = rating.split_once('=').unwrap();
                let value = value.parse().unwrap();
                match part {
                    "x" => acc.x = value,
                    "m" => acc.m = value,
                    "a" => acc.a = value,
                    "s" => acc.s = value,
                    _ => unreachable!(),
                }
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
                if current.contains(":") {
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

#[derive(Debug)]
enum WorkflowCmp {
    Gt,
    Lt,
}
impl WorkflowCmp {
    fn execute(&self, x: u64, y: u64) -> bool {
        match self {
            WorkflowCmp::Gt => x > y,
            WorkflowCmp::Lt => x < y,
        }
    }
}

#[derive(Debug)]
struct WorkflowIf {
    cmp_rating: String,
    cmp: WorkflowCmp,
    cmp_value: u64,
    true_workflow: String,
}

impl WorkflowIf {
    fn parse(condition: &str) -> Self {
        let (rest, true_workflow) = condition.split_once(':').unwrap();
        let ((rating, value), cmp) = if rest.contains('<') {
            (rest.split_once('<').unwrap(), WorkflowCmp::Lt)
        } else {
            (rest.split_once('>').unwrap(), WorkflowCmp::Gt)
        };

        Self {
            cmp_rating: rating.to_string(),
            cmp,
            cmp_value: value.parse().unwrap(),
            true_workflow: true_workflow.to_string(),
        }
    }

    fn execute(&self, rating: &&Ratings) -> bool {
        match self.cmp_rating.as_str() {
            "x" => self.cmp.execute(rating.x, self.cmp_value),
            "m" => self.cmp.execute(rating.m, self.cmp_value),
            "a" => self.cmp.execute(rating.a, self.cmp_value),
            "s" => self.cmp.execute(rating.s, self.cmp_value),
            _ => unreachable!(),
        }
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

fn p1(input: &str) -> String {
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

    ratings
        .iter()
        .filter(|rating| is_accepted(&workflows, &rating))
        .map(|rating| rating.x + rating.m + rating.a + rating.s)
        .sum::<u64>()
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
        assert_eq!(p2(SAMPLE_INPUT), "");
    }

    #[test]
    #[ignore = "not yet implemented"]
    fn test_p2_actual() {
        assert_eq!(p2(ACTUAL_INPUT), "");
    }
}
