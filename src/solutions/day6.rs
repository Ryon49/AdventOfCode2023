use regex::Regex;

fn can_win(limit_time: u64, hold_time: u64, distance: u64) -> bool {
    return distance < (limit_time - hold_time) * hold_time;
}

pub fn part1(_input: &str) -> u64 {
    let lines: Vec<&str> = _input.split("\n").collect::<Vec<&str>>();

    let numbers = Regex::new(r"\d+").unwrap();
    let time = numbers
        .captures_iter(lines[0])
        .map(|caps| {
            let Some(m) = caps.get(0) else { panic!() };
            m.as_str().parse::<u64>().unwrap()
        })
        .collect::<Vec<u64>>();

    let distance = numbers
        .captures_iter(lines[1])
        .map(|caps| {
            let Some(m) = caps.get(0) else { panic!() };
            m.as_str().parse::<u64>().unwrap()
        })
        .collect::<Vec<u64>>();

    time.into_iter()
        .zip(distance.into_iter())
        .map(|(t, d)| -> u64 {
            let mut win_count: u64 = 0;
            for i in 1..=t {
                if can_win(t, i, d) {
                    win_count += 1;
                }
            }
            win_count
        })
        .fold(1_u64, |v1, v2| v1 * v2)
}

pub fn part2(_input: &str) -> u64 {
    let lines: Vec<&str> = _input.split("\n").collect::<Vec<&str>>();

    let numbers = Regex::new(r"\d+").unwrap();
    let time = numbers
        .captures_iter(lines[0])
        .map(|caps| {
            let Some(m) = caps.get(0) else { panic!() };
            m.as_str()
        })
        .collect::<Vec<&str>>()
        .join("")
        .parse::<u64>()
        .unwrap();

    let distance = numbers
        .captures_iter(lines[1])
        .map(|caps| {
            let Some(m) = caps.get(0) else { panic!() };
            m.as_str()
        })
        .collect::<Vec<&str>>()
        .join("")
        .parse::<u64>()
        .unwrap();

    let mut win_count: u64 = 0;
    for i in 1..=time {
        if can_win(time, i, distance) {
            win_count += 1;
        }
    }
    win_count
}
