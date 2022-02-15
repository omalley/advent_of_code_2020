use std::cmp::Ordering;
use std::io;
use std::io::BufRead;

const TARGET_TOTAL: i64 = 2020;

fn find_pair(lower: &[i64], mid:i64, upper: &[i64]) -> Option<(usize, usize)> {
  let mut u = 0;
  let mut l= lower.len() - 1;
  while u < upper.len() {
    match (lower[l] + mid + upper[u]).cmp(&TARGET_TOTAL) {
      Ordering::Less => u += 1,
      Ordering::Equal => return Some((l, lower.len() + u + 1)),
      Ordering::Greater =>
        if l == 0 {
          break
        } else {
          l -= 1
        }
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
  for mid in 1..nums.len()-1 {
    match find_pair(&nums[..mid], nums[mid], &nums[mid+1..]) {
      Some((l,r)) => {
        println!("{} * {} * {}-> {}", nums[l], nums[mid], nums[r],  nums[l] * nums[mid] * nums[r]);
        return
      },
      None => {},
    }
  }
  println!("No triples")
}
