use std::io;
use std::io::BufRead;

#[derive(Debug)]
struct CupGame {
  // current is cup 0
  cups: Vec<u32>,
}

impl CupGame {
  fn parse(input: &str) -> Self {
    let cups = input.chars().map(|c| c.to_string().parse::<u32>().unwrap()).collect();
    CupGame{cups}
  }

  fn find_destination(&self, current:u32) -> usize {
    let mut best = u32::MAX;
    let mut best_posn: usize = 0;
    for i in 1..self.cups.len() {
      let new = (10 + current - self.cups[i]) % 10;
      if new < best {
        best = new;
        best_posn = i;
      }
    }
    best_posn
  }

  const CUPS_TO_MOVE: usize = 3;

  fn next(&mut self) {
    let mut moving: Vec<u32> = Vec::new();
    for _ in 0..Self::CUPS_TO_MOVE {
      moving.push(self.cups.remove(1));
    }
    let dest_posn = self.find_destination(self.cups[0]);
    for _ in 0..Self::CUPS_TO_MOVE {
      self.cups.insert(dest_posn + 1, moving.pop().unwrap())
    }
    // Move current one to the right
    let current = self.cups.remove(0);
    self.cups.push(current);
  }
}

const MOVES_TO_MAKE: usize = 100;

fn main() {
  let stdin = io::stdin();
  let mut game = CupGame::parse(stdin.lock().lines().next().unwrap().unwrap().trim());
  for _ in 0..MOVES_TO_MAKE {
    game.next();
  }
  println!("{:?}", game);
}
