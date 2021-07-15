use fancy_regex::Regex;
use std::fs;

fn read_input() -> String {
  fs::read_to_string("./../../../data/day05.txt").expect("Something went wrong reading the file")
}

fn match_repeating_chars(text: &str) -> bool {
  let re = Regex::new(r"(.)\1+").unwrap();
  re.is_match(text).unwrap()
}

fn match_vowels(text: &str) -> bool {
  let re = Regex::new(r"[aeiou]").unwrap();
  re.find_iter(text).count() > 2
}

fn match_pair_repeat(text: &str) -> bool {
  let re = Regex::new(r"\b\w*(\w{2})\w*\1").unwrap();
  re.is_match(text).unwrap()
}

fn match_letter_between(text: &str) -> bool {
  let re = Regex::new(r"(.).\1").unwrap();
  re.is_match(text).unwrap()
}

fn part_one() {
  // Get the challenge data
  let data = read_input();
  let data_vector: Vec<&str> = data.split("\n").collect();

  let mut total_ft = 0;

  for el in data_vector.iter() {
    if !el.contains("ab") && !el.contains("cd") && !el.contains("pq") && !el.contains("xy") {
      if match_repeating_chars(el) {
        if match_vowels(el) {
          total_ft += 1;
        }
      }
    }
  }

  println!("Result/part_one: {:?}", total_ft);
}

fn part_two() {
  // Get the challenge data
  let data = read_input();
  let data_vector: Vec<&str> = data.split("\n").collect();

  let mut total_ft = 0;

  for el in data_vector.iter() {
    if match_pair_repeat(el) {
      if match_letter_between(el) {
        total_ft += 1;
      }
    }
  }

  println!("Result/part_two: {:?}", total_ft);
}

fn main() {
  part_one();
  part_two();
}
