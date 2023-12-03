mod solutions;
mod utils;

use std::fmt::Display;

use utils::*;

struct Question {
    day: u32,
}

impl Question {
    fn init(_day: u32) -> Self {
        return Question { day: _day };
    }
    #[allow(dead_code)]
    pub fn example(&self) -> String {
        let file_name = format!("day{}/example.txt", self.day);
        return read_input(&file_name);
    }
    #[allow(dead_code)]
    pub fn part1(&self) -> String {
        let file_name = format!("day{}/part1.txt", self.day);
        return read_input(&file_name);
    }
}

fn runner<T: Display>(solver: &dyn Fn(&str) -> T, input: &str) -> T {
    solver(input)
}

fn main() {
    let question = Question::init(1);

    let input = question.part1();

    let result = runner(&solutions::day1::part2, &input);

    print!("result = {}\n", result)
}
