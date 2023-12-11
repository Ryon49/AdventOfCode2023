use std::{cmp::Ordering, collections::HashSet};

type Position = (i32, i32);
type Direction = Position;

const SOUTH: Direction = (1, 0);
const NORTH: Direction = (-1, 0);
const EAST: Direction = (0, 1);
const WEST: Direction = (0, -1);

fn pipe_kind(c: char) -> Vec<Direction> {
    match c {
        'S' => vec![NORTH, SOUTH, EAST, WEST],
        '|' => vec![NORTH, SOUTH],
        '-' => vec![EAST, WEST],
        'L' => vec![NORTH, EAST],
        'J' => vec![NORTH, WEST],
        '7' => vec![SOUTH, WEST],
        'F' => vec![SOUTH, EAST],
        '.' => vec![],
        _ => panic!("unknown c found: {}", c),
    }
}

fn reverse_direction(pipe: Direction) -> Direction {
    match pipe {
        SOUTH => NORTH,
        NORTH => SOUTH,
        EAST => WEST,
        WEST => EAST,
        _ => panic!("reverse_direction, {:?}", pipe),
    }
}

fn parse_input(_input: &str) -> ((i32, i32), Vec<Vec<char>>) {
    let tiles = _input
        .split("\n")
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    for (i, row) in tiles.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if 'S'.cmp(c) == Ordering::Equal {
                return ((i as i32, j as i32), tiles);
            }
        }
    }

    panic!("Starting position not found");
}

fn check_bound(position: Position, m: i32, n: i32) -> bool {
    return position.0 >= 0 && position.0 < m && position.1 >= 0 && position.1 < n;
}

fn find_pipe_connection(tiles: &Vec<Vec<char>>, position: Position) -> Vec<Position> {
    let (m, n) = (tiles.len() as i32, tiles[1].len() as i32);

    let mut neighbors = Vec::new();

    // check for surrounding pipes
    let pipe = tiles[position.0 as usize][position.1 as usize];
    for connection in pipe_kind(pipe) {
        let next_position = (position.0 + connection.0, position.1 + connection.1);
        if check_bound(next_position, m, n) {
            let next_pipe = tiles[next_position.0 as usize][next_position.1 as usize];
            if pipe_kind(next_pipe).contains(&reverse_direction(connection)) {
                neighbors.push(next_position);
            }
        }
    }
    neighbors
}

#[allow(dead_code)]
pub fn part1(_input: &str) -> i32 {
    let (start, tiles) = parse_input(_input);

    println!("start = {:?}", start);

    let mut memo: HashSet<Position> = HashSet::new();

    // the starting queue will be the neighbor pipes that has connection to start
    let mut queue = Vec::from([(start, 0)]);

    let mut max_steps = 0;

    while queue.len() > 0 {
        let mut next_queue = Vec::new();

        for (position, step) in queue.into_iter() {
            memo.insert(position);
            for neighbor in find_pipe_connection(&tiles, position) {
                if memo.contains(&neighbor) {
                    continue;
                }
                next_queue.push((neighbor, step + 1));
            }
            if step > max_steps {
                max_steps = step;
            }
        }
        queue = next_queue;
    }

    max_steps
}

const NORTH_WEST: Direction = (-1, -1);
const NORTH_EAST: Direction = (-1, 1);
const SOUTH_WEST: Direction = (1, -1);
const SOUTH_EAST: Direction = (1, 1);

// the idea is to create a leak map, starting from the edge, and traverse (dfs) in. mark non
#[allow(dead_code)]
pub fn part2(_input: &str) -> i32 {
    let (_, tiles) = parse_input(_input);
    let (m, n) = (tiles.len() as i32, tiles[0].len() as i32);

    let mut reachable = tiles
        .iter()
        .map(|r| r.iter().map(|_| false).collect::<Vec<bool>>())
        .collect::<Vec<Vec<bool>>>();

    let mut queue = Vec::new();

    // Add all side to queue
    // left
    queue.extend(
        (0..tiles.len())
            .map(|row| (row as i32, 0))
            .collect::<Vec<Position>>(),
    );
    // right
    queue.extend(
        (0..tiles.len())
            .map(|row| (row as i32, tiles[0].len() as i32 - 1))
            .collect::<Vec<Position>>(),
    );
    // top
    queue.extend(
        (0..tiles[0].len())
            .map(|col| (0, col as i32))
            .collect::<Vec<Position>>(),
    );
    // bottom
    queue.extend(
        (0..tiles[0].len())
            .map(|col| (tiles.len() as i32 - 1, col as i32))
            .collect::<Vec<Position>>(),
    );

    let mut memo: HashSet<Position> = HashSet::new();
    while queue.len() > 0 {
        let position = queue.remove(0);

        if memo.contains(&position) {
            continue;
        }
        memo.insert(position);
        println!("{:?}", position);
        reachable[position.0 as usize][position.1 as usize] = true;

        // check neighbors
        for connection in pipe_kind('S') {
            let next_position = (position.0 + connection.0, position.1 + connection.1);
            if check_bound(next_position, m, n) {
                if tiles[next_position.0 as usize][next_position.1 as usize] == '.' {
                    queue.push(next_position);
                }
            }
        }

        // print_map(m, n, &reachable, &tiles);
    }

    let mut result = 0;

    for i in 0..(m as usize) {
        for j in 0..(n as usize) {
            if !reachable[i][j] && tiles[i][j] == '.' {
                result += 1;
            }
        }
    }
    result
}

fn print_map(m: i32, n: i32, leak_map: &Vec<Vec<bool>>, tiles: &Vec<Vec<char>>) {
    for i in 0..(m as usize) {
        for j in 0..(n as usize) {
            if !leak_map[i][j] && tiles[i][j] == '.' {
                print!("{}", "I")
            } else {
                print!("{}", ".")
            }
        }
        println!()
    }
}