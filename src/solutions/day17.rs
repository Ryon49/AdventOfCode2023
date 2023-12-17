use std::collections::{BinaryHeap, HashMap, HashSet};

type Position = (i32, i32);
type Direction = (i32, i32);

const NORTH: Direction = (-1, 0);
const SOUTH: Direction = (1, 0);
const WEST: Direction = (0, -1);
const EAST: Direction = (0, 1);

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct State {
    position: Position,
    direction: Direction,
    loss: u32,
    continue_straight: u32,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.loss.cmp(&self.loss)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(other.loss.cmp(&self.loss))
    }
}

fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input
        .split("\n")
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect()
}

fn change_directions(direction: Direction) -> [Direction; 2] {
    match direction {
        NORTH | SOUTH => [WEST, EAST],
        EAST | WEST => [NORTH, SOUTH],
        _ => panic!("unsupported direction: {:?}", direction),
    }
}

fn dijkstra(map: Vec<Vec<u32>>, min_straight: u32, max_straight: u32) -> u32 {
    let (m, n) = (map.len() as i32, map[0].len() as i32);

    let mut visited: HashSet<State> = HashSet::new();
    let mut costs: HashMap<(Position, Direction, u32), u32> = HashMap::new();

    let mut queue: BinaryHeap<State> = BinaryHeap::new();
    queue.push(State {
        position: (0, 0),
        direction: EAST,
        loss: 0,
        continue_straight: 0,
    });
    queue.push(State {
        position: (0, 0),
        direction: SOUTH,
        loss: 0,
        continue_straight: 0,
    });

    let mut minimal = u32::MAX;
    while let Some(state) = queue.pop() {
        let State {
            position,
            direction,
            mut loss,
            mut continue_straight,
        } = state;

        if visited.contains(&state) {
            continue;
        }
        visited.insert(state);

        loss += map[position.0 as usize][position.1 as usize];
        continue_straight += 1;

        let key = (position, direction, continue_straight);
        if let Some(saved_loss) = costs.get(&key) {
            if &loss > saved_loss {
                continue;
            }
        }

        // only remember the loss if has traveled "min_straight" crucibles
        if continue_straight >= min_straight {
            costs.insert(key, loss);
            if position == (m - 1, n - 1) {
                minimal = minimal.min(loss);
            }
        }

        // handle continue straight
        if continue_straight != max_straight {
            let next_position = (position.0 + direction.0, position.1 + direction.1);
            if next_position.0 >= 0
                && next_position.0 < m
                && next_position.1 >= 0
                && next_position.1 < n
            {
                // println!("added {:?}", next_position);
                queue.push(State {
                    position: next_position,
                    direction: direction,
                    loss: loss,
                    continue_straight: continue_straight,
                })
            }
        }
        if continue_straight >= min_straight {
            // handle change direction
            for next_direction in change_directions(direction) {
                let next_position = (position.0 + next_direction.0, position.1 + next_direction.1);
                if next_position.0 >= 0
                    && next_position.0 < m
                    && next_position.1 >= 0
                    && next_position.1 < n
                {
                    // println!("added {:?}", next_position);
                    queue.push(State {
                        position: next_position,
                        direction: next_direction,
                        loss: loss,
                        continue_straight: 0,
                    });
                }
            }
        }
    }
    // remove value of starting location
    minimal - map[0][0]
}

// Dijkstra shortest path
#[allow(dead_code)]
pub fn part1(input: &str) -> u32 {
    let map = parse_input(input);
    dijkstra(map, 1, 3)
}

#[allow(dead_code)]
pub fn part2(input: &str) -> u32 {
    let map = parse_input(input);
    dijkstra(map, 4, 10)
}
