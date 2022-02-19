use std::convert::TryFrom;
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

  fn find_neighbor(&self, x: usize, y:usize, x_delta: i64, y_delta: i64) -> bool {
    let mut cur_x= x as i64 + x_delta;
    let mut cur_y = y as i64 + y_delta;
    while let (Ok(x), Ok(y)) = (usize::try_from(cur_x), usize::try_from(cur_y)) {
      if x >= self.width || y >= self.seats.len() {
        break
      }
      if self.seats[y][x] != Spot::Floor {
        return self.occupied[y][x]
      }
      cur_x += x_delta;
      cur_y += y_delta;
    }
    false
  }

  fn neighbors(&self, x: usize, y: usize) -> usize {
    let mut result = 0;
    for (del_x, del_y) in vec![(-1,-1), (-1,0), (-1, 1), (0, -1),
                               (0, 1), (1, -1), (1, 0), (1, 1)] {
      if self.find_neighbor(x, y, del_x, del_y) {
        result += 1;
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
          if self.neighbors(x,y) >= 5 {
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
