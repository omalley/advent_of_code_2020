use std::collections::HashMap;
use std::io;
use std::io::BufRead;

use lazy_static::lazy_static;
use regex::Regex;

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

  fn is_valid_year(year: Option<&String>, lower: i64, upper: i64) -> bool {
    lazy_static! {
      static ref YEAR_PATTERN: Regex = Regex::new(r"^\d{4}$").unwrap();
    }
    if let Some(yr) = year {
      if YEAR_PATTERN.is_match(yr) {
        let num = yr.parse::<i64>().unwrap();
        return lower <= num && num <= upper
      }
    }
    false
  }

  fn is_valid_height(str: Option<&String>) -> bool {
    lazy_static! {
      static ref HEIGHT_PATTERN: Regex = Regex::new(r"^(?P<num>\d+)(?P<unit>cm|in)$").unwrap();
    }
    if let Some(s) = str {
      if let Some(captures) = HEIGHT_PATTERN.captures(s) {
        let num = captures.name("num").unwrap().as_str().parse::<i64>().unwrap();
        let unit = captures.name("unit").unwrap().as_str();
        match unit {
          "in" => return 59 <= num && num <= 76,
          "cm" => return 150 <= num && num <= 193,
          _ => {}
        }
      }
    }
    false
  }

  fn matches_regex(str: Option<&String>, regex: &Regex) -> bool {
    if let Some(s) = str {
      return regex.is_match(s)
    }
    false
  }

  fn is_valid(&self) -> bool {
    lazy_static! {
      static ref HAIR_PATTERN: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
      static ref EYE_PATTERN: Regex = Regex::new(r"^amb|blu|brn|gry|grn|hzl|oth$").unwrap();
      static ref PASSPORT_PATTERN: Regex = Regex::new(r"^\d{9}$").unwrap();
    }
    if !Self::is_valid_year(self.fields.get("byr"), 1920, 2002) {
      return false
    }
    if !Self::is_valid_year(self.fields.get("iyr"), 2010, 2020) {
      return false
    }
    if !Self::is_valid_year(self.fields.get("eyr"), 2020, 2030) {
      return false
    }
    if !Self::is_valid_height(self.fields.get("hgt")) {
      return false
    }
    if !Self::matches_regex(self.fields.get("hcl"), &HAIR_PATTERN) {
      return false
    }
    if !Self::matches_regex(self.fields.get("ecl"), &EYE_PATTERN) {
      return false
    }
    if !Self::matches_regex(self.fields.get("pid"), &PASSPORT_PATTERN) {
      return false
    }
    true
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
