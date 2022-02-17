use std::io;
use std::io::BufRead;

#[derive(Clone, Debug)]
enum Instruction {
  Acc(i64),
  Jmp(i64),
  Nop(i64),
}

impl Instruction {
  fn parse(line: &str) -> Option<Self> {
    let words : Vec<&str> = line.split_ascii_whitespace().collect();
    let arg = words[1].parse::<i64>().unwrap();
    match words[0] {
      "nop" => Some(Self::Nop(arg)),
      "acc" => Some(Self::Acc(arg)),
      "jmp" => Some(Self::Jmp(arg)),
      _ => None,
    }
  }
}

fn run_program(pgm: &Vec<Instruction>) -> Option<i64> {
  let mut done = vec![false; pgm.len()];
  let mut acc: i64 = 0;
  let mut pc: usize = 0;
  while pc < pgm.len() && !done[pc] {
    println!("pc = {}, acc = {}", pc, acc);
    done[pc] = true;
    match pgm[pc] {
      Instruction::Nop(val) => {pc += 1},
      Instruction::Acc(val) => {acc += val; pc += 1},
      Instruction::Jmp(val) => pc = (pc as i64 + val) as usize,
    }
  }
  if pc == pgm.len() {
    Some(acc)
  } else {
    None
  }
}

fn modify(original: &Vec<Instruction>, location: usize, inst: &Instruction) -> Vec<Instruction> {
  let mut result = original.clone();
  result[location] = inst.clone();
  result
}

fn main() {
  let stdin = io::stdin();
  let program: Vec<Instruction> = stdin.lock().lines()
    .map(|s| Instruction::parse(&s.unwrap()).unwrap())
    .collect();
  for ins in 0..program.len() {
    let result;
    match program[ins] {
      Instruction::Nop(val) => result = run_program(&modify(&program, ins,
                                                   &Instruction::Jmp(val))),
      Instruction::Jmp(val) => result = run_program(&modify(&program, ins,
                                                   &Instruction::Nop(val))),
      _ => result = None,
    }
    if let Some(acc) = result {
      println!("instruction {}: acc = {}", ins, acc);
      break;
    }
  }
}
