use std::io;
use std::io::BufRead;

#[derive(Clone,Debug,Eq,PartialEq)]
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

#[derive(Debug,Eq,PartialEq)]
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

  fn valid_value(ranges: &Vec<&Range>, val: i64) -> bool {
    for r in ranges {
      if r.is_valid(val) {
        return true
      }
    }
    false
  }

  fn valid_tickets(&self) -> Vec<&Ticket> {
    let ranges: Vec<&Range> = self.attributes.iter()
      .flat_map(|a| a.ranges.iter()).collect();
    let mut result = Vec::new();
    for ticket in &self.nearby {
      if ticket.values.iter().filter(|&&v| !Self::valid_value(&ranges, v)).count() == 0 {
        result.push(ticket);
      }
    }
    result
  }

  // find the set of attributes that could be at each column
  fn find_attributes(&self) -> Vec<Vec<&Attribute>> {
    let mut result = vec![Vec::new(); self.your.values.len()];
    let tickets = self.valid_tickets();
    for (i, &val) in tickets[0].values.iter().enumerate() {
      for a in self.attributes.iter().filter(|a| a.is_valid(val)) {
        result[i].push(a);
      }
      for &t in tickets[1..].iter() {
        for (i, &val) in t.values.iter().enumerate() {
          result[i].retain(|a| a.is_valid(val));
        }
      }
    }
    result
  }

  // compute the final list of attributes in the right order
  fn attributes(&self) -> Vec<&Attribute> {
    let mut result = vec![None; self.your.values.len()];
    let mut possibilitiies = self.find_attributes();
    while result.iter().find(|x| x.is_none()).is_some() {
      for i in 0..possibilitiies.len() {
        if result[i].is_none() && possibilitiies[i].len() == 1 {
          result[i] = Some(possibilitiies[i][0]);
          // remove it from consideration for other places
          for j in 0..possibilitiies.len() {
            let loc = possibilitiies[j].iter()
              .position(|&x| *x == *result[i].unwrap());
            if let Some(loc) = loc {
              possibilitiies[j].remove(loc);
            }
          }
        }
      }
    }
    result.iter().map(|x| x.unwrap()).collect()
  }
}

fn main() {
  let stdin = io::stdin();
  let input= Input::parse(&mut stdin.lock().lines()
    .map(|x| x.unwrap().trim().to_string())
    .filter(|x| !x.is_empty()));
  let mapping: Vec<&Attribute> = input.attributes();
  let depart = input.your.values.iter().enumerate()
    .filter(|(i, _)| mapping[*i].name.starts_with("departure "))
    .map(|(_, &v)| v).fold(1, |acc, v| acc * v);
  println!("depart = {}", depart);
}
