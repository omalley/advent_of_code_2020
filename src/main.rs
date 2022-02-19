use std::io;
use std::io::BufRead;

use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug)]
enum Command {
  North(i64),
  East(i64),
  South(i64),
  West(i64),
  Left(i64),
  Right(i64),
  Forward(i64),
}

impl Command {
  fn parse(input: &str) -> Self {
    lazy_static! {
      static ref COMMAND: Regex = Regex::new(r"^(?P<cmd>[NESWLRF])(?P<arg>-?\d+)$").unwrap();
    }
    let capture = COMMAND.captures(input).unwrap();
    let cmd = capture.name("cmd").unwrap().as_str();
    let arg = capture.name("arg").unwrap().as_str().parse::<i64>().unwrap();
    match cmd {
      "N" => Self::North(arg),
      "S" => Self::South(arg),
      "W" => Self::West(arg),
      "E" => Self::East(arg),
      "L" => Self::Left(arg),
      "R" => Self::Right(arg),
      "F" => Self::Forward(arg),
      _ => panic!("Unknown command"),
    }
  }
}

fn wrap_360(dir: i64) -> i64 {
  ((dir % 360) + 360) % 360
}

#[derive(Debug,Default)]
struct State {
  x: i64,
  y: i64,
  facing: i64,
}

impl State {
  fn execute(&mut self, command: &Command) {
    match command {
      Command::North(dist) => self.y -= dist,
      Command::South(dist) => self.y += dist,
      Command::West(dist) => self.x -= dist,
      Command::East(dist) => self.x += dist,
      Command::Left(deg) => self.facing = wrap_360(self.facing - deg),
      Command::Right(deg) => self.facing = wrap_360(self.facing + deg),
      Command::Forward(dist) => {
        match self.facing {
          0 => self.x += dist,
          90 => self.y += dist,
          180 => self.x -= dist,
          270 => self.y -= dist,
          _ => panic!("Non-cardinal direction - {}", self.facing),
        }
      }
    }
  }

  fn distance(&self) -> i64 {
    i64::abs(self.x) + i64::abs(self.y)
  }
}

fn main() {
  let stdin = io::stdin();
  let cmds: Vec<Command> = stdin.lock().lines()
    .map(|s| Command::parse(s.unwrap().trim())).collect();
  let mut state = State::default();
  for cmd in cmds {
    state.execute(&cmd);
  }
  println!("state = {:?}, dist = {}", state, state.distance());
}
