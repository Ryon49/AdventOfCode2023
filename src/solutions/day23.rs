use std::{collections::{HashMap, HashSet}, vec};

type Position = (i32, i32);
type Direction = (i32, i32);

const NORTH: Direction = (-1, 0);
const SOUTH: Direction = (1, 0);
const WEST: Direction = (0, -1);
const EAST: Direction = (0, 1);

fn parse_input(input: &str) -> (Vec<Vec<char>>, Position, Position) {
    let map = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut starting_poisition = (0, 0);
    for (i, c) in map[0].iter().enumerate() {
        if c == &'.' {
            starting_poisition = (0, i as i32);
            break;
        }
    }

    let mut stop_poisition = (0, 0);
    for (i, c) in map[map.len() - 1].iter().enumerate() {
        if c == &'.' {
            stop_poisition = ((map.len() - 1) as i32, i as i32);
            break;
        }
    }

    (map, starting_poisition, stop_poisition)
}

fn as_key(steps: &HashSet<Position>) -> String {
    let mut v = Vec::from_iter(steps);
    v.sort();
    v.into_iter().map(|p| format!("{:?}", p)).collect::<Vec<String>>().join(",")
}

fn dfs(
    map: &Vec<Vec<char>>,
    current_position: Position,
    stop_position: Position,
    mut steps: HashSet<Position>,
    mut memo: HashMap<String, usize>,
) -> usize {
    let memo_key = as_key(&steps);
    if memo.contains_key(&memo_key) {
        let r = memo.get(&memo_key).unwrap();
        println!("found a hit: {}", r);
        return *r;
    }
    if steps.contains(&current_position) {
        return 0;
    }
    if current_position == stop_position {
        return steps.len();
    }
    let (m, n) = (map.len() as i32, map[0].len() as i32);
    // check boundary
    if !(current_position.0 >= 0
        && current_position.0 < m
        && current_position.1 >= 0
        && current_position.1 < n)
    {
        return 0;
    }

    let trail = map[current_position.0 as usize][current_position.1 as usize];
    if trail == '#' {
        return 0;
    }
    steps.insert(current_position);

    let mut max_result = 0;

    if trail == '^' {
        let next_position = (current_position.0 + NORTH.0, current_position.1 + NORTH.1);
        max_result = dfs(
            map,
            next_position,
            stop_position,
            steps.clone(),
            memo.clone(),
        );
    } else if trail == '>' {
        let next_position = (current_position.0 + EAST.0, current_position.1 + EAST.1);
        max_result = dfs(
            map,
            next_position,
            stop_position,
            steps.clone(),
            memo.clone(),
        );
    } else if trail == 'v' {
        let next_position = (current_position.0 + SOUTH.0, current_position.1 + SOUTH.1);
        max_result = dfs(
            map,
            next_position,
            stop_position,
            steps.clone(),
            memo.clone(),
        );
    } else if trail == '<' {
        let next_position = (current_position.0 + WEST.0, current_position.1 + WEST.1);
        max_result = dfs(
            map,
            next_position,
            stop_position,
            steps.clone(),
            memo.clone(),
        );
    } else {
        for direction in [NORTH, SOUTH, EAST, WEST] {
            let next_position = (
                current_position.0 + direction.0,
                current_position.1 + direction.1,
            );
            max_result = max_result.max(dfs(
                map,
                next_position,
                stop_position,
                steps.clone(),
                memo.clone(),
            ));
        }
    }

    // print_map(map, &steps);

    memo.insert(memo_key, max_result);
    return max_result;
}

#[allow(dead_code)]
pub fn part1(input: &str) -> usize {
    let (map, starting_poisition, stop_poisition) = parse_input(input);

    dfs(
        &map,
        starting_poisition,
        stop_poisition,
        HashSet::new(),
        HashMap::new(),
    )
}

#[allow(dead_code)]
pub fn part2(input: &str) -> usize {
    let modified = input
        .replace("^", ".")
        .replace(">", ".")
        .replace("v", ".")
        .replace("<", ".");
    let (map, starting_poisition, stop_poisition) = parse_input(modified.as_str());

    dfs(
        &map,
        starting_poisition,
        stop_poisition,
        HashSet::new(),
        HashMap::new(),
    )
}

pub fn print_map(map: &Vec<Vec<char>>, steps: &Vec<Position>) {
    println!("{}", steps.len());

    for (i, line) in map.iter().enumerate() {
        for (j, c) in line.iter().enumerate() {
            let key = (i as i32, j as i32);
            if steps.contains(&key) {
                print!("O");
            } else {
                print!("{}", c);
            }
        }
        println!()
    }
}
