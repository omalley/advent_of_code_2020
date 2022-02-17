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
  for i in WINDOW_SIZE..input.len() {
    if !checksum(input[i], &input[i-WINDOW_SIZE..i]) {
      println!("unmatched {} = {}", i, input[i]);
    }
  }
}
