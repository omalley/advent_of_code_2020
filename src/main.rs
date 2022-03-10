use std::collections::HashSet;
use std::io;
use std::io::BufRead;

#[derive(Debug)]
struct WarGame {
  players: Vec<Vec<i32>>,
  previous: HashSet<Vec<Vec<i32>>>,
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
    WarGame{players, previous: HashSet::new()}
  }

  fn can_recurse(&self, hand: &Vec<i32>) -> bool {
    for p in 0..hand.len() {
      if (self.players[p].len() as i32) < hand[p] {
        return false;
      }
    }
    true
  }

  fn recurse_game(&self, hand: &Vec<i32>) -> usize {
    let mut players: Vec<Vec<i32>> = Vec::new();
    for p in 0..hand.len() {
      players.push(self.players[p][..hand[p] as usize].iter().map(|&c| c).collect());
    }
    let mut sub_game = WarGame{players, previous: HashSet::new()};
    loop {
      if let Some(winner) = sub_game.next() {
        return winner
      }
    }
  }

  fn next(&mut self) -> Option<usize> {
    // If we're repeating the previous state, player 0 wins
    if self.previous.contains(&self.players) {
      for p in 1..self.players.len() {
        self.players[p].clear();
      }
      return Some(0)
    }
    // Save the current state into previous
    self.previous.insert(self.players.clone());
    // get the list of top cards from each hand
    let mut hand: Vec<i32> = self.players.iter_mut()
      .map(|h| h.remove(0)).collect();
    // figure out the winner
    let winner;
    if self.can_recurse(&hand) {
      winner = self.recurse_game(&hand);
    } else {
      winner = hand.iter().enumerate()
        .reduce(|acc, c| if c.1 > acc.1 {c} else {acc}).unwrap().0;
    }
    let winning_card = hand.remove(winner);
    hand.insert(0, winning_card);
    // put the cards back on the winner's deck
    self.players[winner].append(&mut hand);
    // remove a player that is out of cards
    self.players.retain(|h| !h.is_empty());
    if self.is_done() {
      Some(winner)
    } else {
      None
    }
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
