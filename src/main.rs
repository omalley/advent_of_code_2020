use std::collections::HashMap;
use std::io;
use std::io::BufRead;

const MAX_SKIP: i64 = 3;

fn count_arrangements(cur: i64, remaining: &[i64],
                      cache: &mut HashMap<(i64,usize), usize>) -> usize {
  let key = (cur, remaining.len());
  if cache.contains_key(&key) {
    *cache.get(&key).unwrap()
  } else if remaining.is_empty() {
    1
  } else {
    let mut total = 0;
    let mut next = 0;
    while next < remaining.len() && remaining[next] - cur <= MAX_SKIP {
      total += count_arrangements(remaining[next], &remaining[next+1..], cache);
      next += 1;
    }
    cache.insert(key, total);
    total
  }
}

fn main() {
  let stdin = io::stdin();
  let mut input: Vec<i64> = stdin.lock().lines()
    .map(|s| s.unwrap().parse::<i64>().unwrap())
    .collect();
  input.sort();
  let mut cache: HashMap<(i64,usize), usize> = HashMap::new();
  println!("{}", count_arrangements(0, &input, &mut cache));
}
