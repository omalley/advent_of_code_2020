use std::io;
use std::io::BufRead;

fn iterate(subject: i64, x: i64) -> i64 {
  (subject * x) % 20201227
}

fn find_iteration(public_key: i64) -> usize {
  let mut value = 1;
  let mut iterations = 0;
  while value != public_key {
    value = iterate(7, value);
    iterations += 1;
  }
  iterations
}

fn compute_key(subject: i64, iterations: usize) -> i64 {
  let mut value = 1;
  for _ in 0..iterations {
    value = iterate(subject, value);
  }
  value
}

fn main() {
  let stdin = io::stdin();
  let keys: Vec<i64> = stdin.lock().lines()
    .map(|l| l.unwrap().trim().parse::<i64>().unwrap())
      .collect();
  let iterations: Vec<usize> = keys.iter()
    .map(|&k| find_iteration(k)).collect();
  println!("iterations = {:?}", iterations);
  println!("shared key = {}", compute_key(keys[0], iterations[1]));
}
