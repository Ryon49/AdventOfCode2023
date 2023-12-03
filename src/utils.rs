use std::fs;

pub fn read_input(file_name: &str) -> String {
    let path = format!("data/{}", file_name);
    return fs::read_to_string(path).unwrap()
}
