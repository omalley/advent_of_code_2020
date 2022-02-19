use std::collections::HashMap;
use std::io;
use std::io::BufRead;

use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug)]
enum Operation {
  // set is a mask of bits to always set to 1
  // float is a list of positions to float
  Mask { set: u64, float: Vec<usize> },
  Set{address: u64, value: u64},
}

impl Operation {
  fn parse(line: &str) -> Option<Operation> {
    lazy_static! {
      static ref MASK: Regex = Regex::new(r"^mask = (?P<mask>[X01]{36})$").unwrap();
      static ref SET: Regex = Regex::new(r"^mem\[(?P<addr>\d+)] = (?P<value>\d+)$").unwrap();
    }
    if let Some(mask_match) = MASK.captures(line) {
      let mut set: u64 = 0;
      let mut float: Vec<usize> = Vec::new();
      for (posn, ch) in mask_match.name("mask").unwrap().as_str().chars().enumerate() {
        match ch {
          '1' => set |= 1 << (35 - posn),
          'X' => float.push(35 - posn),
          _ => {},
        }
      }
      return Some(Operation::Mask { set, float });
    }
    if let Some(set_match) = SET.captures(line) {
      let address = set_match.name("addr").unwrap().as_str().parse::<u64>().unwrap();
      let value = set_match.name("value").unwrap().as_str().parse::<u64>().unwrap();
      return Some(Operation::Set{address, value});
    }
    None
  }
}

struct State {
  set_mask: u64,
  float_mask: Vec<usize>,
  memory: HashMap<u64, u64>,
}

impl State {
  fn new() -> Self {
    State{set_mask: 0, float_mask: Vec::new(), memory: HashMap::new()}
  }

  fn execute(&mut self, operation: &Operation) {
    match operation {
      Operation::Mask{set, float} => {
        self.set_mask = *set;
        self.float_mask = float.clone();
      }
      Operation::Set{address, value} => {
        self.set(*address, *value);
      }
    }
  }

  fn generate_addresses(address: u64, floats: &[usize]) -> Vec<u64> {
    if floats.is_empty() {
      vec![address]
    } else {
      let tail = Self::generate_addresses(address, &floats[1..]);
      let head = floats[0];
      let mut result: Vec<u64> = tail.iter().map(|x| x & !(1 << head)).collect();
      for val in tail {
        result.push(val | (1 << head));
      }
      result
    }
  }

  fn set(&mut self, address: u64, value: u64) {
    let address = address | self.set_mask;
    for add in Self::generate_addresses(address, &self.float_mask) {
      self.memory.insert(add, value);
    }
  }

  fn sum(&self) -> u64 {
    self.memory.iter().fold(0, |acc, (_, v)| acc + v)
  }
}

fn main() {
  let stdin = io::stdin();
  let operations: Vec<Operation> = stdin.lock().lines()
    .map(|s| Operation::parse(s.unwrap().trim()).unwrap()).collect();
  let mut state = State::new();
  for o in operations {
    println!("execute {:?}", o);
    state.execute(&o);
  }
  println!("sum = {}", state.sum());
}
