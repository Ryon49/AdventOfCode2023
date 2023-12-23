use std::collections::HashSet;

type Position = (i32, i32);
type Direction = (i32, i32);

const NORTH: Direction = (-1, 0);
const SOUTH: Direction = (1, 0);
const WEST: Direction = (0, -1);
const EAST: Direction = (0, 1);

fn parse_input(input: &str) -> (Vec<Vec<char>>, Position) {
    let garden = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    // empty garden only contains the garden plots (".") and rocks ("#")
    let empty_garden = input
        .replace("S", ".")
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    for (i, row) in garden.iter().enumerate() {
        for (j, tile) in row.iter().enumerate() {
            if tile == &'S' {
                return (empty_garden, (i as i32, j as i32));
            }
        }
    }
    unreachable!();
}

#[allow(dead_code)]
pub fn part1(input: &str) -> usize {
    let (garden, starting_position) = parse_input(input);
    let (m, n) = (garden.len() as i32, garden[0].len() as i32);

    let mut reachable: HashSet<Position> = HashSet::new();
    reachable.insert(starting_position);

    let required_steps = 64;
    for _ in 0..required_steps {
        let mut after_step: HashSet<Position> = HashSet::new();

        for reachable_position in reachable {
            for direction in [NORTH, SOUTH, WEST, EAST] {
                let next_position = (
                    reachable_position.0 + direction.0,
                    reachable_position.1 + direction.1,
                );
                if next_position.0 >= 0
                    && next_position.0 < m
                    && next_position.1 >= 0
                    && next_position.1 < n
                    && garden[next_position.0 as usize][next_position.1 as usize] == '.'
                {
                    after_step.insert(next_position);
                }
            }
        }
        reachable = after_step;
        // println!("reachable = {}", reachable.len());
    }
    reachable.len()
}
