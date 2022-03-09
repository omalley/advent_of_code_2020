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
  // build a map of ingredient to allergen by picking any allergen that has a single
  // ingredient and then removing it from the others
  let mut contains: HashMap<String, String> = HashMap::new();
  while !possible_ingredients.is_empty() {
    let mut new_bindings : Vec<(String,String)> = Vec::new();
    for (k, v) in &possible_ingredients {
      if v.len() == 1 {
        new_bindings.push((k.clone(), v.iter().next().unwrap().clone()))
      }
    }
    for (a,i) in new_bindings {
      contains.insert(i, a.clone());
      possible_ingredients.remove(&a);
    }
    // remove the ingredient from the other allergens
    for allergens in &mut possible_ingredients.values_mut() {
      allergens.retain(|i| !contains.contains_key(i));
    }
  }
  // Now we need to sort by allergen
  let mut contains: Vec<(String,String)> = contains.iter()
    .map(|(i, a)| (a.clone(), i.clone())).collect();
  contains.sort_unstable();
  // and print out the ingredients
  let mut first = true;
  for (_, i) in contains {
    print!("{}{}", if first {""} else {","}, i);
    first = false;
  }
  println!();
}
