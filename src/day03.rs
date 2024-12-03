use crate::aoc::read_input;
use regex::Regex;

fn match_mul(input: &str) -> String {
  let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

  re.captures_iter(input)
    .map(|c| str::parse::<u32>(&c[1]).unwrap() * str::parse::<u32>(&c[2]).unwrap())
    .sum::<u32>()
    .to_string()
}

pub fn solution_1() -> String {
  match_mul(read_input(3, 1).concat().as_str())
}

pub fn solution_2() -> String {
  let memory = read_input(3, 2).concat().to_string();
  let repl = Regex::new(r"don't\(\).*?(do\(\)|$)").unwrap();
  
  match_mul(repl.replace_all(&memory, "").to_string().as_str())
}
