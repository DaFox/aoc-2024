use crate::aoc::read_input;

fn is_safe(report: Vec<usize>) -> bool {
    let diff:Vec<isize> = report
      .windows(2)
      .map(|pair| pair[1] as isize - pair[0] as isize)
      .collect();

    diff.iter().all(|x| *x > 0 && *x <= 3) || diff.iter().all(|x| *x < 0 && *x >= -3)
}

fn without_nth<T: Copy>(slice: &[T], n: usize) -> Vec<T> {
  slice
    .iter()
    .take(n)
    .chain(slice.iter().skip(n + 1))
    .copied()
    .collect()
}

pub fn solution_1() -> String {
    let reports = read_input(2, 1)
      .iter()
      .map(|line| line.split(' ').map(|n| str::parse::<usize>(n).unwrap()).collect::<Vec<_>>())
      .filter(|report| is_safe(report.clone()))
      .count();

    reports.to_string()
}


pub fn solution_2() -> String {
    let reports = read_input(2, 1)
      .iter()
      .map(|line| line.split(' ').map(|n| str::parse::<usize>(n).unwrap()).collect::<Vec<_>>())
      .filter(|report| (0..=report.len()).any(|i| is_safe(without_nth(&report, i))))
      .count();

    reports.to_string()
}
