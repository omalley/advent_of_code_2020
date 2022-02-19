use std::io;
use std::io::BufRead;

fn main() {
  let stdin = io::stdin();
  let lines: Vec<String> = stdin.lock().lines()
    .map(|s| s.unwrap().trim().to_string()).collect();
  let arrival = lines[0].parse::<u64>().unwrap();
  let buses: Vec<u64> = lines[1].split(",").filter(|x| *x != "x")
    .map(|x| x.parse::<u64>().unwrap()).collect();
  let waits: Vec<u64> = buses.iter().map(|b| b - (arrival % b)).collect();
  let best = waits.iter().position(|x| x == waits.iter().min().unwrap()).unwrap();
  println!("best = {}, wait = {}, prod = {}", buses[best], waits[best], buses[best] * waits[best]);
}
