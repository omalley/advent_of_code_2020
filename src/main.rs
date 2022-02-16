use std::collections::HashMap;
use std::io;
use std::io::BufRead;

#[derive(Debug)]
struct Passport {
  fields: HashMap<String, String>,
}

impl Passport {
  fn parse(lines: &mut dyn Iterator<Item=String>) -> Option<Self> {
    let mut fields: HashMap<String, String> = HashMap::new();
    loop {
      match lines.next() {
        Some(line) => {
          if line.is_empty() {
            break
          }
          for word in line.split_ascii_whitespace() {
            let mut parts = word.split(":");
            fields.insert(String::from(parts.next().unwrap()),
                          String::from(parts.next().unwrap()));
          }
        }
        None => break,
      }
    }
    if fields.is_empty() {
      None
    } else {
      Some(Passport{fields})
    }
  }

  const FIELDS: &'static[&'static str] = &["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

  fn is_valid(&self) -> bool {
    let mut count = 0;
    for &k in Self::FIELDS {
      if self.fields.contains_key(k) {
        count += 1;
      }
    }
    count == Self::FIELDS.len()
  }
}

fn main() {
  let stdin = io::stdin();
  let mut input = stdin.lock().lines()
    .map(|x| String::from(x.unwrap().trim()));
  let mut count = 0;
  while let Some(passport) = Passport::parse(&mut input) {
    if passport.is_valid() {
      count += 1;
    }
  }
  println!("{}", count);
}
