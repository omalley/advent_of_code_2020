use std::io;
use std::io::BufRead;

#[derive(Debug)]
struct BoardingCard {
  row: usize,
  seat: usize,
}

impl BoardingCard {
  fn parse(line: &str) -> Self {
    let chars: Vec<char> = line.chars().collect();
    let row = chars[..7].iter()
      .map(|&c| if c == 'F' {0} else {1})
      .fold(0, |acc, v| 2 * acc + v);
    let seat = chars[7..].iter()
      .map(|&c| if c == 'L' {0} else {1})
      .fold(0, |acc, v| 2 * acc + v);
    BoardingCard{row, seat}
  }

  fn seat_id(&self) -> usize {
    self.row * 8 + self.seat
  }
}

fn main() {
  let stdin = io::stdin();
  let cards: Vec<BoardingCard> = stdin.lock().lines()
    .map(|x| BoardingCard::parse(x.unwrap().trim()))
    .collect();
  let mut seats: Vec<usize> = cards.iter().map(|s| s.seat_id()).collect();
  seats.sort();
  for i in 1..seats.len() {
    if seats[i-1] + 2 == seats[i] {
      println!("seat = {}", seats[i-1] + 1);
    }
  }
}
