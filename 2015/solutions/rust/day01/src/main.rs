use std::fs;

fn read_input() -> String {
  return fs::read_to_string("./../../../data/day01.txt")
    .expect("Something went wrong reading the file");
}

fn part_one() {

  // Get the challenge data
  let data = read_input();
  let data_vector: Vec<char> = data.chars().collect();

  let mut floor = 0;

  for element in data_vector.iter() {
    if element == &'(' {
      floor += 1;
    } else {
      floor -= 1;
    }
  }

  println!("Result/part_one: {}", floor);
}

fn part_two() {

  // Get the challenge data
  let data = read_input();
  let data_vector: Vec<char> = data.chars().collect();

  let mut floor = 0;
  let mut char_position = 0;
  for (position, element) in data_vector.iter().enumerate() {
    if floor == -1 {
      break;
    }

    if element == &'(' {
      floor += 1;
    } else {
      floor -= 1;
    }

    // The challenge states the position should start from 1.
    char_position = position + 1;
    println!("Loop/ floor: {}, position: {}, element: {}", floor, position, element);
  }

  println!("Result/part_two: {}", char_position);
}

fn main() {
  part_one();
  part_two();
}
