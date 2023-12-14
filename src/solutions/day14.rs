use std::collections::HashMap;

fn calculate_load(board: &Vec<Vec<char>>) -> usize {
    let mut load = 0;
    for (i, row) in board.into_iter().enumerate() {
        for c in row {
            if c == &'O' {
                load += board.len() - i;
            }
        }
    }
    return load;
}

fn parse_input(_input: &str) -> (Vec<Vec<char>>, Vec<Vec<char>>) {
    let board = _input
        .split("\n")
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();

    let blank_board: Vec<Vec<char>> = _input
        .split("\n")
        .map(|line| line.replace("O", ".").chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();
    (board, blank_board)
}

#[allow(dead_code)]
pub fn part1(input: &str) -> usize {
    let (board, mut blank_board) = parse_input(input);
    let (m, n) = (board.len(), board[0].len());

    for j in 0..n {
        // this represent a invisable wall
        let mut placable = 0;
        for i in 0..m {
            if board[i][j] == '#' {
                placable = i + 1;
            } else if board[i][j] == 'O' {
                blank_board[placable][j] = 'O';
                placable = placable + 1;
            }
        }
    }
    calculate_load(&blank_board)
}

#[allow(dead_code)]
pub fn part2(input: &str) -> usize {
    let mut memo = HashMap::new();

    let (mut board, blank_board) = parse_input(input);
    let (m, n) = (board.len(), board[0].len());

    let (mut cycle, total_cycle) = (0, 1000000000);
    while cycle < total_cycle {
        let mut roll_north = blank_board.clone();

        // roll north
        for j in 0..n {
            // this represent a invisable wall
            let mut placable = 0;
            for i in 0..m {
                if board[i][j] == '#' {
                    placable = i + 1;
                } else if board[i][j] == 'O' {
                    roll_north[placable][j] = 'O';
                    placable = placable + 1;
                }
            }
        }

        // roll west
        let mut roll_west = blank_board.clone();
        for i in 0..m {
            let mut placable = 0;
            for j in 0..n {
                if roll_north[i][j] == '#' {
                    placable = j + 1;
                } else if roll_north[i][j] == 'O' {
                    roll_west[i][placable] = 'O';
                    placable = placable + 1;
                }
            }
        }

        // roll south
        let mut roll_south = blank_board.clone();
        for j in 0..n {
            // this represent a invisable wall
            let mut placable = m - 1;
            for i in (0..m).rev() {
                if roll_west[i][j] == '#' {
                    if i != 0 {
                        placable = i - 1;
                    }
                } else if roll_west[i][j] == 'O' {
                    roll_south[placable][j] = 'O';
                    if i != 0 {
                        placable = placable - 1;
                    }
                }
            }
        }

        // roll east
        let mut roll_east = blank_board.clone();
        for i in 0..m {
            let mut placable = n - 1;
            for j in (0..n).rev() {
                if roll_south[i][j] == '#' {
                    if j != 0 {
                        placable = j - 1;
                    }
                } else if roll_south[i][j] == 'O' {
                    roll_east[i][placable] = 'O';
                    if j != 0 {
                        placable = placable - 1;
                    }
                }
            }
        }

        board = roll_east;

        // remember the current board
        let memo_key = to_string(&board);
        if let Some(prev) = memo.get(&memo_key) {
            // cycle found, skip cycle
            let diff = cycle - prev;
            while (cycle + diff) < total_cycle {
                cycle += diff;
            }
        }
        memo.insert(memo_key, cycle);
        cycle += 1;
    }
    // we are guaranteed a cycle
    return calculate_load(&board);
}

fn to_string(board: &Vec<Vec<char>>) -> String {
    board
        .into_iter()
        .map(|r| String::from_iter(r.into_iter()))
        .collect::<Vec<String>>()
        .join("\n")
}
