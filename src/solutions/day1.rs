fn find_first_digit(_input: &str) -> u32 {
    let letters = [("one", 1), ("two", 2), ("three", 3), ("four", 4), ("five", 5), ("six", 6), ("seven", 7), ("eight", 8), ("nine", 9)];

    for n in 0.._input.len() {
        // check for single character
        let c = _input.chars().nth(n).unwrap();
        if c.is_digit(10) {
            return c.to_digit(10).unwrap();
        }
        // also check for letters
        for (letter, v) in letters {
            if _input.chars().skip(n).collect::<String>().as_str().starts_with(letter) {
                print!("letter = {}\n", letter);
                return v
            }
        }
    }
    return 0;
}

// for part1, we simply just convert each line into first digit and last digit first, and sum them
#[allow(dead_code)]
pub fn part1(_input: &str) -> String {
    let result: u32 = _input
        .split("\n") // Split by newline character
        .map(|line| -> u32 {
            // extract first and last digit from each line
            let first = find_first_digit(line);
            let last = find_first_digit(&line.chars().rev().collect::<String>());

            return first * 10 + last;
        }) // extract first and last digit from each line
        .sum(); //
    return result.to_string();
}

fn process_line(_input: &str) -> String {
    let mut result: Vec<String> = Vec::new();

    let letters = [("one", "1"), ("two", "2"), ("three", "3"), ("four", "4"), ("five", "5"), ("six", "6"), ("seven", "7"), ("eight", "8"), ("nine", "9")];

    for n in 0.._input.len() {
        // check for single character
        let c = _input.chars().nth(n).unwrap();
        if c.is_digit(10) {
            result.push(c.to_string())
        }
        // also check for letters
        for (letter, v) in letters {
            if _input.chars().skip(n).collect::<String>().as_str().starts_with(letter) {
                result.push(v.to_string());
            }
        }
    }

    return result.join("");
}

// for part2, the idea is the same, the only extra work is to recognize the letter digits,
// so for eaiser to understand, I want to rewrite each line to its proper form and then use part1 to solve
#[allow(dead_code)]
pub fn part2(_input: &str) -> String {
    let result: u32 = _input
        .split("\n") 
        .map(process_line) // this is the new change
        .map(|line| -> u32 {
            // because process_line() convert line to String instead of &str, need & to reference.
            let first = find_first_digit(&line);
            let last = find_first_digit(&line.chars().rev().collect::<String>());

            return first * 10 + last;
        }) // extract first and last digit from each line
        .sum(); //

    return result.to_string();
}
