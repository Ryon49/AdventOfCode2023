use std::collections::HashMap;

use regex::Regex;

#[derive(Debug)]
struct Rule {
    category: String,                // specify which category is tested on
    predicate: fn(u32, u32) -> bool, // comparison: ">" or "<"
    target: u32,
    destination: String,
}

// Comparsion function.
const LT: fn(u32, u32) -> bool = |v1: u32, v2: u32| v1 < v2; // Less than
const GT: fn(u32, u32) -> bool = |v1: u32, v2: u32| v1 > v2; // Greater than
const PASS: fn(u32, u32) -> bool = |_: u32, _: u32| true; // Always true, Used only for last case in workflow

impl Rule {
    fn new(input: &str) -> Self {
        let re =
            Regex::new(r"(?<on>[xmas])(?<sign><|>)(?<value>\d+):(?<target>[a-zA-Z]+)").unwrap();

        let (_, [category, sign, target_value, destination]) =
            re.captures(input).unwrap().extract();

        let predicate: fn(u32, u32) -> bool = if sign == "<" { LT } else { GT };

        Rule {
            category: category.to_string(),
            predicate: predicate,
            target: target_value.parse::<u32>().unwrap(),
            destination: destination.to_string(),
        }
    }
}

#[derive(Debug)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
}

impl Workflow {
    fn new(input: &str) -> Self {
        let Some(index) = input.find("{") else { panic!("invalid parsing, Workflow::new") };
        let name = &input[..index];
        let raw_rules = input[index + 1..input.len() - 1] // input.len() - 1 to skip "}"
            .split(",")
            .collect::<Vec<&str>>();

        let mut rules = raw_rules
            .iter()
            .take(raw_rules.len() - 1)
            .map(|r| Rule::new(*r))
            .collect::<Vec<Rule>>();

        // The end destination is handled seperately
        rules.push(Rule {
            category: "x".to_string(),
            predicate: PASS,
            target: 0,
            destination: raw_rules[raw_rules.len() - 1].to_string(),
        });

        Workflow {
            name: name.to_string(),
            rules: rules,
        }
    }
}

fn parse_input(input: &str) -> (HashMap<String, Workflow>, Vec<HashMap<&str, u32>>) {
    let sections = input.split("\n\n").collect::<Vec<&str>>();

    // parse workflow
    let workflows = sections[0]
        .lines()
        .map(Workflow::new)
        .map(|workflow| (workflow.name.clone(), workflow)) // convert into (name, workflow) pair
        .collect::<HashMap<String, Workflow>>();

    let parts = sections[1]
        .lines()
        .map(|line| line.strip_prefix("{").unwrap().strip_suffix("}").unwrap()) // remove "{" and "}"
        .map(|line| {
            // convert into list of (category, value) pairs and create a hashmap
            line.split(",")
                .map(|category| {
                    let symbol = &category[0..1]; // first char
                    let value = &category[2..].parse::<u32>().unwrap();

                    (symbol, *value)
                })
                .collect::<HashMap<&str, u32>>()
        })
        .collect::<Vec<HashMap<&str, u32>>>();

    (workflows, parts)
}

#[allow(dead_code)]
pub fn part1(input: &str) -> u32 {
    let (workflows, parts) = parse_input(input);

    let mut sum = 0;
    for part in parts {
        let mut current_workflow = "in";

        // Assume each part will reach either "R" or "A"
        loop {
            let workflow = workflows.get(current_workflow).unwrap();

            for rule in &workflow.rules {
                let category_value = part.get(rule.category.as_str()).unwrap();
                if (rule.predicate)(*category_value, rule.target) {
                    current_workflow = rule.destination.as_str();
                    break;
                }
            }
            // println!()
            if current_workflow == "R" {
                // println!("rejected: {:?}", part);
                break;
            } else if current_workflow == "A" {
                // println!("accepted: {:?}", part);
                sum += part.values().sum::<u32>();
                break;
            }
        }
    }
    sum
}
