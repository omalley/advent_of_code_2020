use std::collections::HashSet;
use std::io;
use std::io::BufRead;

#[derive(Debug)]
struct Person {
  answers: Vec<char>,
}

impl Person {
  fn parse(line: &str) -> Self {
    let mut answers: Vec<char> = line.trim().chars().collect();
    answers.sort();
    Person{answers}
  }

  fn set(&self) -> HashSet<char> {
    self.answers.iter().map(|&c| c).collect()
  }
}

#[derive(Debug)]
struct Group {
  people: Vec<Person>,
}

impl Group {
  fn parse(lines: &mut dyn Iterator<Item=Result<String,io::Error>>) -> Option<Self> {
    let mut people: Vec<Person> = Vec::new();
    for line in lines {
      let line = line.unwrap();
      if line.is_empty() {
        return Some(Group{people})
      } else {
        people.push(Person::parse(&line));
      }
    }
    if people.is_empty() {
      None
    } else {
      Some(Group{people})
    }
  }

  fn count(&self) -> usize {
    self.people.iter().map(|p| p.set())
      .reduce(|l, r| l.intersection(&r).map(|&c| c).collect())
      .unwrap_or(HashSet::new()).len()
  }
}

fn main() {
  let stdin = io::stdin();
  let mut input = stdin.lock().lines();
  let mut count = 0;
  while let Some(group) = Group::parse(&mut input) {
    println!("{:?} = {}", group, group.count());
    count += group.count();
  }
  println!("{}", count);
}
