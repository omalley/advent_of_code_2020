use std::io;
use std::io::BufRead;

#[derive(Debug)]
struct Range {
  lower: i64,
  upper: i64,
}

impl Range {
  fn parse(line: &str) -> Self {
    let nums: Vec<i64> = line.split("-")
      .map(|x| x.trim().parse::<i64>().unwrap()).collect();
    Range{lower: nums[0], upper: nums[1]}
  }

  fn is_valid(&self, num: i64) -> bool {
    self.lower <= num && num <= self.upper
  }
}

#[derive(Debug)]
struct Attribute {
  name: String,
  ranges: Vec<Range>,
}

impl Attribute {
  fn parse(line: &str) -> Self {
    let mut parts = line.split(":");
    let name = parts.next().unwrap().trim().to_string();
    let ranges = parts.next().unwrap().split("or")
      .map(|x| Range::parse(x.trim())).collect();
    Attribute{name, ranges}
  }

  fn is_valid(&self, num: i64) -> bool {
    for r in &self.ranges {
      if r.is_valid(num) {
        return true;
      }
    }
    false
  }
}

#[derive(Debug)]
struct Ticket {
  values: Vec<i64>,
}

impl Ticket {
  fn parse(line: &str) -> Self {
    let values = line.split(",").map(|x| x.parse::<i64>().unwrap()).collect();
    Ticket{values}
  }
}

#[derive(Debug)]
struct Input {
  attributes: Vec<Attribute>,
  your: Ticket,
  nearby: Vec<Ticket>,
}

impl Input {
  fn parse(input: &mut dyn Iterator<Item=String>) -> Self {
    let mut attributes = Vec::new();
    while let Some(line) = input.next() {
      if line == "your ticket:" {
        break;
      }
      attributes.push(Attribute::parse(&line));
    }
    let your = Ticket::parse(input.next().unwrap().as_str());
    if input.next().unwrap() != "nearby tickets:" {
      panic!("missing header line");
    }
    let nearby = input.map(|x| Ticket::parse(&x)).collect();
    Input{attributes, your, nearby}
  }

  fn is_valid(&self, val: i64) -> bool {
    self.attributes.iter().flat_map(|a| a.ranges.iter()).find(|r| r.is_valid(val)).is_some()
  }
}

fn main() {
  let stdin = io::stdin();
  let input= Input::parse(&mut stdin.lock().lines()
    .map(|x| x.unwrap().trim().to_string())
    .filter(|x| !x.is_empty()));
  let bad_values: Vec<i64> = input.nearby.iter().flat_map(|t| t.values.iter())
    .map(|v| *v).filter(|v| !input.is_valid(*v)).collect();
  println!("sum = {}", bad_values.iter().fold(0, |acc, v| acc + v));
}
