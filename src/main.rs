use std::collections::{HashMap, HashSet};
use std::io;
use std::io::BufRead;

use lazy_static::lazy_static;
use regex::Regex;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum Direction {
  Top=0, Left, Bottom, Right
}

impl Direction {
  // rotate from self to goal
  fn rotate_to(&self, goal:Self) -> Self {
    match ((*self as i32) - (goal as i32) + 4) % 4 {
      0 => Direction::Top,
      1 => Direction::Right,
      2 => Direction::Bottom,
      3 => Direction::Left,
      _ => panic!("Bad direction {:?} - {:?}", self, goal),
    }
  }

  fn flip(&self, flip_x: bool) -> Self {
    if flip_x {
      match self {
        Direction::Top => Direction::Top,
        Direction::Left => Direction::Right,
        Direction::Bottom => Direction::Bottom,
        Direction::Right => Direction::Left,
      }
    } else {
      *self
    }
  }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct BorderName {
  direction: Direction,
  id: i64,
  flipped: bool,
}

#[derive(Clone, Debug)]
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

  // uses the min of the value and the value swapped left for right
  fn reverse_bits(&self, val: u64) -> u64 {
    (0..self.width).map(|x| (val >> x) & 1).fold(0, |acc, x| acc << 1 | x)
  }

  fn convert_row(&self, row: usize) -> u64 {
    self.pixels[row].iter().map(|x| if *x {1} else {0})
      .fold(0, |acc, x| acc << 1 | x)
  }

  fn convert_column(&self, column: usize) -> u64 {
    (0..self.pixels.len()).map(|r| if self.pixels[r][column] {1} else {0})
      .fold(0, |acc, x| acc << 1 | x)
  }

  // For each direction, return the normalized edge and whether it was flipped.
  // The MSB is in the counter clockwise direction (top = left msb, right = top msb, etc...)
  fn edges(&self) -> Vec<(Direction, u64, bool)> {
    vec![(Direction::Top, self.convert_row(0)),
         (Direction::Left, self.reverse_bits(self.convert_column(0))),
         (Direction::Bottom, self.reverse_bits(self.convert_row(self.width - 1))),
         (Direction::Right, self.convert_column(self.width - 1))].iter()
      .map(|(d, e)| {let r = self.reverse_bits(*e);
        (d.clone(), u64::min(*e, r), r < *e)}).collect::<Vec<(Direction, u64, bool)>>()
  }

  fn rotate(&self, placement: &Placement) -> Self {
    let mut pixels = vec![vec![false; self.width]; self.width];
    for x in 0..self.width {
      for y in 0..self.width {
        let rotated = rotate_point(x, y, self.width, self.width,
                                   placement.rotation, placement.flip_x).unwrap();
        pixels[rotated.1][rotated.0] = self.pixels[y][x];
      }
    }
    Tile{id: self.id, pixels, width: self.width}
  }
}

fn parse(input: &mut dyn Iterator<Item=String>) -> Vec<Tile> {
  let mut result = Vec::new();
  while let Some(t) = Tile::parse(input) {
    result.push(t);
  }
  result
}

// Group the tiles by the normalized value of each of their edges
fn find_edges(input: &Vec<Tile>) -> HashMap<u64, Vec<BorderName>> {
  // find the tiles that share edges
  let mut edges: HashMap<u64, Vec<BorderName>> = HashMap::new();
  for t in input {
    for (direction, edge, flipped) in t.edges() {
      if let Some(old) = edges.get_mut(&edge) {
        old.push(BorderName { direction, id: t.id, flipped});
      } else {
        edges.insert(edge, vec![BorderName { direction, id: t.id, flipped}]);
      }
    }
  }
  edges
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Connection {
  source: BorderName,
  destination: BorderName,
}

// Given the grouping by edges, find the neighbors of each tile
fn find_neighbors(edges: &HashMap<u64, Vec<BorderName>>) -> HashMap<i64, Vec<Connection>> {
  let mut result: HashMap<i64, Vec<Connection>> = HashMap::new();
  for (_, borders) in edges {
    for source in borders {
      for destination in borders {
        if source != destination {
          if let Some(old) = result.get_mut(&source.id) {
            old.push(Connection{source: source.clone(), destination: destination.clone()});
          } else {
            result.insert(source.id,
                          vec![Connection{source: source.clone(),
                            destination: destination.clone()}]);
          }
        }
      }
    }
  }
  // sort the connection lists for each tile
  for list in &mut result.values_mut() {
    list.sort_unstable();
  }
  result
}

#[derive(Clone, Debug)]
struct Placement {
  id: i64,
  rotation: Direction,
  flip_x: bool,
}

// Rotate the given point within the given output box and the given rotation.
// Returns None if the resulting coordinate is outside of (0..width,0..height).
fn rotate_point(x: usize, y: usize,
                width: usize, height: usize,
                rotation: Direction, flip_x: bool) -> Option<(usize, usize)> {
  // If the result is outside of the bounds, return None.
  match rotation {
    Direction::Top | Direction::Bottom => if x >= width || y >= height {return None},
    Direction::Left | Direction::Right => if y >= width || x >= height {return None},
  }

  let mut rot_x;
  let rot_y;
  match rotation {
    Direction::Top => {rot_x = x; rot_y = y},
    Direction::Left => {rot_x = y; rot_y = height - 1 - x},
    Direction::Bottom => {rot_x = width - 1 - x; rot_y = height - 1 - y},
    Direction::Right => {rot_x = width - 1 - y; rot_y = x},
  }
  if flip_x {
    rot_x = width - 1 - rot_x;
  }
  Some((rot_x, rot_y))
}


// Given a tile and its connections, find the placement so that the left and up tiles
// match.
fn make_placement(id: i64, connections: &Vec<Connection>,
                  left: &Option<Placement>, up: &Option<Placement>) -> Placement {
  let dir;
  let flip_x;
  match up {
    Some(north) => {
      let conn = connections.iter()
        .find(|&x| x.destination.id == north.id).unwrap();
      flip_x = (conn.source.flipped == conn.destination.flipped) != north.flip_x;
      dir = conn.source.direction.rotate_to(Direction::Top.flip(flip_x));
    },
    None => {
      match left {
      Some(west) => {
        let conn = connections.iter()
          .find(|&x| x.destination.id == west.id).unwrap();
        flip_x = (conn.source.flipped == conn.destination.flipped) != west.flip_x;
        dir = conn.source.direction.rotate_to(Direction::Left.flip(flip_x))
      },
      None => {
          flip_x = false;
          // depends on the connections being sorted by source direction
          match connections[0].source.direction {
            Direction::Top => if connections[1].source.direction == Direction::Left {
              // top & left
              dir = Direction::Bottom;
            } else {
              // must be top & right
              dir = Direction::Right;
            }
            Direction::Left => {
              // left & bottom
              dir = Direction::Left;
            }
            Direction::Bottom => {
              // bottom & right
              dir = Direction::Top;
            }
            Direction::Right => panic!("Confusing corner {:?}", connections),
          }
        }
      }
    }
  }
  Placement{id, rotation: dir, flip_x }
}

// Find the tile in the move_to direction so that we can place it next
fn next_tile(placement: &Placement, connections: &Vec<Connection>,
             move_to: Direction) -> Option<i64> {
  let dir = placement.rotation.rotate_to(move_to.flip(placement.flip_x));
  let result = connections.iter()
    .find(|&c| c.source.direction == dir)
    .and_then(|c| Some(c.destination.id));
  result
}

#[derive(Debug)]
struct Layout {
  tiles: HashMap<i64, Tile>,
  placements: Vec<Vec<Placement>>,
}

impl Layout {
  // Build up the map starting from the upper left corner and build each
  // row until the entire layout is discovered.
  fn new(tiles: &Vec<Tile>) -> Self {
    let connections = find_neighbors(&find_edges(tiles));
    let mut seen = HashSet::new();
    // find the corners
    let mut corners: Vec<i64> = connections.iter()
      .filter_map(|(id, conn)| if conn.len() == 2 {Some(*id)} else {None})
      .collect();
    corners.sort_unstable();
    let mut placements: Vec<Vec<Placement>> = Vec::new();
    // Add each row of placements, starting from a corner
    let mut current = Some(corners[0]);
    while current.is_some() {
      let mut row = Vec::new();
      let mut prev: Option<Placement> = None;
      // within the row, add each tile
      while let Some(curr) = current {
        if !seen.insert(curr) {
          panic!("Loop detected at {}", curr);
        }
        let up: Option<Placement>;
        if placements.is_empty() {
          up = None;
        } else {
          up = Some(placements.get(placements.len() - 1).unwrap().get(row.len()).unwrap().clone());
        }
        let place = make_placement(curr, &connections.get(&curr).unwrap(),
                                   &prev, &up);
        row.push(place.clone());
        prev = Some(place);
        current = next_tile(&row[row.len() - 1],
                            &connections[&current.unwrap()], Direction::Right);
      }
      current = next_tile(&row[0], &connections[&row[0].id],
                          Direction::Bottom);
      placements.push(row);
    }
    let mut tile_map = HashMap::new();
    for t in tiles {
      tile_map.insert(t.id, t.clone());
    }
    Layout{tiles: tile_map, placements}
  }

  fn map(&self) -> Vec<Vec<bool>> {
    let tile_width = self.tiles.values().next().unwrap().width - 2;
    let width = self.placements[0].len() * tile_width;
    let height = self.placements.len() * tile_width;
    let mut result = vec![vec![false; width]; height];
    for y_tile in 0..self.placements.len() {
      for x_tile in 0..self.placements[y_tile].len() {
        let place = &self.placements[y_tile][x_tile];
        let rot_tile = self.tiles.get(&place.id).unwrap().rotate(place);
        for y in 0..tile_width {
          for x in 0..tile_width {
            result[y_tile*tile_width+y][x_tile*tile_width+x] = rot_tile.pixels[y+1][x+1];
          }
        }
      }
    }
    result
  }
}

struct Mask {
  mask: Vec<Vec<bool>>,
  width: usize,
}

impl Mask {
  fn dragon() -> Self {
    let pattern= vec!["                  # ",
                      "#    ##    ##    ###",
                      " #  #  #  #  #  #   "];
    let mask: Vec<Vec<bool>> = pattern.iter()
      .map(|s| s.chars().map(|c| c == '#').collect()).collect();
    let width = pattern[0].len();
    Mask{mask, width}
  }

  // Does the mask match the pixels at the given offset, rotation, and flip?
  fn matches(&self, offset_x: usize, offset_y: usize, rotation: Direction, flip_x: bool,
             pixels: &Vec<Vec<bool>>, width: usize, height: usize) -> bool {
    for x in 0..self.width {
      for y in 0..self.mask.len() {
        if let Some(rotate) = rotate_point(offset_x+x, offset_y+y, width, height,
                                     rotation, flip_x) {
          if self.mask[y][x] && !pixels[rotate.1][rotate.0] {
            return false
          }
        } else {
          return false
        }
      }
    }
    true
  }

  fn set(&self, offset_x: usize, offset_y: usize, flag: &mut Vec<Vec<bool>>) {
    for x in 0..self.width {
      for y in 0..self.mask.len() {
        if self.mask[y][x] {
          flag[offset_y + y][offset_x + x] = true;
        }
      }
    }
  }

  fn find(&self, pixels: &Vec<Vec<bool>>, rotation: Direction, flip_x: bool) -> usize {
    let width;
    let height;
    match rotation {
      Direction::Top | Direction::Bottom => {width = pixels[0].len(); height = pixels.len()},
      Direction::Left | Direction::Right => {width = pixels.len(); height = pixels[0].len()},
    }
    let mut result = vec![vec![false; width]; height];
    for offset_x in 0..width-self.width {
      for offset_y in 0..height-self.mask.len() {
        if self.matches(offset_x, offset_y, rotation, flip_x, pixels, width, height) {
          self.set(offset_x, offset_y, &mut result);
        }
      }
    }
    count(&result)
  }
}

fn count(map: &Vec<Vec<bool>>) -> usize {
  map.iter().flat_map(|r| r.iter())
    .fold(0, |acc, b| if *b { acc + 1 } else { acc })
}

fn print_map(map: &Vec<Vec<bool>>) {
  for row in map {
    println!("{}", row.iter().map(|&b| if b {'#'} else {'.'}).collect::<String>());
  }
}

fn main() {
  let stdin = io::stdin();
  let input = parse( & mut stdin.lock().lines()
    .map(|x| x.unwrap().trim().to_string()));
  let layout = Layout::new(&input);
  let map = layout.map();
  print_map(&map);
  println!();
  let count = count(&map);
  let dragon = Mask::dragon();
  for dir in &[Direction::Top, Direction::Left, Direction::Bottom, Direction::Right] {
    for flip in &[false, true] {
      println!("dir = {:?}, flip = {}, count = {}", dir, flip,
               count - dragon.find(&map, *dir, *flip));
    }
  }
}
