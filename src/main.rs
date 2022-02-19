use std::io;
use std::io::BufRead;

use itertools::Itertools;
use num::integer::lcm;

fn main() {
  let stdin = io::stdin();
  let lines: Vec<String> = stdin.lock().lines()
    .map(|s| s.unwrap().trim().to_string()).collect();
  let buses: Vec<(u64,u64)> = lines[1].split(",").enumerate()
    .filter(|(_,x)| *x != "x")
    .map(|(p, x) | (p as u64, x.parse::<u64>().unwrap())).collect();
  println!("buses = {:?}", buses);
  // since a bus of period P that needs offset O can work with O % P
  let mut periods: Vec<(u64, u64)> = buses.iter().map(|(o, p) | (o % p, *p)).collect();
  // group the buses by offset, to find the offset with the most buses
  periods.sort_by(|(l, _), (r, _)| l.cmp(r));
  let mut best_offset: u64 = 0;
  let mut best_periods: Vec<u64> = Vec::new();
  for (offset, group) in &periods.into_iter().group_by(|(o, _)| o.clone()) {
    let group_list: Vec<u64> = group.map(|(_, p)| p).collect();
    if group_list.len() > best_periods.len() {
      best_offset = offset;
      best_periods = group_list;
    }
  }
  let period = best_periods.iter().fold(1, |acc, &p| lcm::<u64>(acc, p));
  println!("best offset = {}, periods = {:?}, period = {}", best_offset, best_periods, period);
  'outer: for t in (period - best_offset..u64::MAX).step_by(period as usize) {
    for b in &buses {
      if (t + b.0) % b.1 != 0 {
        continue 'outer;
      }
    }
    println!("time = {}", t);
    break;
  }
}
