fn parse_input(_input: &str) -> Vec<i32> {
    _input
        .split(" ")
        .map(|s| s.parse::<i32>().unwrap())
        .collect()
}

fn forward_history(history: Vec<i32>) -> i32 {
    if history.iter().all(|n| n == &0) {
        return 0;
    }

    let mut next_sequence: Vec<i32> = Vec::new();
    for i in 1..history.len() {
        next_sequence.push(history[i] - history[i - 1]);
    }

    let forward = forward_history(next_sequence);

    return history[history.len() - 1] + forward;
}

#[allow(dead_code)]
pub fn part1(_input: &str) -> i32 {
    _input
        .split("\n")
        .map(parse_input)
        .map(forward_history)
        .sum()
}

fn backward_history(history: Vec<i32>) -> i32 {
    if history.iter().all(|n| n == &0) {
        return 0;
    }

    let mut next_sequence: Vec<i32> = Vec::new();
    for i in 1..history.len() {
        next_sequence.push(history[i] - history[i - 1]);
    }

    let backward = backward_history(next_sequence);

    return history[0] - backward;
}

#[allow(dead_code)]
pub fn part2(_input: &str) -> i32 {
    _input
        .split("\n")
        .map(parse_input)
        .map(backward_history)
        .sum()
}
