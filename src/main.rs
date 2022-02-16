use std::io;
use std::io::BufRead;

#[derive(Debug)]
struct Field {
  trees: Vec<Vec<bool>>,
  width: usize,
  height: usize,
}

impl Field {
  fn parse(lines: &mut dyn Iterator<Item=String>) -> Self {
    let trees: Vec<Vec<bool>> = lines
      .map(|l| l.chars().map(|c| c == '#').collect())
      .collect();
    let width = trees[0].len();
    let height = trees.len();
    Field{trees, width, height}
  }

  fn trees_hit(&self, path: (usize, usize)) -> usize {
    let mut x = 0;
    let mut y = 0;
    let mut trees = 0;
    while y < self.height {
      if self.trees[y][x] {
        trees += 1;
      }
      x = (x + path.0) % self.width;
      y += path.1;
    }
    trees
  }
}

fn main() {
  let stdin = io::stdin();
  let field = Field::parse(&mut stdin.lock().lines()
    .map(|x| String::from(x.unwrap().trim())));
  let mut product = 1;
  for path in vec![(1,1), (3,1), (5,1), (7,1), (1, 2)] {
    let hits = field.trees_hit(path);
    println!("{:?} = {}", path, hits);
    product *= hits;
  }
  println!("product = {}", product);
}
