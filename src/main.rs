use std::cmp::Ordering;
use std::io;
use std::io::BufRead;

const TARGET_TOTAL: i64 = 2020;

fn find_pair(nums: &[i64]) -> Option<(usize, usize)> {
  let mut upper = nums.iter().position(|&n| n >= TARGET_TOTAL / 2)?;
  let mut lower: i64 = upper as i64 - 1;
  while lower >= 0 && upper < nums.len() {
    match (nums[lower as usize] + nums[upper]).cmp(&TARGET_TOTAL) {
      Ordering::Less => upper += 1,
      Ordering::Equal => return Some((lower as usize, upper)),
      Ordering::Greater => lower -= 1,
    }
  }
  None
}

fn main() {
  let stdin = io::stdin();
  let mut nums: Vec<i64> = stdin.lock().lines()
    .map(|x| x.unwrap().trim().parse::<i64>().unwrap())
    .collect();
  nums.sort();
  match find_pair(&nums) {
    Some((l,r)) => println!("{} * {} -> {}", nums[l], nums[r], nums[l] * nums[r]),
    None => println!("No pairs"),
  }
}
