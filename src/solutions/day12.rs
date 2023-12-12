use std::collections::HashMap;

fn parse_input(_input: &str) -> Vec<(&str, Vec<i32>)> {
    _input
        .split("\n")
        .map(|line| {
            let pair = line.split(" ").collect::<Vec<&str>>();

            (
                pair[0],
                pair[1]
                    .split(",")
                    .map(|group| group.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>(),
            )
        })
        .collect::<Vec<(&str, Vec<i32>)>>()
}

fn calculate_arrangement<'a>(
    arrangement: &'a str,
    broken_groups: &Vec<i32>,
    previous: char,
    memo: &mut HashMap<(&'a str, String, char), u64>,
) -> u64 {
    // reach end of input. check if broken_groups is exhausted.
    if arrangement.len() == 0 {
        if broken_groups.len() == 0 || (broken_groups.len() == 1 && broken_groups[0] == 0) {
            return 1
        }
    }

    // broken_groups is exhausted, check if there is extra '#'
    if broken_groups.len() == 0 {
        if arrangement.find("#") == None {
            return 1;
        } else {
            return 0;
        }
    }

    // invalid input
    if broken_groups[0] < 0 || arrangement.len() == 0 {
        return 0;
    }

    // check memo
    let memo_key = (
        arrangement,
        broken_groups
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(","),
        previous,
    );
    if memo.contains_key(&memo_key) {
        return *memo.get(&memo_key).unwrap();
    }

    let mut cloned = broken_groups.clone();

    let c = arrangement.chars().nth(0).unwrap();
    if c == '#' {
        cloned[0] -= 1;
        return calculate_arrangement(&arrangement[1..arrangement.len()], &cloned, '#', memo);
    }
    if c == '.' {
        // invalid condition for "#." with [2...]
        if previous == '#' && broken_groups[0] > 0 {
            memo.insert(memo_key, 0);
            return 0;
        }
        if cloned[0] == 0 {
            cloned.remove(0);
        }
        return calculate_arrangement(&arrangement[1..arrangement.len()], &cloned, '.', memo);
    }
    // This is a special condition for continuing "#?" case
    if c == '?' && previous == '#' {
        if cloned[0] == 0 {
            // c must be '.'
            cloned.remove(0);
            return calculate_arrangement(&arrangement[1..arrangement.len()], &cloned, '.', memo);
        } else {
            // c must be '#'
            cloned[0] -= 1;
            return calculate_arrangement(&arrangement[1..arrangement.len()], &cloned, '#', memo);
        }
    }

    let mut result = 0;
    // rest are case for '?' be '#' or '.'
    {
        // for '.'
        result += calculate_arrangement(&arrangement[1..arrangement.len()], &cloned, '.', memo);
    }
    {
        // for '#'
        cloned[0] -= 1;
        result += calculate_arrangement(&arrangement[1..arrangement.len()], &cloned, '#', memo);
    }

    memo.insert(memo_key, result);

    result
}

#[allow(dead_code)]
pub fn part1(_input: &str) -> u64 {
    let records = parse_input(_input);

    records
        .into_iter()
        .map(|(arrangement, broken_groups)| {
            let mut memo = HashMap::new();
            calculate_arrangement(arrangement, &broken_groups, '?', &mut memo)
        })
        .sum()
}

#[allow(dead_code)]
pub fn part2(_input: &str) -> u64 {
    let records = parse_input(_input);

    records
        .into_iter()
        .map(|(arrangement, broken_groups)| {
            // expand input by 5x
            (
                vec![arrangement].repeat(5).join("?"),
                broken_groups.repeat(5),
            )
        })
        .map(|(arrangement, broken_groups)| {
            let mut memo = HashMap::new();
            calculate_arrangement(&arrangement, &broken_groups, '?', &mut memo)
        })
        .sum()
}
