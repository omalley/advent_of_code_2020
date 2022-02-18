use std::io;
use std::io::BufRead;

fn main() {
  let stdin = io::stdin();
  let mut input: Vec<i64> = stdin.lock().lines()
    .map(|s| s.unwrap().parse::<i64>().unwrap())
    .collect();
  input.sort();
  input.push(input[input.len() -1] + 3);
  let mut prev = 0;
  let mut cnt: Vec<u64> = vec![0; 4];
  for i in input {
    cnt[(i - prev) as usize] += 1;
    prev = i;
  }
  println!("{} * {} = {}", cnt[1], cnt[3], cnt[1] * cnt[3]);
}
