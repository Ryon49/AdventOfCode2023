// The struct represents the location of a Part in the input
const DIRS: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

#[derive(Debug)]
struct Part {
    row: i32,
    begin: i32,
    end: i32,

    value: u32,
}

fn parse_parts(_input: &str) -> Vec<Part> {
    let mut parts: Vec<Part> = Vec::new();

    for (row, line) in _input.split("\n").enumerate() {
        let chars: Vec<char> = line.chars().collect();

        let mut i = 0;
        while i < line.len() {
            if chars[i].is_digit(10) {
                let (begin, mut end): (usize, usize) = (i, i + 1);
                while end < line.len() && chars[end].is_digit(10) {
                    end += 1;
                }

                parts.push(Part {
                    row: row as i32,
                    begin: begin as i32,
                    end: (end - 1) as i32,
                    value: String::from_iter(&chars[begin..end])
                        .parse::<u32>()
                        .unwrap(),
                });
                i = end;
            }
            i += 1;
        }
    }

    return parts;
}

fn check_valid_part(part: &Part, symbols: &Vec<(i32, i32)>) -> bool {
    for (i, j) in symbols {
        for (offset_i, offset_j) in DIRS {
            let (next_i, next_j) = (i + offset_i, j + offset_j);

            if part.row == next_i && next_j >= part.begin && next_j <= part.end {
                return true;
            }
        }
    }
    return false;
}

#[allow(dead_code)]
pub fn part1(_input: &str) -> u32 {
    let parts: Vec<Part> = parse_parts(_input);

    let mut symbols: Vec<(i32, i32)> = Vec::new();
    for (i, line) in _input.split("\n").enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c != '.' && !c.is_digit(10) {
                symbols.push((i as i32, j as i32));
            }
        }
    }

    parts
        .iter()
        .filter(|part| check_valid_part(part, &symbols))
        .map(|part| part.value)
        .sum()
}

#[allow(dead_code)]
pub fn part2(_input: &str) -> u32 {
    let parts: Vec<Part> = parse_parts(_input);
    let mut gears: Vec<(i32, i32)> = Vec::new(); // coordinates of '*'
    for (i, line) in _input.split("\n").enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c == '*' {
                gears.push((i as i32, j as i32));
            }
        }
    }

    gears
        .iter()
        .map(|gear| -> Vec<u32> {
            // wrap gear in a Vec
            let cog = vec![(gear.0, gear.1)];

            // this returns all adjacent parts near '*'
            parts
                .iter()
                .filter(|part| check_valid_part(part, &cog))
                .map(|part| part.value)
                .collect::<Vec<u32>>()
        })
        .filter(|parts| parts.len() == 2) // only only pair of 2
        .map(|parts| parts[0] * parts[1])
        .sum()
}
