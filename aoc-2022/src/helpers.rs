use std::fs;

pub fn read_input(day: i8) -> String {
    let contents = fs::read_to_string(format!("./input/day_{}", day)).expect("Should have been able to read the file");
    return contents
}
