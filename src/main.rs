use std::io;
use std::io::BufRead;

const WINDOW_SIZE: usize = 25;

fn checksum(num: i64, prev: &[i64]) -> bool {
  for p0 in prev {
    for p1 in prev {
      if p0 != p1 && p0 + p1 == num {
        return true;
      }
    }
  }
  false
}

fn main() {
  let stdin = io::stdin();
  let input: Vec<i64> = stdin.lock().lines()
    .map(|s| s.unwrap().parse::<i64>().unwrap())
    .collect();
  let mut unmatched = 0;
  for i in WINDOW_SIZE..input.len() {
    if !checksum(input[i], &input[i-WINDOW_SIZE..i]) {
      unmatched = input[i];
      println!("unmatched {} = {}", i, unmatched);
    }
  }
  for i0 in 0..input.len()-1 {
    for i1 in i0+1..input.len() {
      if input[i0..=i1].iter().fold(0, |a, b| a + b) == unmatched {
        let min = input[i0..=i1].iter().min().unwrap();
        let max = input[i0..=i1].iter().max().unwrap();
        println!("{} .. {} = {} min + max = {}", input[i0], input[i1], unmatched, min + max);
      }
    }
  }
}
