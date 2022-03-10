use std::io;
use std::io::BufRead;

#[derive(Debug)]
struct WarGame {
  players: Vec<Vec<i32>>,
}

impl WarGame {
  fn parse(input: &mut dyn Iterator<Item=String>) -> Self {
    let mut players: Vec<Vec<i32>> = Vec::new();
    while let Some(line) = input.next() {
      if !line.starts_with("Player") {
        panic!("Bad input line {}", line);
      }

      let mut hand: Vec<i32> = Vec::new();
      while let Some(line) = input.next() {
        match line.parse::<i32>() {
          Ok(card) => hand.push(card),
          _ => break,
        }
      }
      players.push(hand);
    }
    WarGame{players}
  }

  fn next(&mut self) {
    // get the list of top cards from each hand
    let mut hand: Vec<i32> = self.players.iter_mut()
      .map(|h| h.remove(0)).collect();
    // figure out the winner
    let winner = hand.iter().enumerate()
      .reduce(|acc, c| if c.1 > acc.1 {c} else {acc}).unwrap().0;
    // sort the hand
    hand.sort_unstable_by(|l,r| i32::cmp(r,l));
    // put the cards back on the winner's deck
    self.players[winner].append(&mut hand);
    // remove a player that is out of cards
    self.players.retain(|h| !h.is_empty());
  }

  fn is_done(&self) -> bool {
    self.players.len() == 1
  }

  fn score(&self) -> i32 {
    let card_count = self.players[0].len();
    self.players[0].iter().enumerate()
      .fold(0, |acc, (posn, card) | acc + card * (card_count - posn) as i32)
  }
}

fn main() {
  let stdin = io::stdin();
  let mut game = WarGame::parse(&mut stdin.lock().lines()
    .map(|x| x.unwrap().trim().to_string()));
  while !game.is_done() {
    println!("{:?}", game);
    game.next();
  }
  println!("score = {}", game.score());
}
