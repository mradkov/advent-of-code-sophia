use std::fs;
extern crate crypto;
use crypto::digest::Digest;
use crypto::md5::Md5;

fn read_input() -> String {
  return fs::read_to_string("./../../../data/day04.txt")
    .expect("Something went wrong reading the file");
}

fn part_one() {
  // Get the challenge data
  let data = read_input();
  let mut hasher = Md5::new();
  let mut i = 0;
  loop {
    hasher.input(data.as_bytes());
    hasher.input(i.to_string().as_bytes());

    let mut output = [0; 16];
    hasher.result(&mut output);

    if output[..2] == [0, 0] && output[2] <= 0x0F {
      println!("Result/part_one: {:?}", i);
      break;
    }
    hasher.reset();
    i += 1;
  }
}

fn part_two() {
  // Get the challenge data
  let data = read_input();
  let mut hasher = Md5::new();
  let mut i = 0;
  loop {
    hasher.input(data.as_bytes());
    hasher.input(i.to_string().as_bytes());

    let mut output = [0; 16];
    hasher.result(&mut output);

    if output[..3] == [0, 0, 0] {
      println!("Result/part_two: {:?}", i);
      break;
    }
    hasher.reset();
    i += 1;
  }
}

fn main() {
  part_one();
  part_two();
}
