use crate::aoc::read_input;

fn transpose(matrix: &Vec<String>) -> Vec<String> {
  let char_vec:Vec<Vec<char>> = matrix.into_iter().map(|line| line.chars().collect()).collect();
  let mut target = vec![String::new(); char_vec[0].len()];

  for i in 0..char_vec.len() {
    for k in 0..char_vec[i].len() {
      target[k].push(char_vec[i][k]);
    }
  }

  target
}

fn reverse(matrix: &Vec<String>) -> Vec<String> {
  matrix
    .iter()
    .map(|i| i.chars().rev().collect())
    .collect()
}

fn diagonals(matrix: &Vec<String>) -> Vec<String> {
  let diag_rows = matrix.len() + matrix[0].len() - 1;
  let char_vec:Vec<Vec<char>> = matrix.into_iter().map(|line| line.chars().collect()).collect();
  let mut target = vec![String::new(); diag_rows];

  for i in 0..diag_rows {
    let mut k = 0;
    let mut j = i;

    if j >= char_vec.len() {
      k = j - (char_vec.len() - 1);
      j = char_vec.len() - 1;
    }

    loop {
      target[i].push(char_vec[j][k]);

      if j == 0 || k == char_vec[j].len() - 1 {
        break;
      }     
  
      j = j - 1;
      k = k + 1;
    }
  }

  target
}

pub fn solution_1() -> String {  
  let input = read_input(4, 1);

  input.iter()
    .chain(&transpose(&input))
    .chain(&diagonals(&input))
    .chain(&diagonals(&reverse(&input)))
    .map(|line| line.matches("XMAS").count() + line.matches("SAMX").count())
    .sum::<usize>()
    .to_string()
}

pub fn solution_2() -> String {
  let input = read_input(4, 1);
  let mut count = 0;

  // We just have to scan all rows >1 and <len()-2 for an A. Then check the surounding fields.
  let char_vec:Vec<Vec<char>> = input.into_iter().map(|line| line.chars().collect()).collect();

  for i in 1..(char_vec.len() - 1) {
    for k in 1..(char_vec[i].len() - 1) {
      if char_vec[i][k] == 'A' {
        // Check the suroundings
        if (char_vec[i - 1][k - 1] == 'M' && char_vec[i + 1][k + 1] == 'S' || char_vec[i - 1][k - 1] == 'S' && char_vec[i + 1][k + 1] == 'M') 
          && (char_vec[i - 1][k + 1] == 'S' && char_vec[i + 1][k - 1] == 'M' || char_vec[i - 1][k + 1] == 'M' && char_vec[i + 1][k - 1] == 'S') {
          count += 1;
        }
      }
    }
  }
  
  count.to_string()
}
