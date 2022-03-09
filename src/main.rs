use std::collections::{HashMap, HashSet};
use std::io;
use std::io::BufRead;

use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug)]
struct Food {
  ingredients: Vec<String>,
  allergens: Vec<String>,
}

impl Food {
  fn parse(input: &str) -> Self {
    lazy_static! {
      static ref FOOD_PATTERN: Regex =
         Regex::new(r"^(?P<ingredients>[^)]*)\s*\(contains\s+(?P<allergens>[^)]*)\)$").unwrap();
    }
    let food_match = FOOD_PATTERN.captures(input).unwrap();
    let ingredients = food_match.name("ingredients").unwrap().as_str().split_whitespace().map(|x| x.to_string()).collect();
    let allergens = food_match.name("allergens").unwrap().as_str().split(", ").map(|x| x.to_string()).collect();
    Food{ingredients, allergens}
  }
}

fn main() {
  let stdin = io::stdin();
  let foods: Vec<Food> = stdin.lock().lines()
    .map(|x| Food::parse(x.unwrap().trim())).collect();
  let mut possible_ingredients: HashMap<String, HashSet<String>> = HashMap::new();
  for f in &foods {
    let ingredients: HashSet<String> = f.ingredients.iter()
      .map(|s| s.to_string()).collect();
    for allergen in &f.allergens {
      if let Some(prev) = possible_ingredients.get_mut(allergen) {
        prev.retain(|i| ingredients.contains(i));
      } else {
        possible_ingredients.insert(allergen.clone(), ingredients.clone());
      }
    }
  }
  println!("possible = {:?}", possible_ingredients);
  let potentials: HashSet<String> = possible_ingredients.values()
    .flat_map(|s| s.iter())
    .map(|s| s.clone()).collect();
  let count = &foods.iter().flat_map(|f| f.ingredients.iter())
    .filter(|&i| !potentials.contains(i)).count();
  println!("Count = {}", count);
}
