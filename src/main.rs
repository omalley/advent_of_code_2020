use std::collections::HashMap;
use std::io;
use std::io::BufRead;

const MAX_TURN: usize = 30000000;

fn main() {
  let stdin = io::stdin();
  let line: String = stdin.lock().lines().next().unwrap().unwrap();
  let nums: Vec<i64> = line.split(",").map(|x| x.parse::<i64>().unwrap()).collect();
  let mut state: HashMap<i64, usize> = HashMap::new();
  let mut current: i64 = 0;
  for turn in 0..MAX_TURN {
    let prev_num = current;
    if turn < nums.len() {
      current = nums[turn];
    } else {
      if let Some(prev) = state.get(&current) {
        current = (turn - *prev) as i64;
      } else {
        current = 0;
      }
    }
    if turn > 0 {
      state.insert(prev_num, turn);
    }
  }
  println!("last = {}", current);
}
