use std::io;
use std::io::BufRead;

#[derive(Debug)]
enum Instruction {
  Acc(i64),
  Jmp(i64),
  Nop,
}

impl Instruction {
  fn parse(line: &str) -> Option<Self> {
    let words : Vec<&str> = line.split_ascii_whitespace().collect();
    let arg = words[1].parse::<i64>().unwrap();
    match words[0] {
      "nop" => Some(Self::Nop),
      "acc" => Some(Self::Acc(arg)),
      "jmp" => Some(Self::Jmp(arg)),
      _ => None,
    }
  }
}

fn run_program(pgm: &Vec<Instruction>) -> i64 {
  let mut done = vec![false; pgm.len()];
  let mut acc: i64 = 0;
  let mut pc: usize = 0;
  while !done[pc] {
    println!("pc = {}, acc = {}", pc, acc);
    done[pc] = true;
    match pgm[pc] {
      Instruction::Nop => {pc += 1},
      Instruction::Acc(val) => {acc += val; pc += 1},
      Instruction::Jmp(val) => pc = (pc as i64 + val) as usize,
    }
  }
  acc
}
fn main() {
  let stdin = io::stdin();
  let program: Vec<Instruction> = stdin.lock().lines()
    .map(|s| Instruction::parse(&s.unwrap()).unwrap())
    .collect();
  println!("count = {}", run_program(&program));
}
