use std::io;
use std::io::BufRead;

use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug)]
struct Rule {
  lower: usize,
  upper: usize,
  goal: char,
  password: String,
}

impl Rule {
  fn parse(line: &str) -> Option<Self> {
    lazy_static! {
        static ref PASSWORD_PATTERN: Regex = Regex::new(
            r"^(?P<lower>\d+)-(?P<upper>\d+) (?P<char>.):\s*(?P<pass>\w+)$").unwrap();
    }
    let cap = PASSWORD_PATTERN.captures(line)?;
    let lower = cap.name("lower")?.as_str().parse::<usize>().unwrap();
    let upper = cap.name("upper")?.as_str().parse::<usize>().unwrap();
    let goal = cap.name("char")?.as_str().chars().next()?;
    let password = String::from(cap.name("pass")?.as_str());
    Some(Rule{lower, upper, goal, password})
  }

  fn compliant(&self) -> bool {
    let chars: Vec<char> = self.password.chars().collect();
    (chars[self.lower - 1] == self.goal) != (chars[self.upper - 1] == self.goal)
  }
}

fn main() {
  let stdin = io::stdin();
  let rules: Vec<Rule> = stdin.lock().lines()
    .map(|x| Rule::parse(x.unwrap().trim()).unwrap())
    .filter(|r| r.compliant())
    .collect();
  println!("{}", rules.len());
}
