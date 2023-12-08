use std::collections::HashMap;

use regex::Regex;

struct Node {
    left: String,
    right: String,
}

fn parse_input(_input: &str) -> (String, HashMap<String, Node>) {
    let split = _input.split("\n\n").collect::<Vec<&str>>();

    let instructions = split[0].to_string();

    let re = Regex::new(r"(?<at>\w{3}) = \((?<left>\w{3}), (?<right>\w{3})\)").unwrap();

    let map = split[1]
        .split("\n")
        .map(|haystack| -> (String, Node) {
            let Some(cap) = re.captures(haystack) else { panic!() };
            let (_, [at, left, right]) = cap.extract();
            (
                at.to_string(),
                Node {
                    left: left.to_string(),
                    right: right.to_string(),
                },
            )
        })
        .collect::<HashMap<String, Node>>();

    return (instructions, map);
}

#[allow(dead_code)]
pub fn part1(_input: &str) -> i32 {
    let (instructions, map) = parse_input(_input);

    let chars = instructions.chars().collect::<Vec<char>>();

    let (mut position, mut steps) = ("AAA".to_string(), 0);
    let mut i: usize = 0;
    loop {
        if position == "ZZZ" {
            break;
        }
        let instruction: char = chars[i];
        let Some(node) = map.get(&position) else { panic!("In loop: {}", position); };
        if instruction == 'L' {
            position = String::from(&node.left);
        } else {
            position = String::from(&node.right);
        }
        steps += 1;
        i += 1;
        if i == chars.len() {
            i = 0;
        }
    }

    return steps;
}

#[allow(dead_code)]
pub fn part2(_input: &str) -> u64 {
    let (instructions, map) = parse_input(_input);

    let chars = instructions.chars().collect::<Vec<char>>();

    let mut positions = map
        .keys()
        .filter(|position| position.ends_with("A"))
        .map(|position| position.to_string())
        .collect::<Vec<String>>();
    // this variable store number of steps for each position to reach "the destination"
    let mut counts = positions.iter().map(|_| 0_u64).collect::<Vec<u64>>();

    let mut steps = 0;
    let mut i: usize = 0;

    loop {
        // instand of check one position, check if all positions ends with "Z"
        if counts.iter().all(|count| count > &0) {
            break;
        }

        let instruction: char = chars[i];
        positions = positions
            .into_iter()
            .map(|position| -> String {
                let Some(node) = map.get(&position) else { panic!("In loop: {}", position); };
                if instruction == 'L' {
                    String::from(&node.left)
                } else {
                    String::from(&node.right)
                }
            })
            .collect::<Vec<String>>();

        steps += 1;

        // check all positions
        for (i, position) in positions.iter().enumerate() {
            if position.ends_with("Z") && counts[i] == 0 {
                counts[i] = steps
            }
        }

        i += 1;
        if i == chars.len() {
            i = 0;
        }
    }

    return counts.into_iter().fold(1_u64, |a, b| lcm(a, b));
}

// https://www.hackertouch.com/least-common-multiple-in-rust.html
// least common mulitple
fn lcm(first: u64, second: u64) -> u64 {
    first * second / gcd(first, second)
}

// greatest common divisor
fn gcd(first: u64, second: u64) -> u64 {
    let (mut max, mut min) = if first > second {
        (first, second)
    } else {
        (second, first)
    };

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }
        max = min;
        min = res;
    }
}
