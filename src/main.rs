use std::io;
use std::io::BufRead;

use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug)]
enum Operation {
  Mask{clear: u64, set: u64},
  Set{address: u64, value: u64},
}

impl Operation {
  fn parse(line: &str) -> Option<Operation> {
    lazy_static! {
      static ref MASK: Regex = Regex::new(r"^mask = (?P<mask>[X01]{36})$").unwrap();
      static ref SET: Regex = Regex::new(r"^mem\[(?P<addr>\d+)] = (?P<value>\d+)$").unwrap();
    }
    if let Some(mask_match) = MASK.captures(line) {
      let mut clear: u64 = 0;
      let mut set: u64 = 0;
      let mut mask: u64 = 1;
      for ch in mask_match.name("mask").unwrap().as_str().chars().rev() {
        match ch {
          '0' => clear |= mask,
          '1' => set |= mask,
          _ => {},
        }
        mask <<= 1;
      }
      return Some(Operation::Mask{clear, set});
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
  clear_mask: u64,
  set_mask: u64,
  memory: Vec<u64>,
}

impl State {
  fn new() -> Self {
    State{clear_mask: 0, set_mask: 0, memory: Vec::new()}
  }

  fn execute(&mut self, operation: &Operation) {
    match operation {
      Operation::Mask{set, clear} => {
        self.set_mask = *set;
        self.clear_mask = *clear;
      }
      Operation::Set{address, value} => {
        let value = (value | self.set_mask) & !self.clear_mask;
        self.set(*address, value);
      }
    }
  }

  fn set(&mut self, address: u64, value: u64) {
    while self.memory.len() <= address as usize {
      self.memory.push(0);
    }
    self.memory[address as usize] = value;
  }

  fn sum(&self) -> u64 {
    self.memory.iter().fold(0, |acc, v| acc + v)
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
