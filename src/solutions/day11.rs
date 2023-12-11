fn parse_input(_input: &str) -> (Vec<(u64, u64)>, Vec<u64>, Vec<u64>) {
    let map = _input
        .split("\n")
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let empty_rows = map
        .iter()
        .enumerate()
        .filter(|(_, row)| row.iter().all(|c| c == &'.'))
        .map(|(i, _)| i as u64)
        .collect::<Vec<u64>>();

    let empty_columns = (0..map[0].len())
        .into_iter()
        .filter(|j| {
            let col = map.iter().map(|r| r[*j]).collect::<Vec<char>>();
            col.iter().all(|c| c == &'.')
        })
        .map(|j| j as u64)
        .collect::<Vec<u64>>();

    let mut galaxies = Vec::new();
    // records positions of '#'
    for (i, row) in map.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if c == &'#' {
                galaxies.push((i as u64, j as u64));
            }
        }
    }
    (galaxies, empty_rows, empty_columns)
}

fn galaxy_distance(
    g1: (u64, u64),
    g2: (u64, u64),
    expansion: u64,
    empty_rows: &Vec<u64>,
    empty_columns: &Vec<u64>,
) -> u64 {
    // sort: compare (smaller, larger)
    let (r1, r2) = if g1.0 < g2.0 {
        (g1.0, g2.0)
    } else {
        (g2.0, g1.0)
    };

    let (c1, c2) = if g1.1 < g2.1 {
        (g1.1, g2.1)
    } else {
        (g2.1, g1.1)
    };

    let mut distance = (r2 - r1) + (c2 - c1);
    for row in empty_rows.clone() {
        if row > r1 && row < r2 {
            distance += expansion;
        }
    }
    for column in empty_columns.clone() {
        if column > c1 && column < c2 {
            distance += expansion;
        }
    }

    distance
}

#[allow(dead_code)]
pub fn part1(_input: &str) -> u64 {
    let (galaxies, empty_rows, empty_columns) = parse_input(_input);

    let mut galaxy_pairs: Vec<((u64, u64), (u64, u64))> = Vec::new();
    for i in 0..galaxies.len() {
        for j in (i + 1)..galaxies.len() {
            galaxy_pairs.push((galaxies[i], galaxies[j]));
        }
    }

    let expansion = 2;
    // here should be expansion - 1 because "twice as big" = 1 (existing) + 1 (new)
    // and the existing distance is already accounted.
    galaxy_pairs
        .into_iter()
        .map(|(g1, g2)| galaxy_distance(g1, g2, expansion - 1, &empty_rows, &empty_columns))
        .sum()
}

#[allow(dead_code)]
pub fn part2(_input: &str) -> u64 {
    let (galaxies, empty_rows, empty_columns) = parse_input(_input);

    let mut galaxy_pairs: Vec<((u64, u64), (u64, u64))> = Vec::new();
    for i in 0..galaxies.len() {
        for j in (i + 1)..galaxies.len() {
            galaxy_pairs.push((galaxies[i], galaxies[j]));
        }
    }

    let expansion = 1000000u64;
    galaxy_pairs
        .into_iter()
        .map(|(g1, g2)| galaxy_distance(g1, g2, expansion - 1, &empty_rows, &empty_columns))
        .sum()
}

// odd code for solving part1 naively.
// #[allow(dead_code)]
// pub fn part1(_input: &str) -> i32 {
//     let map: Vec<Vec<char>> = parse_input(_input);

//     // handle row expansion
//     let row_expanded_map = map
//         .into_iter()
//         .map(|row| {
//             if row.iter().all(|c| c == &'.') {
//                 return vec![row.clone(), row];
//             }
//             vec![row]
//         })
//         .flatten()
//         .collect::<Vec<Vec<char>>>();

//     let mut expanded_map: Vec<Vec<char>> = Vec::new();

//     // map init
//     for _ in 0..row_expanded_map.len() {
//         expanded_map.push(Vec::new());
//     }
//     // expend columns
//     for j in 0..(row_expanded_map[0].len()) {
//         let col = row_expanded_map.iter().map(|r| r[j]).collect::<Vec<char>>();

//         let is_empty = col.iter().all(|c| c == &'.');

//         for i in 0..row_expanded_map.len() {
//             expanded_map[i].push(col[i]);
//             if is_empty {
//                 expanded_map[i].push('.');
//             }
//         }
//     }

//     let mut galaxies = Vec::new();
//     // records positions of '#'
//     for (i, row) in expanded_map.iter().enumerate() {
//         for (j, c) in row.iter().enumerate() {
//             if c == &'#' {
//                 galaxies.push((i as i32, j as i32));
//             }
//         }
//     }

//     let mut galaxy_pairs: Vec<((i32, i32), (i32, i32))> = Vec::new();
//     for i in 0..galaxies.len() {
//         for j in (i+1)..galaxies.len() {
//             galaxy_pairs.push((galaxies[i], galaxies[j]));
//         }
//     }

//     galaxy_pairs.into_iter().map(|(g1, g2)| -> i32 {
//         (g1.0-g2.0).abs() + (g1.1-g2.1).abs()
//     }).sum()
// }
