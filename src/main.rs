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

#[derive(Debug)]
struct State {
  x: i64,
  y: i64,
  waypoint_x: i64,
  waypoint_y: i64,
}

impl State {
  fn new() -> Self {
    State{x: 0, y: 0, waypoint_x: 10, waypoint_y: -1}
  }

  fn execute(&mut self, command: &Command) {
    match command {
      Command::North(dist) => self.waypoint_y -= dist,
      Command::South(dist) => self.waypoint_y += dist,
      Command::West(dist) => self.waypoint_x -= dist,
      Command::East(dist) => self.waypoint_x += dist,
      Command::Left(deg) => self.rotate(-*deg),
      Command::Right(deg) => self.rotate(*deg),
      Command::Forward(mult) => {
        self.x += mult * self.waypoint_x;
        self.y += mult * self.waypoint_y;
      }
    }
  }

  fn rotate(&mut self, deg: i64) {
    let deg = ((deg % 360) + 360) % 360;
    let old = (self.waypoint_x, self.waypoint_y);
    match deg {
      0 => {},
      90 => {
        self.waypoint_y = old.0;
        self.waypoint_x = -old.1;
      }
      180 => {
        self.waypoint_y = -old.1;
        self.waypoint_x = -old.0;
      }
      270 => {
        self.waypoint_y = -old.0;
        self.waypoint_x = old.1;
      }
      _ => panic!("non-cardinal direction {}", deg),
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
  let mut state = State::new();
  for cmd in cmds {
    state.execute(&cmd);
  }
  println!("state = {:?}, dist = {}", state, state.distance());
}
