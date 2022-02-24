use std::collections::{HashMap, HashSet};
use std::io;
use std::io::BufRead;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum Item {
  Ref(usize),
  Letter(char),
}

impl Item {
  fn parse(s: &str) -> Self {
    let s = s.trim();
    if let Ok(i) = s.parse() {
      Item::Ref(i)
    } else {
      let parts: Vec<char> = s.chars().collect();
      Item::Letter(parts[1])
    }
  }
}

#[derive(Debug)]
struct Sequence {
  items: Vec<Item>,
}

impl Sequence {
  fn parse(s: &str) -> Self {
    let items = s.trim().split_ascii_whitespace()
      .map(|x| Item::parse(x)).collect();
    Sequence{items}
  }
}

#[derive(Debug)]
struct Rule {
  name: usize,
  alternatives: Vec<Sequence>,
}

impl Rule {
  fn parse(s: &str) -> Self {
    let mut parts = s.split(":");
    let name: usize = *&parts.next().unwrap().trim().parse().unwrap();
    let alternatives = parts.next().unwrap().trim().split("|")
      .map(|x| Sequence::parse(x)).collect();
    Rule{name, alternatives}
  }
}

#[derive(Debug)]
struct Input {
  rules: HashMap<usize, Rule>,
  strings: Vec<String>,
  singletons: HashMap<Item, Vec<usize>>,
}

impl Input {
  fn parse(input: &mut dyn Iterator<Item=String>) -> Self {
    let mut rules = HashMap::new();
    let mut singletons: HashMap<Item, Vec<usize>> = HashMap::new();
    while let Some(line) = input.next() {
      if line.is_empty() {
        break;
      } else {
        let rule = Rule::parse(&line);
        for a in &rule.alternatives {
          if a.items.len() == 1 {
            let key = &a.items[0];
            if let Some(old) = singletons.get_mut(key) {
              old.push(rule.name);
            } else {
              singletons.insert(key.clone(), vec![rule.name]);
            }
          }
        }
        let name = rule.name;
        rules.insert(name, rule);
      }
    }
    Input{rules, singletons, strings: input.collect()}
  }

  fn match_alternative(&self, alt: &[Item],
                       start: usize, end: usize,
                       chars: &Vec<char>,
                       subs: &Vec<Vec<HashSet<usize>>>) -> bool {
    //println!("match alternative {:?} to {:?} ({}..={})", alt, chars[start..=end].iter().collect::<String>(), start, end);
    let length = end - start + 1;
    if alt.len() > 0 && alt.len() <= length {
      let head = &alt[0];
      let tail = &alt[1..];
      match head {
        Item::Letter(c) => if *c == chars[start] {
          if tail.is_empty() && length == 1 {
            //println!("matched literal");
            return true
          } else {
            let result = self.match_alternative(tail, start + 1, end, chars, subs);
            //println!("literal & pattern = {}", result);
            return result
          }
        },
        Item::Ref(r) => {
          if tail.is_empty() {
            let result = subs[length - 1][start].contains(r);
            //println!("tail pattern = {}", result);
            return result
          } else {
            for child_end in start..= end - tail.len() {
              if subs[child_end - start][start].contains(r) {
                if self.match_alternative(tail, child_end+1, end, chars, subs) {
                  //println!("Found match at {}", child_end);
                  return true
                }
              }
            }
          }
        },
      }
    }
    //println!("return false");
    false
  }

  fn check(&self, s: &str) -> HashSet<usize> {
    let chars: Vec<char> = s.chars().collect();
    let mut result = vec![vec![HashSet::new(); chars.len()]; chars.len()];
    for length in 1..=chars.len() {
      for start in 0..=chars.len() - length {
        let mut to_do: Vec<usize> = Vec::new();

        // check each rule to see what we can match
        for rule in self.rules.values() {
          for alt in &rule.alternatives {
            if self.match_alternative(&alt.items[..], start, start + length - 1,
                                      &chars,&result) {
              to_do.push(rule.name);
              // once a rule is matched, we can check the next rule
              break;
            }
          }
        }

        // for rules that match singletons, they get checked at the end here.
        while let Some(r) = to_do.pop() {
          if result[length - 1][start].insert(r) {
            if let Some(more) = self.singletons.get(&Item::Ref(r)) {
              to_do.append(&mut more.clone());
            }
          }
        }
      }
    }
    result[chars.len()-1][0].to_owned()
  }
}

fn main() {
  let stdin = io::stdin();
  let input = Input::parse( & mut stdin.lock().lines()
    .map(|x| x.unwrap().trim().to_string()));
  for r in input.rules.values() {
    println!("rule: {:?}", r);
  }

  let mut count = 0;
  for str in &input.strings {
    let result = input.check(str).contains(&0);
    println!("{} = {}", str, result);
    if result {
      count += 1;
    }
  }
  println!("count = {}", count);
}
