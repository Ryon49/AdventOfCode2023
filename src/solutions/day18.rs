use std::collections::HashSet;

type Position = (i32, i32);
type Direction = (i32, i32);

const NORTH: Direction = (-1, 0);
const SOUTH: Direction = (1, 0);
const WEST: Direction = (0, -1);
const EAST: Direction = (0, 1);

fn parse_input(input: &str) -> Vec<(Direction, i32, String)> {
    input
        .lines()
        .map(|line| {
            let parts = line.split(" ").collect::<Vec<&str>>();
            let direction = match parts[0] {
                "R" => EAST,
                "L" => WEST,
                "U" => NORTH,
                "D" => SOUTH,
                _ => panic!("unsupported parsing, {}", parts[0]),
            };

            let step = parts[1].parse::<i32>().unwrap();

            (direction, step, parts[2].to_string())
        })
        .collect()
}

// resuse day10's code.
fn count_reachable(dig_plans: Vec<((i32, i32), i32)>) -> i32 {
    let mut position = (0, 0);

    let mut visited: HashSet<Position> = HashSet::new();
    visited.insert(position);

    let (mut min_row, mut max_row) = (i32::MAX, i32::MIN);
    let (mut min_col, mut max_col) = (i32::MAX, i32::MIN);

    for (direction, step) in dig_plans {
        for _ in 0..step {
            let next = (position.0 + direction.0, position.1 + direction.1);
            visited.insert(next);
            position = next;
        }
        (min_row, max_row) = (min_row.min(position.0), max_row.max(position.0));
        (min_col, max_col) = (min_col.min(position.1), max_col.max(position.1));
    }

    // because it is zero index, length must +1.
    let (m, n) = (max_row - min_row + 1, max_col - min_col + 1);

    // remap to visited
    visited = visited
        .into_iter()
        .map(|position: (i32, i32)| (position.0 - min_row, position.1 - min_col))
        .collect();

    let mut digged_map = vec![vec!['.'; n as usize]; m as usize];
    for i in 0..m {
        for j in 0..n {
            if visited.contains(&(i, j)) {
                digged_map[i as usize][j as usize] = '#';
            }
        }
    }

    // count number of undigged space.
    let mut queue = Vec::new();
    // Add all side to queue

    // row
    for i in 0..m {
        if !visited.contains(&(i, 0)) {
            queue.push((i, 0));
        }
        if !visited.contains(&(i, n - 1)) {
            queue.push((i, n - 1));
        }
    }
    // column
    for j in 0..n {
        if !visited.contains(&(0, j)) {
            queue.push((0, j));
        }
        if !visited.contains(&(m - 1, j)) {
            queue.push((m - 1, j));
        }
    }
    let mut reachable = vec![vec![false; n as usize]; m as usize];
    let mut memo: HashSet<Position> = HashSet::new();

    while queue.len() > 0 {
        let position = queue.remove(0);

        if memo.contains(&position) {
            continue;
        }
        memo.insert(position);
        reachable[position.0 as usize][position.1 as usize] = true;

        // check neighbors
        for connection in [NORTH, SOUTH, EAST, WEST] {
            let next_position = (position.0 + connection.0, position.1 + connection.1);
            if next_position.0 >= 0
                && next_position.0 < m
                && next_position.1 >= 0
                && next_position.1 < n
            {
                if digged_map[next_position.0 as usize][next_position.1 as usize] == '.' {
                    queue.push(next_position);
                }
            }
        }
    }

    let mut result = 0;
    for i in 0..m {
        for j in 0..n {
            if reachable[i as usize][j as usize] {
                result += 1;
            }
        }
    }
    m * n - result
}

#[allow(dead_code)]
pub fn part1(input: &str) -> i32 {
    let dig_plans: Vec<((i32, i32), i32)> = parse_input(input)
        .into_iter()
        .map(|plan| (plan.0, plan.1))
        .collect();
    count_reachable(dig_plans)
}

#[allow(dead_code)]
pub fn part2(input: &str) -> i32 {
    let dig_plans = parse_input(input)
        .into_iter()
        .map(|plan| plan.2)
        .map(|actual_plan| {
            // extract first 5 digit and convert them from hex to decimal
            let step = i32::from_str_radix(&actual_plan.as_str()[2..7], 16).unwrap();
            let direction = match &actual_plan.as_str()[7..8] {
                "0" => EAST,
                "1" => SOUTH,
                "2" => WEST,
                "3" => NORTH,
                _ => panic!("unsupported parsing"),
            };
            (direction, step)
        })
        .collect::<Vec<((i32, i32), i32)>>();
    count_reachable(dig_plans)
}
