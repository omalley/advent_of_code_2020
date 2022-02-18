use std::io;
use std::io::BufRead;

#[derive(Debug, Eq, PartialEq)]
enum Spot {
  Floor,
  Chair,
  Person,
}

#[derive(Debug)]
struct Floor {
  seats: Vec<Vec<Spot>>,
  width: usize,
  occupied: Vec<Vec<bool>>,
}

impl Floor {
  fn parse(input: &mut dyn Iterator<Item=String>) -> Self {
    let mut seats: Vec<Vec<Spot>> = Vec::new();
    for line in input {
      let row = line.chars().map(|c|
        match c {
          '.' => Spot::Floor,
          'L' => Spot::Chair,
          _ => Spot::Person,
        }).collect();
      seats.push(row);
    }
    let width= seats[0].len();
    let occupied = vec![vec![false; width]; seats.len()];
    Floor{seats, width, occupied}
  }

  fn neighbors(&self, x: usize, y: usize) -> usize {
    let mut result = 0;
    for neighbor_x in (x as i64) - 1 ..= (x as i64) + 1 {
      for neighbor_y in (y as i64) - 1 ..= (y as i64) + 1 {
        if neighbor_x >= 0 && neighbor_x < self.width as i64 && neighbor_y >= 0 &&
            neighbor_y < self.seats.len() as i64 &&
            (neighbor_x != x as i64 || neighbor_y != y as i64) {
          if self.occupied[neighbor_y as usize][neighbor_x as usize] {
            result += 1;
          }
        }
      }
    }
    result
  }

  fn update(&mut self) -> bool {
    let mut next = self.occupied.clone();
    let mut result = false;
    for y in 0..next.len() {
      for x in 0..self.width {
        if self.occupied[y][x] {
          if self.neighbors(x,y) >= 4 {
            result = true;
            next[y][x] = false;
          }
        } else if self.seats[y][x] == Spot::Chair && self.neighbors(x,y) == 0 {
          result = true;
          next[y][x] = true;
        }
      }
    }
    if result {
      self.occupied = next;
    }
    result
  }

  fn count(&self) -> usize {
    let mut result = 0;
    for y in 0..self.occupied.len() {
      for x in 0..self.width {
        if self.occupied[y][x] {
          result += 1;
        }
      }
    }
    result
  }
}

fn main() {
  let stdin = io::stdin();
  let mut map: Floor = Floor::parse(&mut stdin.lock().lines()
    .map(|s| s.unwrap()));
  while map.update() {
    // pass
  }
  println!("count = {}", map.count());
}
