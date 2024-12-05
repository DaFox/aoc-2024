use std::cmp::Ordering;

use crate::aoc::read_input;

type Rule = (usize, usize);

fn remove_unused_rules(rules:&Vec<Rule>, pages: &Vec<usize>) -> Vec<Rule> {
  rules
    .iter()
    .filter(|(a, b)| pages.contains(a) && pages.contains(b))
    .copied()
    .collect()
}

fn check_rule(rule: &Rule, pages: &Vec<usize>) -> bool {
  let i = pages.iter().enumerate().find(|(_, page)| **page == rule.0).unwrap().0;
  let j = pages.iter().enumerate().find(|(_, page)| **page == rule.1).unwrap().0;

  i < j
}


pub fn solution_1() -> String {  
  let input = read_input(5, 1);

  let sort_rules:Vec<Rule> = input.iter()
    .take_while(|line| !line.is_empty())
    .map(|line| line.split_once('|').unwrap())
    .map(|(a,b)| (str::parse::<usize>(a).unwrap(), str::parse::<usize>(b).unwrap()))
    .collect();

  input.iter()
    .skip_while(|line| !line.is_empty())
    .skip(1)
    .map(|line| line.split(',').map(|n| str::parse::<usize>(n).unwrap()).collect())
    .filter(|pages| remove_unused_rules(&sort_rules, pages).iter().all(|rule| check_rule(rule, pages)))
    .map(|pages| *pages.get(pages.len() / 2).unwrap())
    .sum::<usize>()
    .to_string()
}

pub fn solution_2() -> String {
  let input = read_input(5, 1);

  let sort_rules:Vec<Rule> = input.iter()
    .take_while(|line| !line.is_empty())
    .map(|line| line.split_once('|').unwrap())
    .map(|(a,b)| (str::parse::<usize>(a).unwrap(), str::parse::<usize>(b).unwrap()))
    .collect();

   let man_pages: Vec<Vec<usize>> = input.iter()
    .skip_while(|line| !line.is_empty())
    .skip(1)
    .map(|line| line.split(',').map(|n| str::parse::<usize>(n).unwrap()).collect())
    .filter(|pages| !remove_unused_rules(&sort_rules, pages).iter().all(|rule| check_rule(rule, pages)))
    .collect();

  let mut total = 0;

  for mut pages in man_pages {
    let rules = remove_unused_rules(&sort_rules, &pages);

    pages.sort_by(|a, b| {
      let rule = rules.iter().find(|r| r.0 == *a && r.1 == *b);

      if rule.is_some() {
        return Ordering::Less;
      }

      let rule = rules.iter().find(|r| r.1 == *a && r.0 == *b);

      if rule.is_some() {
        return Ordering::Greater;
      }

      Ordering::Equal
    });

    total += pages.get(pages.len() / 2).unwrap();
  }

  total.to_string()
}
