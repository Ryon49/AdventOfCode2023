fn find_difference(line1: &str, line2: &str) -> i32 {
    let mut count = 0;
    let (c1, c2) = (
        line1.chars().collect::<Vec<char>>(),
        line2.chars().collect::<Vec<char>>(),
    );
    for i in 0..c1.len() {
        if c1[i] != c2[i] {
            count += 1;
        }
    }
    count
}

// constrain: alawys i < j
// for part 1, max_smudge should be 0.
// for part 2, max_smudge should be 1.
fn check_matching(
    lines: &Vec<&str>,
    i: usize,
    j: usize,
    max_steps: usize,
    max_smudge: i32,
) -> bool {
    let mut smudge = 0;
    for step in 0..=max_steps {
        let diff = find_difference(lines[i - step], lines[j + step]);
        smudge += diff;
    }
    smudge == max_smudge
}

fn rotate_pattern(pattern: &str) -> String {
    let mut result: Vec<String> = Vec::new();

    let lines = pattern.split("\n").collect::<Vec<&str>>();
    for i in 0..lines[0].len() {
        let column = lines
            .iter()
            .map(|lines| lines.chars().nth(i).unwrap())
            .collect::<String>();
        result.push(column);
    }
    return result.join("\n");
}

// loop over pairs of lines
fn find_pattern(pattern: &str, max_smudge: i32) -> i32 {
    let lines = pattern.split("\n").collect::<Vec<&str>>();

    for i in 1..lines.len() {
        // max_steps decided how far we want to check for mirror reflection,
        // the value should be the min distance for (i-1) to index 0 and (i to lines.len() - 1);
        let max_steps = if (i - 1) < (lines.len() - i - 1) {
            i - 1
        } else {
            lines.len() - i - 1
        };
        if check_matching(&lines, i - 1, i, max_steps, max_smudge) {
            return i as i32;
        }
    }
    0
}

// There is only 1 valid pattern.
fn summarize_pattern(pattern: &str, max_smudge: i32) -> i32 {
    // look over horizontally first
    let horizontal = find_pattern(pattern, max_smudge);
    if horizontal > 0 {
        return horizontal * 100;
    }
    // look over vertial now.
    // rotate the pattern
    let rotated_pattern = rotate_pattern(pattern);
    return find_pattern(&rotated_pattern, max_smudge);
}

#[allow(dead_code)]
pub fn part1(_input: &str) -> i32 {
    _input
        .split("\n\n")
        .map(|pattern| summarize_pattern(pattern, 0))
        .sum()
}

#[allow(dead_code)]
pub fn part2(_input: &str) -> i32 {
    _input
        .split("\n\n")
        .map(|pattern| summarize_pattern(pattern, 1))
        .sum()
}
