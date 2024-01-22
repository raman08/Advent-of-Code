use aoc2023::{get_file_path, read_file};
use itertools::Itertools;
use std::{
    borrow::BorrowMut,
    collections::HashMap,
    ops::{RangeBounds, RangeInclusive},
};

#[derive(Debug)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl Part {
    fn add_up(self: &Self) -> usize {
        self.x + self.m + self.a + self.s
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Target {
    Workflow(String),
    Accepted,
    Rejected,
}

impl Target {
    fn from(id: String) -> Self {
        match id.as_str() {
            "A" => Target::Accepted,
            "R" => Target::Rejected,
            _ => Target::Workflow(id.to_string()),
        }
    }
}

#[derive(Debug, Clone)]
enum Condition {
    Greater,
    Less,
}

#[derive(Debug, Clone)]
enum Rule {
    Test {
        part_sub: String,
        condition: Condition,
        val: usize,
        target: Target,
    },
    Target(Target),
}

enum Apply {
    Split {
        pass: (PartRange, Target),
        fail: PartRange,
    },
    PassedTest(Target),
    FailedTest,
}

impl Rule {
    fn apply(self: &Self, part: &Part) -> Option<&Target> {
        match self {
            Rule::Test {
                part_sub,
                condition,
                val,
                target,
            } => {
                let v = match part_sub.as_str() {
                    "x" => part.x,
                    "m" => part.m,
                    "a" => part.a,
                    "s" => part.s,
                    _ => unreachable!("Invalid part_sub"),
                };
                if match condition {
                    Condition::Greater => v > *val,
                    Condition::Less => v < *val,
                } {
                    Some(target)
                } else {
                    None
                }
            }
            Rule::Target(t) => Some(t),
        }
    }

    fn apply_range(&self, part: &PartRange) -> Apply {
        match self {
            Rule::Test {
                part_sub,
                condition,
                val: value,
                target,
            } => {
                let test_value = match part_sub.as_str() {
                    "x" => &part.x,
                    "m" => &part.m,
                    "a" => &part.a,
                    "s" => &part.s,
                    _ => unreachable!("Invalid part_sub"),
                };

                if test_value.contains(value) {
                    match condition {
                        Condition::Greater => {
                            let mut part_low = part.clone();
                            part_low.set(part_sub.to_string(), *test_value.start()..=*value);

                            let mut part_high = part.clone();
                            part_high.set(part_sub.to_string(), (value + 1)..=*test_value.end());

                            Apply::Split {
                                pass: (part_high, target.clone()),
                                fail: part_low,
                            }
                        }
                        Condition::Less => {
                            let mut part_low = part.clone();
                            part_low.set(part_sub.to_string(), *test_value.start()..=(value - 1));

                            let mut part_high = part.clone();
                            part_high.set(part_sub.to_string(), *value..=*test_value.end());

                            Apply::Split {
                                pass: (part_low, target.clone()),
                                fail: part_high,
                            }
                        }
                    }
                } else {
                    if match condition {
                        Condition::Greater => test_value.start() > value,
                        Condition::Less => test_value.end() < value,
                    } {
                        Apply::PassedTest(target.clone())
                    } else {
                        Apply::FailedTest
                    }
                }
            }
            Rule::Target(target) => Apply::PassedTest(target.clone()),
        }
    }
}

#[derive(Clone)]
struct PartRange {
    x: RangeInclusive<usize>,
    m: RangeInclusive<usize>,
    a: RangeInclusive<usize>,
    s: RangeInclusive<usize>,
}

impl PartRange {
    fn set(&mut self, field: String, val: RangeInclusive<usize>) {
        match field.as_str() {
            "x" => self.x = val,
            "m" => self.m = val,
            "a" => self.a = val,
            "s" => self.s = val,
            _ => unreachable!("Invalid field"),
        }
    }
}

type Workflows = HashMap<String, Vec<Rule>>;

fn parse_lines(input: &String) -> (Workflows, Vec<Part>) {
    let (instructions, parts) = input.split_once("\n\n").unwrap();

    let parts = parts
        .lines()
        .map(|part| {
            let part = &part[1..part.len() - 1];
            let sparts = part
                .split(",")
                .map(|p| p.split_once("=").unwrap().1.parse::<usize>().unwrap())
                .collect_vec();

            Part {
                x: sparts[0],
                m: sparts[1],
                a: sparts[2],
                s: sparts[3],
            }
        })
        .collect_vec();

    let instructions = instructions
        .lines()
        .map(|instruction| {
            let (id, rules) = instruction.split_once("{").unwrap();

            let rules = &rules[0..rules.len() - 1]
                .split(",")
                .map(|rule| match rule.split_once(":") {
                    Some((conditon, target)) => {
                        let c = match conditon.contains("<") {
                            true => Condition::Less,
                            false => Condition::Greater,
                        };
                        let (p, v) = match c {
                            Condition::Less => conditon.split_once("<").unwrap(),
                            Condition::Greater => conditon.split_once(">").unwrap(),
                        };

                        Rule::Test {
                            part_sub: p.to_string(),
                            condition: c,
                            val: v.parse::<usize>().unwrap(),
                            target: Target::from(target.to_string()),
                        }
                    }
                    None => Rule::Target(Target::from(rule.to_string())),
                })
                .collect_vec();
            (id.to_string(), rules.to_vec())
        })
        .fold(HashMap::new(), |mut acc, i| {
            acc.insert(i.0, i.1);
            acc
        });

    (instructions, parts)
}

fn process_workflow(curr: &String, part: &Part, workflows: &Workflows) -> Target {
    let rules = workflows
        .get(curr)
        .expect(&format!("Unexpected workflow rule {:?}", curr));

    for rule in rules {
        match rule.apply(part) {
            Some(t) => match t {
                Target::Workflow(id) => return process_workflow(id, part, workflows),
                Target::Accepted => return Target::Accepted,
                Target::Rejected => return Target::Rejected,
            },
            None => continue,
        };
    }

    unreachable!("This should not be reached!!!!!");
}

fn part_1(lines: &String) -> usize {
    let (instructions, parts) = parse_lines(lines);

    parts
        .iter()
        .filter(|&part| {
            process_workflow(&"in".to_string(), &part, &instructions) == Target::Accepted
        })
        .fold(0, |acc, p| acc + p.add_up())
}

fn process_workflow_range(curr: &Target, part: &PartRange, workflows: &Workflows) -> usize {
    0
}

fn part_2(lines: &String) -> usize {
    let (instructions, _) = parse_lines(lines);
    let curr = PartRange {
        x: 1..=4000,
        m: 1..=4000,
        a: 1..=4000,
        s: 1..=4000,
    };

    process_workflow_range(&Target::Workflow("in".to_string()), &curr, &instructions)
}

fn main() {
    #[allow(unused_variables)]
    let is_sample = true;
    let is_sample = false;
    let path = get_file_path(19, is_sample);
    let contents = read_file(path);

    println!("Part 1: {}", part_1(&contents));
    println!("Part 2: {}", part_2(&contents));
}
