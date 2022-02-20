use std::fmt;
use std::io;
use std::io::BufRead;

type Point = (i64, i64, i64, i64);
type Matrix = Vec<Vec<Vec<Vec<bool>>>>;

#[derive(Debug)]
struct ConwaySpace {
  origin: Point,
  far_corner: Point,
  active: Matrix,
}

impl ConwaySpace {

  fn parse(input: &mut dyn Iterator<Item=String>) -> Self {
    let origin = (0, 0, 0, 0);
    let mut far_corner = (0, 0, 1, 1);
    let mut active: Matrix = Vec::new();
    let mut plane: Vec<Vec<bool>> = Vec::new();
    for line in input {
      plane.push(line.chars().map(|c| c == '#').collect());
    }
    far_corner.1 = plane.len() as i64;
    if far_corner.1 > 0 {
      far_corner.0 = plane[0].len() as i64;
    }
    active.push(vec![plane]);
    ConwaySpace{origin, far_corner, active}
  }

  // Is a given cell active?
  fn is_active(&self, point: Point) -> bool {
    if point.0 >= self.origin.0 && point.0 < self.far_corner.0 &&
      point.1 >= self.origin.1 && point.1 < self.far_corner.1 &&
      point.2 >= self.origin.2 && point.2 < self.far_corner.2 &&
      point.3 >= self.origin.3 && point.3 < self.far_corner.3 {
      return self.active[(point.3 - self.origin.3) as usize][(point.2 - self.origin.2) as usize]
                        [(point.1 - self.origin.1) as usize][(point.0 - self.origin.0) as usize];
    }
    false
  }

  // count the active neighbors
  fn count_neighbors(&self, point: Point) -> u64 {
    let mut result = 0;
    for x_delta in -1..2 {
      for y_delta in -1..2 {
        for z_delta in -1..2 {
          for w_delta in -1..2 {
            if x_delta != 0 || y_delta != 0 || z_delta != 0 || w_delta != 0{
              if self.is_active((point.0 + x_delta, point.1 + y_delta,
                                 point.2 + z_delta, point.3 + w_delta)) {
                result += 1;
              }
            }
          }
        }
      }
    }
    result
  }

  fn create_matrix(origin: Point, far_corner: Point) -> Matrix {
    let mut result = Vec::new();
    for _ in origin.3..far_corner.3 {
      let mut space = Vec::new();
      for _ in origin.2..far_corner.2 {
        let mut plane = Vec::new();
        for _ in origin.1..far_corner.1 {
          plane.push(vec![false; (far_corner.0 - origin.0) as usize]);
        }
        space.push(plane);
      }
      result.push(space);
    }
    result
  }

  fn set(active: &mut Matrix, origin: Point, point: Point, value: bool) {
    active[(point.3 - origin.3) as usize][(point.2 - origin.2) as usize]
      [(point.1 - origin.1) as usize][(point.0 - origin.0) as usize] = value;
  }

  fn next(&mut self) {
    let new_origin = (self.origin.0 - 1, self.origin.1 -1, self.origin.2 - 1,
                      self.origin.3 - 1);
    let new_far = (self.far_corner.0 + 1, self.far_corner.1 + 1,
                   self.far_corner.2 + 1, self.far_corner.2 + 1);
    let mut new_active = Self::create_matrix(new_origin, new_far);
    for x in new_origin.0..new_far.0 {
      for y in new_origin.1..new_far.1 {
        for z in new_origin.2..new_far.2 {
          for w in new_origin.3..new_far.3 {
            let neighbors = self.count_neighbors((x, y, z, w));
            if self.is_active((x, y, z, w)) {
              Self::set(&mut new_active, new_origin, (x, y, z, w),
                        (2..=3).contains(&neighbors));
            } else {
              Self::set(&mut new_active, new_origin, (x, y, z, w), neighbors == 3);
            }
          }
        }
      }
    }
    self.active = new_active;
    self.origin = new_origin;
    self.far_corner = new_far;
  }

  fn active(&self) -> u64 {
    let mut result = 0;
    for space in &self.active {
      for plane in space {
        for row in plane {
          result += row.iter().filter(|&&b| b).count();
        }
      }
    }
    result as u64
  }
}

impl fmt::Display for ConwaySpace {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    writeln!(f, "origin = {:?}, far = {:?}", self.origin, self.far_corner)?;
    for w in self.origin.3..self.far_corner.3 {
      for z in self.origin.2..self.far_corner.2 {
        writeln!(f, "z = {}, w = {}", z, w)?;
        for y in self.origin.1..self.far_corner.1 {
          for x in self.origin.0..self.far_corner.0 {
            write!(f, "{}", if self.is_active((x, y, z, w)) {'#'} else {'.'})?;
          }
          writeln!(f)?
        }
        writeln!(f)?;
      }
    }
    write!(f, "")
  }
}

fn main() {
  let stdin = io::stdin();
  let mut space= ConwaySpace::parse(&mut stdin.lock().lines()
    .map(|x| x.unwrap().trim().to_string())
    .filter(|x| !x.is_empty()));
  for _ in 0..6 {
    println!("active = {}", space.active());
    space.next();
  }
  println!("{}", space);
  println!("active = {}", space.active());
}
