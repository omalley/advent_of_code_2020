use std::collections::HashMap;
use std::io;
use std::io::BufRead;

use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug)]
struct Bag {
  contains: Vec<BagCount>,
}

#[derive(Debug)]
struct BagCount {
  count: u64,
  bag: String,
}

#[derive(Debug)]
struct Rules {
  rules: HashMap<String,Bag>,
  contained_in: HashMap<String, Vec<String>>,
}

impl Rules {
  fn add_parent(&mut self, parent: &str, child: &str) {
    if !self.contained_in.contains_key(child) {
      self.contained_in.insert(child.to_string(), Vec::new());
    }
    self.contained_in.get_mut(child).unwrap().push(parent.to_string());
  }

  fn parse_bag(&mut self, line: &str) {
    lazy_static! {
      static ref LINE_PATTERN: Regex = Regex::new(
        r"^(?P<name>.+) bags contain (?P<contains>.+)[.]$").unwrap();
      static ref CHILD_PATTERN: Regex = Regex::new(r"^(?P<count>\d+) (?P<bag>.+) bags?$").unwrap();
    }
    let capture = LINE_PATTERN.captures(line).unwrap();
    let name = capture.name("name").unwrap().as_str().to_string();
    let tail = capture.name("contains").unwrap().as_str().to_string();
    let mut contains = Vec::new();
    if tail != "no other bags" {
      for bag in tail.split(", ") {
        let bag_cap = CHILD_PATTERN.captures(bag).unwrap();
        let bag = bag_cap.name("bag").unwrap().as_str().to_string();
        let count = bag_cap.name("count").unwrap().as_str().parse::<u64>().unwrap();
        contains.push(BagCount{count, bag: bag.clone()});
        self.add_parent(&name, &bag);
      }
    }
    self.rules.insert(name,Bag{contains});
  }

  fn parse(input: &mut dyn Iterator<Item=String>) -> Self {
    let mut result = Rules{rules: HashMap::new(), contained_in: HashMap::new()};
    for line in input {
      result.parse_bag(&line);
    }
    result
  }

  fn count(&self, count: u64, color:& str) -> u64 {
    let result = count + self.rules.get(color).unwrap().contains.iter()
      .map(|b| self.count(count * b.count, &b.bag))
      .fold(0, |a,b| a + b);
    println!("{} {} = {}", color, count, result);
    result
  }
}

fn main() {
  let stdin = io::stdin();
  let rules = Rules::parse(&mut stdin.lock().lines()
    .map(|s| s.unwrap()));
  println!("count = {}", rules.count(1, "shiny gold") - 1);
}
