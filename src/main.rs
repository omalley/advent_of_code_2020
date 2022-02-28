use std::collections::HashMap;
use std::io;
use std::io::BufRead;

use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug)]
struct Tile {
  id: i64,
  pixels: Vec<Vec<bool>>,
  width: usize,
}

impl Tile {
  fn parse(input: &mut dyn Iterator<Item=String>) -> Option<Self> {
    lazy_static! {
      static ref HEADER: Regex = Regex::new(r"^Tile\s+(?P<id>\d+):$").unwrap();
    }
    let first_line = input.next()?;
    let id_match = HEADER.captures(&first_line)?;
    let id = id_match.name("id").unwrap().as_str().parse::<i64>().unwrap();
    let mut pixels = Vec::new();
    while let Some(line) = input.next() {
      if line.is_empty() {
        break;
      }
      pixels.push(line.chars().map(|c| c == '#').collect());
    }
    let width = pixels.len();
    Some(Tile{id, pixels, width})
  }

  fn normalize(&self, val: u64) -> u64 {
    let mut result = 0;
    for i in 0..self.width {
      result = (result << 1) | ((val >> i) & 1);
    }
    u64::min(val, result)
  }

  fn convert_row(&self, row: usize) -> u64 {
    self.pixels[row].iter().map(|x| if *x {1} else {0})
      .fold(0, |acc, x| acc << 1 | x)
  }

  fn convert_column(&self, column: usize) -> u64 {
    (0..self.pixels.len()).map(|r| if self.pixels[r][column] {1} else {0})
      .fold(0, |acc, x| acc << 1 | x)
  }

  // Return the four edges as bit vectors
  fn edges(&self) -> [u64; 4] {
    [self.convert_row(0), self.convert_column(0),
      self.convert_row(self.width - 1), self.convert_column(self.width -1)]
  }
}

// Group the tiles by the normalized value of each of their edges
fn find_edges(input: &Vec<Tile>) -> HashMap<u64, Vec<i64>> {
  // find the tiles that share edges
  let mut edges: HashMap<u64, Vec<i64>> = HashMap::new();
  for t in input {
    for e in t.edges() {
      let norm = t.normalize(e);
      if let Some(old) = edges.get_mut(&norm) {
        old.push(t.id);
      } else {
        edges.insert(norm, vec![t.id]);
      }
    }
  }
  edges
}

// Given the grouping by edges, find the neighbors of each tile
fn find_neighbors(edges: &HashMap<u64, Vec<i64>>) -> HashMap<i64, Vec<i64>> {
  let mut result: HashMap<i64, Vec<i64>> = HashMap::new();
  for (_, tiles) in edges {
    for t in tiles {
      for other in tiles {
        if t != other {
          if let Some(old) = result.get_mut(&t) {
            old.push(*other);
          } else {
            result.insert(*t, vec![*other]);
          }
        }
      }
    }
  }
  result
}

fn parse(input: &mut dyn Iterator<Item=String>) -> Vec<Tile> {
  let mut result = Vec::new();
  while let Some(t) = Tile::parse(input) {
    result.push(t);
  }
  result
}

fn main() {
  let stdin = io::stdin();
  let input = parse( & mut stdin.lock().lines()
    .map(|x| x.unwrap().trim().to_string()));
  let index = find_neighbors(&find_edges(&input));
  for (k,v) in &index {
    println!("{} -> {:?}", k, v);
  }
  // find the tiles with only two neighbors since they are the corners
  // multiply their ids together
  let result: i64 = (&index).iter()
    .filter_map(|(k,v)| if v.len() == 2 {Some(k)} else {None})
    .product();
  println!("result = {}", result);
}
