use std::collections::HashMap;
use std::io;
use std::io::BufRead;

#[derive(Clone, Copy, Debug)]
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

  fn add(&self, other: Point) -> Point {
    Point{x: self.x + other.x, y: self.y + other.y}
  }
}

fn main() {
  let stdin = io::stdin();
  let orders: Vec<Vec<HexDirection>> = stdin.lock().lines()
    .map(|l| HexDirection::parse_line(l.unwrap().trim())).collect();
  let mut state: HashMap<Point, bool> = HashMap::new();
  for cmds in orders {
    let dest = cmds.iter().map(|&c| Point::from_dir(c))
      .fold(Point::new(), |acc, p| acc.add(p));
    println!("{:?}", dest);
    state.insert(dest, !state.get(&dest).or(Some(&false)).unwrap());
  }
  let mut count: u64 = 0;
  for value in state.values() {
    if *value {
      count += 1;
    }
  }
  println!("{} of {}", count, state.len());
}
