use std::fs;

fn read_input() -> String {
  return fs::read_to_string("./../../../data/day02.txt")
    .expect("Something went wrong reading the file");
}

fn calculate_wrapping(length: i32, width: i32, height: i32) -> i32 {
  let area = vec![2 * length * width, 2 * width * height, 2 * height * length];
  let min = area.iter().min().unwrap() / 2;
  let sum: i32 = area.iter().sum();
  let result = sum + min;
  return result;
}

fn part_one() {
  // Get the challenge data
  let data = read_input();
  let data_vector: Vec<&str> = data.split("\n").collect();

  let mut total_ft = 0;

  for el in data_vector.iter() {
    let size: Vec<i32> = el.split("x").map(|x| x.parse::<i32>().unwrap()).collect();
    total_ft += calculate_wrapping(size[0], size[1], size[2]);
  }

  println!("Result/part_one: {}", total_ft);
}

fn calculate_ribbon_and_bow(length: i32, width: i32, height: i32) -> i32 {
  let area = vec![2 * length, 2 * width, 2 * height];
  let max = area.iter().max().unwrap();
  let sum: i32 = area.iter().sum();
  let bow = length * width * height;
  let result = sum - max + bow;
  return result;
}

fn part_two() {
  // Get the challenge data
  let data = read_input();
  let data_vector: Vec<&str> = data.split("\n").collect();

  let mut total_ft = 0;

  for el in data_vector.iter() {
    let size: Vec<i32> = el.split("x").map(|x| x.parse::<i32>().unwrap()).collect();
    total_ft += calculate_ribbon_and_bow(size[0], size[1], size[2]);
  }

  println!("Result/part_two: {}", total_ft);
}

fn main() {
  part_one();
  part_two();
}
