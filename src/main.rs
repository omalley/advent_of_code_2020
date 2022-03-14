use std::collections::HashSet;
use std::io;
use std::io::BufRead;

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Clone, Copy, Debug, EnumIter)]
enum HexDirection {
  East,
  SouthEast,
  SouthWest,
  West,
  NorthWest,
  NorthEast,
}

impl HexDirection {
  fn parse(chars: &mut dyn Iterator<Item=char>) -> Option<Self> {
    match chars.next() {
      None => None,
      Some('w') => Some(HexDirection::West),
      Some('e') => Some(HexDirection::East),
      Some('n') => {
        match chars.next() {
          Some('w') => Some(HexDirection::NorthWest),
          Some('e') => Some(HexDirection::NorthEast),
          Some(other) => panic!("bad direction 'n{}'", other),
          None => panic!("bad direction 'n'"),
        }
      }
      Some('s') => {
        match chars.next() {
          Some('w') => Some(HexDirection::SouthWest),
          Some('e') => Some(HexDirection::SouthEast),
          Some(other) => panic!("bad direction 's{}'", other),
          None => panic!("bad direction 's'"),
        }
      }
      Some(other) => panic!("bad direction '{}'", other),
    }
  }

  fn parse_line(line: &str) -> Vec<HexDirection> {
    let mut chars = line.chars();
    let mut result = Vec::new();
    while let Some(dir) = Self::parse(&mut chars) {
      result.push(dir);
    }
    result
  }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Point {
  x: i64,
  y: i64,
}

impl Point {
  fn new() -> Self {
    Point{x:0, y:0}
  }

  fn from_dir(dir: HexDirection) -> Self {
    match dir {
      HexDirection::East => Point{x: 1, y:0},
      HexDirection::West => Point{x: -1, y:0},
      HexDirection::SouthEast => Point{x: 1, y:-1},
      HexDirection::SouthWest => Point{x: 0, y:-1},
      HexDirection::NorthEast => Point{x: 0, y:1},
      HexDirection::NorthWest => Point{x: -1, y:1},
    }
  }

  fn from_cmds(cmds: &Vec<HexDirection>) -> Self {
    cmds.iter().map(|&c| Point::from_dir(c))
      .fold(Point::new(), |acc, p| acc.add(p))
  }

  fn add(&self, other: Point) -> Self {
    Point{x: self.x + other.x, y: self.y + other.y}
  }
}

struct Floor {
  black: HashSet<Point>,
}

impl Floor {
  fn new() -> Self {
    Floor{black: HashSet::new()}
  }

  fn flip(&mut self, pnt: Point) {
    if self.black.contains(&pnt) {
      self.black.remove(&pnt);
    } else {
      self.black.insert(pnt);
    }
  }

  fn count_neighbors(&self, pnt: Point) -> u64 {
    let mut count = 0;
    for dir in HexDirection::iter() {
      let neighbor = pnt.add(Point::from_dir(dir));
      if self.black.contains(&neighbor) {
        count += 1;
      }
    }
    count
  }

  fn bounds(&self) -> (Point, Point) {
    let mut max = Point{x: i64::MIN, y:i64::MIN};
    let mut min = Point{x: i64::MAX, y: i64::MAX};
    for p in &self.black {
      min.x = i64::min(min.x,p.x);
      min.y = i64::min(min.y, p.y);
      max.x = i64::max(max.x, p.x);
      max.y = i64::max(max.y, p.y);
    }
    (min, max)
  }

  fn count(&self) -> usize {
    self.black.len()
  }

  fn advance_day(&mut self) {
    let (min, max) = self.bounds();
    let mut flip: Vec<Point> = Vec::new();
    for x in min.x-1..=max.x+1 {
      for y in min.y-1..=max.y+1 {
        let p = Point{x, y};
        let neighbors = self.count_neighbors(p);
        if self.black.contains(&p) {
          if neighbors == 0 || neighbors > 2 {
            flip.push(p);
          }
        } else if neighbors == 2 {
          flip.push(p);
        }
      }
    }
    for p in flip {
      self.flip(p);
    }
  }
}

fn main() {
  let stdin = io::stdin();
  let points: Vec<Point> = stdin.lock().lines()
    .map(|l| Point::from_cmds(&HexDirection::parse_line(l.unwrap().trim())))
      .collect();
  let mut floor = Floor::new();
  for &pnt in &points {
    floor.flip(pnt);
  }
  for day in 0..100 {
    floor.advance_day();
    println!("Day {}: {}", day+1, floor.count());
  }
}
