use std::collections::HashSet;
use std::fs;

fn read_input() -> String {
  return fs::read_to_string("./../../../data/day03.txt")
    .expect("Something went wrong reading the file");
}

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
struct Point {
  pub x: i64,
  pub y: i64,
}

impl Point {
  pub fn new(x: i64, y: i64) -> Self {
    Self { x, y }
  }
}

fn part_one() {
  // Get the challenge data
  let data = read_input();
  let data_vector: Vec<&str> = data.split("").collect();

  let mut houses: Vec<Point> = vec![];
  let mut current: Point = Point { x: 0, y: 0 };

  for i in data_vector {
    match i {
      "^" => current.y += 1,
      "v" => current.y -= 1,
      ">" => current.x += 1,
      "<" => current.x -= 1,
      _ => println!("no such case."),
    }
    houses.push(current);
  }

  let mut result = HashSet::new();
  for p in houses {
    if !result.contains(&p) {
      result.insert(p);
    }
  }

  println!("Result/part_one: {:?}", result.len());
}

fn part_two() {
  // Get the challenge data
  let data = read_input();
  let data_vector: Vec<&str> = data.split("").collect();

  let mut houses_santa: Vec<Point> = vec![];
  let mut houses_santa_robo: Vec<Point> = vec![];
  let mut current_santa: Point = Point { x: 0, y: 0 };
  let mut current_santa_robo: Point = Point { x: 0, y: 0 };

  for (pos, i) in data_vector.iter().enumerate() {
    if pos % 2 == 0 {
      // Robo-Santa (every second move)
      match i {
        &"^" => current_santa_robo.y += 1,
        &"v" => current_santa_robo.y -= 1,
        &">" => current_santa_robo.x += 1,
        &"<" => current_santa_robo.x -= 1,
        _ => println!("no such case."),
      }
      houses_santa_robo.push(current_santa_robo);
    } else {
      // Santa
      match i {
        &"^" => current_santa.y += 1,
        &"v" => current_santa.y -= 1,
        &">" => current_santa.x += 1,
        &"<" => current_santa.x -= 1,
        _ => println!("no such case."),
      }
      houses_santa.push(current_santa);
    }
  }

  let mut result = HashSet::new();
  houses_santa.append(&mut houses_santa_robo);
  for p in houses_santa {
    if !result.contains(&p) {
      result.insert(p);
    }
  }

  println!("Result/part_two: {:?}", result.len());
}

fn main() {
  part_one();
  part_two();
}
