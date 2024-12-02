use std::path::PathBuf;

// Global AOC settings go here
const INPUT_DIR:&'static str = "input";

#[cfg(test)]
const PREFIX:&'static str = "test";

#[cfg(not(test))]
const PREFIX:&'static str = "data";

pub fn read_input(day: u8, part: u8) -> Vec<String> {
    let mut path = PathBuf::from(INPUT_DIR);
    path.push(format!("{}.{:0>2}.{}.txt", PREFIX, day, part));

    std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|line| line.to_string())
        .collect()
}


pub fn read_input_pairs(day: u8, part: u8) -> Vec<(usize, usize)> {
    read_input(day, part)
        .iter()
        .map(|line| line.split_once("   ").unwrap())
        .map(|(a, b)| (str::parse::<usize>(a).unwrap(), str::parse::<usize>(b).unwrap()))
        .collect()
}
