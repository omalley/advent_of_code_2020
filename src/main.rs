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

#[derive(Clone, Debug)]
enum Expression {
  Number(i64),
  Add(Box<Expression>, Box<Expression>),
  Multiply(Box<Expression>, Box<Expression>),
}

impl Expression {
  fn parse_term(input: &mut dyn Iterator<Item=&Token>) -> Option<Expression> {
    if let Some(head) = input.next() {
      match head {
        Token::Number(i) => Some(Expression::Number(*i)),
        Token::Left => Some(Self::parse(input)),
        _ => panic!("Illegal token - {:?}", head),
      }
    } else {
      None
    }
  }

  fn parse(input: &mut dyn Iterator<Item=&Token>) -> Expression {
    let mut current = Self::parse_term(input).unwrap();
    while let Some(head) = input.next() {
      match &head {
        Token::Add => {
          let right = Self::parse_term(input).unwrap();
          current = Expression::Add(Box::new(current),
                                    Box::new(right))
        },
        Token::Multiply => {
          let right = Self::parse_term(input).unwrap();
          current = Expression::Multiply(Box::new(current),
                                         Box::new(right))
        },
        Token::Right => break,
        _ => panic!("Bad token {:?}", head),
      }
    }
    current
  }

  fn evaluate(&self) -> i64 {
    match self {
      Expression::Number(i) => *i,
      Expression::Add(left, right) =>
        left.evaluate() + right.evaluate(),
      Expression::Multiply(left, right) =>
        left.evaluate() * right.evaluate(),
    }
  }
}

fn main() {
  let stdin = io::stdin();
  let lines: Vec<String> = stdin.lock().lines()
    .map(|x| x.unwrap().trim().to_string())
    .filter(|x| !x.is_empty())
    .collect();
  let lexer = Lexer::new();
  let mut sum = 0;
  for line in lines {
    let expr = Expression::parse(&mut lexer.parse(&line).iter());
    let val = expr.evaluate();
    println!("{}", val);
    sum += val;
  }
  println!("sum = {}", sum);
}
