use std::io;
use std::io::BufRead;

use regex::{Match, Regex};

#[derive(Debug, PartialEq, Eq)]
enum Token {
  Number(i64),
  Add,
  Multiply,
  Left,
  Right,
}

struct Lexer {
  regexs: Vec<(Regex, Box<dyn Fn(Match) -> Option<Token>>)>,
}

impl Lexer {
  fn new() -> Self {
    let mut regexs: Vec<(Regex, Box<dyn Fn(Match) -> Option<Token>>)> = Vec::new();
    regexs.push((Regex::new(r"^\d+").unwrap(),
                 Box::new(|m| Some(Token::Number(m.as_str().parse().unwrap())))));
    regexs.push((Regex::new(r"^\+").unwrap(), Box::new(|_| Some(Token::Add))));
    regexs.push((Regex::new(r"^\*").unwrap(), Box::new(|_| Some(Token::Multiply))));
    regexs.push((Regex::new(r"^\(").unwrap(), Box::new(|_| Some(Token::Left))));
    regexs.push((Regex::new(r"^\)").unwrap(), Box::new(|_| Some(Token::Right))));
    regexs.push((Regex::new(r"^\s+").unwrap(), Box::new(|_| None)));
    Lexer{regexs}
  }

  fn parse(&self, line: &str) -> Vec<Token> {
    let mut result = Vec::new();
    let mut posn = 0;
    while posn < line.len() {
      for (regex, handler) in &self.regexs {
        if let Some(m) = regex.find(&line[posn..]) {
          posn += m.end();
          if let Some(t) = handler(m) {
            result.push(t);
          }
          break;
        }
      }
    }
    result
  }
}

fn main() {
  let stdin = io::stdin();
  let lines: Vec<String> = stdin.lock().lines()
    .map(|x| x.unwrap().trim().to_string())
    .filter(|x| !x.is_empty())
    .collect();
  let lexer = Lexer::new();
  for line in lines {
    println!("{:?}", lexer.parse(&line));
  }
}
