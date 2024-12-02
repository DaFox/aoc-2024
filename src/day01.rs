use crate::aoc::read_input_pairs;

pub fn solution_1() -> String {
    let (mut a, mut b): (Vec<usize>, Vec<usize>) = read_input_pairs(1, 1).iter().cloned().unzip();

    a.sort();
    b.sort();

    a.iter()
     .zip(b.iter())
     .fold(0, |acc, (a, b)| acc + if a > b { a - b } else { b - a })
     .to_string()
}

pub fn solution_2() -> String {
    let (a, b): (Vec<usize>, Vec<usize>) = read_input_pairs(1, 1).iter().cloned().unzip();

    a.iter()
     .fold(0, |acc, val_a| acc + val_a * b.iter().filter(|val_b| *val_b == val_a).count())
     .to_string()
}
