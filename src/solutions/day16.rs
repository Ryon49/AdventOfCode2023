use std::collections::HashSet;

type Position = (i32, i32);
type Direction = (i32, i32);

const NORTH: Direction = (-1, 0);
const SOUTH: Direction = (1, 0);
const WEST: Direction = (0, -1);
const EAST: Direction = (0, 1);

fn parse_input(input: &str) -> (Vec<Vec<char>>, Vec<Vec<bool>>) {
    let layout: Vec<Vec<char>> = input
        .split("\n")
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    let empty = layout
        .iter()
        .map(|line| line.iter().map(|_| false).collect::<Vec<bool>>())
        .collect();
    (layout, empty)
}

fn count_enegerized(
    layout: &Vec<Vec<char>>, // layout is always immutable, safe to just reference
    mut energized: Vec<Vec<bool>>,
    starting: (Position, Direction),
) -> i32 {
    let (m, n) = (layout.len() as i32, layout[0].len() as i32);

    // Remembers the position and direction of a beam to prevent reprocess.
    let mut tiles_visited: HashSet<(Position, Direction)> = HashSet::new();

    // Stores all beams not processed.
    let mut beams: Vec<(Position, Direction)> = Vec::new();

    // beam starts at top left
    beams.push(starting);

    while !beams.is_empty() {
        // ((row index, column index), (row direction, column direction))
        let ((r, c), mut dir) = beams.remove(0);

        if !(r >= 0 && r < m && c >= 0 && c < n) {
            // invalid index
            continue;
        }
        if tiles_visited.contains(&((r, c), dir)) {
            continue;
        }

        // mark beam's position energized
        tiles_visited.insert(((r, c), dir));
        energized[r as usize][c as usize] = true;

        let behavior = layout[r as usize][c as usize];

        if behavior == '.' {
            // do nothing
        } else if behavior == '/' {
            // mirror
            dir = match dir {
                EAST => NORTH,
                NORTH => EAST,
                WEST => SOUTH,
                SOUTH => WEST,
                _ => panic!("unsupported '/'"),
            };
        } else if behavior == '\\' {
            // mirror
            dir = match dir {
                EAST => SOUTH,
                NORTH => WEST,
                WEST => NORTH,
                SOUTH => EAST,
                _ => panic!("unsupported '\'"),
            };
        } else if behavior == '|' && !(dir == NORTH || dir == SOUTH) {
            // splitter
            // save the beam split for NORTH
            let next_north_beam = (r + NORTH.0, c + NORTH.1);
            beams.push((next_north_beam, NORTH));

            // let current beam will continue SOUTH
            dir = SOUTH
        } else if behavior == '-' && !(dir == WEST || dir == EAST) {
            // splitter
            // save the beam split for WEST
            let next_west_beam = (r + WEST.0, c + WEST.1);
            beams.push((next_west_beam, WEST));

            // let current beam will continue EAST
            dir = EAST;
        }

        let next_beam = (r + dir.0, c + dir.1);
        beams.push((next_beam, dir));
    }

    let mut result = 0;
    for row in energized.into_iter() {
        for energize in row {
            if energize {
                result += 1;
            }
        }
    }
    result
}

#[allow(dead_code)]
pub fn part1(input: &str) -> i32 {
    let (layout, energized_blank) = parse_input(input);
    count_enegerized(&layout, energized_blank.clone(), ((0, 0), EAST))
}

#[allow(dead_code)]
pub fn part2(input: &str) -> i32 {
    let (layout, energized_blank) = parse_input(input);
    let (m, n) = (layout.len() as i32, layout[0].len() as i32);

    let mut all_startings = Vec::new();
    // top row
    for j in 0..n {
        all_startings.push(((0, j), SOUTH));
    }
    // bottom row
    for j in 0..n {
        all_startings.push(((m - 1, j), NORTH));
    }
    // left column
    for i in 0..m {
        all_startings.push(((i, 0), EAST));
    }
    // right column
    for i in 0..m {
        all_startings.push(((i, n - 1), WEST));
    }

    all_startings
        .into_iter()
        .map(|starting| count_enegerized(&layout, energized_blank.clone(), starting))
        .max()
        .unwrap()
}
