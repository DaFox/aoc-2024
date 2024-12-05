mod aoc;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;

use std::env;
use std::collections::HashMap;

type SolutionFn = dyn Fn() -> String;

fn get_day_arg() -> Option<u8> {
    let args: Vec<String> = env::args().collect();

    if args.len() == 3 && args[1] == "--day" {
        if let Ok(day) = args[2].parse::<u8>() {
            if (1..=24).contains(&day) {
                return Some(day);
            }
        }
    }

    None
}

fn run_day(days: &HashMap<u8, (Box<SolutionFn>, Box<SolutionFn>)>, day: u8) {
    println!("Day {:0>2}", day);
    println!();

    if let Some((solution1, solution2)) = days.get(&day) {
        println!("  Solution 1: {}", solution1());
        println!("  Solution 2: {}", solution2());
    } else {
        println!("  Solution for day {} is not implemented yet.", day);
    }

    println!();
}

macro_rules! day {
    ($module:ident, $day:literal) => {
        (
            $day,
            (
                Box::new($module::solution_1) as Box<SolutionFn>,
                Box::new($module::solution_2) as Box<SolutionFn>,
            ),
        )
    };
}

fn setup() -> HashMap<u8, (Box<SolutionFn>, Box<SolutionFn>)> {
    HashMap::from([
        day!(day01, 1),
        day!(day02, 2),
        day!(day03, 3),
        day!(day04, 4),
        day!(day05, 5)
    ])
}

fn main() {
    let all_days = setup();

    if let Some(day) = get_day_arg() {
        run_day(&all_days, day);
    } else {
        for day in 0..all_days.len() {
            run_day(&all_days, day as u8);
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    fn run_test(days: HashMap<u8, (Box<SolutionFn>, Box<SolutionFn>)>, day: u8, expected_1: &str, expected_2: &str) {
        if let Some((solution1, solution2)) = days.get(&day) {
           assert_eq!(expected_1, solution1());
           assert_eq!(expected_2, solution2());
        }
    }

    #[test]
    fn test_day_01() {
        run_test(setup(), 1, "11", "31");
    }

    #[test]
    fn test_day_02() {
        run_test(setup(), 2, "2", "4");
    }

    #[test]
    fn test_day_03() {
        run_test(setup(), 3, "161", "48");
    }

    #[test]
    fn test_day_04() {
        run_test(setup(), 4, "18", "9");
    }

    #[test]
    fn test_day_05() {
        run_test(setup(), 5, "143", "123");
    }
}
