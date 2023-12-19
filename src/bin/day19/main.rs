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
struct WorkflowCond {
    cmp_rating: String,
    cmp: WorkflowCmp,
    cmp_value: u64,
}

#[derive(Debug, Clone, Copy)]
struct Constraints {
    x: (u64, u64),
    m: (u64, u64),
    a: (u64, u64),
    s: (u64, u64),
}

const INVALID_CONSTRAINT: (u64, u64) = (99999, 99999);

impl Constraints {
    fn combos(&self) -> u64 {
        if self.x == INVALID_CONSTRAINT
            || self.m == INVALID_CONSTRAINT
            || self.a == INVALID_CONSTRAINT
            || self.s == INVALID_CONSTRAINT
        {
            0
        } else {
            (self.x.1 - self.x.0 + 1)
                * (self.m.1 - self.m.0 + 1)
                * (self.a.1 - self.a.0 + 1)
                * (self.s.1 - self.s.0 + 1)
        }
    }
}

fn constraint(current: (u64, u64), cmp: WorkflowCmp, cmp_value: u64) -> (u64, u64) {
    if current == INVALID_CONSTRAINT {
        INVALID_CONSTRAINT
    } else if cmp_value >= current.0 && cmp_value <= current.1 {
        match cmp {
            WorkflowCmp::Gt => {
                if cmp_value + 1 > current.1 {
                    INVALID_CONSTRAINT
                } else {
                    (cmp_value + 1, current.1)
                }
            }
            WorkflowCmp::Lt => {
                if cmp_value == 0 || cmp_value - 1 < current.0 {
                    INVALID_CONSTRAINT
                } else {
                    (current.0, cmp_value - 1)
                }
            }
        }
    } else {
        match cmp {
            WorkflowCmp::Gt => {
                if cmp_value > current.1 {
                    INVALID_CONSTRAINT
                } else {
                    current
                }
            }
            WorkflowCmp::Lt => {
                if cmp_value < current.0 {
                    INVALID_CONSTRAINT
                } else {
                    current
                }
            }
        }
    }
}

fn rev_constraint(current: (u64, u64), cmp: WorkflowCmp, cmp_value: u64) -> (u64, u64) {
    if current == INVALID_CONSTRAINT {
        INVALID_CONSTRAINT
    } else if cmp_value >= current.0 && cmp_value <= current.1 {
        match cmp {
            WorkflowCmp::Lt => {
                if cmp_value > current.1 {
                    INVALID_CONSTRAINT
                } else {
                    (cmp_value, current.1)
                }
            }
            WorkflowCmp::Gt => {
                if cmp_value < current.0 {
                    INVALID_CONSTRAINT
                } else {
                    (current.0, cmp_value)
                }
            }
        }
    } else {
        match cmp {
            WorkflowCmp::Lt => {
                if cmp_value > current.1 {
                    INVALID_CONSTRAINT
                } else {
                    current
                }
            }
            WorkflowCmp::Gt => {
                if cmp_value < current.0 {
                    INVALID_CONSTRAINT
                } else {
                    current
                }
            }
        }
    }
}

impl WorkflowCond {
    fn parse(condition: &str) -> Self {
        let ((rating, value), cmp) = if condition.contains('<') {
            (condition.split_once('<').unwrap(), WorkflowCmp::Lt)
        } else {
            (condition.split_once('>').unwrap(), WorkflowCmp::Gt)
        };
        Self {
            cmp_rating: rating.to_string(),
            cmp,
            cmp_value: value.parse().unwrap(),
        }
    }

    fn execute(&self, rating: &Ratings) -> bool {
        match self.cmp_rating.as_str() {
            "x" => self.cmp.execute(rating.x, self.cmp_value),
            "m" => self.cmp.execute(rating.m, self.cmp_value),
            "a" => self.cmp.execute(rating.a, self.cmp_value),
            "s" => self.cmp.execute(rating.s, self.cmp_value),
            _ => unreachable!(),
        }
    }

    fn gen_true_constraints(&self, current_constraints: &Constraints) -> Constraints {
        let result = match self.cmp_rating.as_str() {
            "x" => Constraints {
                x: constraint(current_constraints.x, self.cmp, self.cmp_value),
                ..*current_constraints
            },
            "m" => Constraints {
                m: constraint(current_constraints.m, self.cmp, self.cmp_value),
                ..*current_constraints
            },
            "a" => Constraints {
                a: constraint(current_constraints.a, self.cmp, self.cmp_value),
                ..*current_constraints
            },
            "s" => Constraints {
                s: constraint(current_constraints.s, self.cmp, self.cmp_value),
                ..*current_constraints
            },
            _ => unreachable!(),
        };

        assert!(result.x.0 <= result.x.1);
        assert!(result.m.0 <= result.m.1);
        assert!(result.a.0 <= result.a.1);
        assert!(result.s.0 <= result.s.1);

        result
    }

    fn gen_false_constraints(&self, current_constraints: &Constraints) -> Constraints {
        let result = match self.cmp_rating.as_str() {
            "x" => Constraints {
                x: rev_constraint(current_constraints.x, self.cmp, self.cmp_value),
                ..*current_constraints
            },
            "m" => Constraints {
                m: rev_constraint(current_constraints.m, self.cmp, self.cmp_value),
                ..*current_constraints
            },
            "a" => Constraints {
                a: rev_constraint(current_constraints.a, self.cmp, self.cmp_value),
                ..*current_constraints
            },
            "s" => Constraints {
                s: rev_constraint(current_constraints.s, self.cmp, self.cmp_value),
                ..*current_constraints
            },
            _ => unreachable!(),
        };

        assert!(result.x.0 <= result.x.1);
        assert!(result.m.0 <= result.m.1);
        assert!(result.a.0 <= result.a.1);
        assert!(result.s.0 <= result.s.1);

        result
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
        .map(|rating| rating.x + rating.m + rating.a + rating.s)
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
                    if_workflow
                        .condition
                        .gen_true_constraints(constraints.iter().last().unwrap()),
                );

                result += traverse(workflows, if_workflow.true_workflow.as_str(), constraints);

                constraints.pop();

                constraints.push(
                    if_workflow
                        .condition
                        .gen_false_constraints(constraints.iter().last().unwrap()),
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
            x: (1, 4000),
            m: (1, 4000),
            a: (1, 4000),
            s: (1, 4000),
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
