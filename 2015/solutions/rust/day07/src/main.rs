// use regex::Regex;
use std::collections::HashMap;
use std::fs;

fn read_input() -> String {
  fs::read_to_string("./../../../data/day07.txt").expect("Something went wrong reading the file")
}

pub struct Circuit {
  pub wires: HashMap<String, i32>,
}

impl Circuit {
  pub fn new() -> Circuit {
    Circuit {
      wires: HashMap::new(),
    }
  }

  // TODO: finish the approach with regex;

  // pub fn parse_line_regex(&mut self, input: &str) -> i32 {
  //   let match_operator = Regex::new(r".*(AND|OR|LSHIFT|RSHIFT|NOT).*").unwrap();
  //   let match_assign = Regex::new(r"^(?P<from>.+) -> (?P<to>.+)$").unwrap();
  //   let match_operator_not = Regex::new(r"^NOT (?P<from>.+) -> (?P<to>.+)$+").unwrap();
  //   let match_operator_rest =
  //     Regex::new(r"^(?P<first>.+) (?P<operator>.+) (?P<second>.+) -> (?P<to>.+)$").unwrap();
  //   let mut problems = 0;
  //   match match_operator.captures(input) {
  //     Some(op) => match op.name("operator").unwrap().as_str() {
  //       "NOT" => match match_operator_not.captures(input) {
  //         Some(x) => {
  //           let from = x.name("from").unwrap().as_str();
  //           let to = x.name("to").unwrap().as_str();
  //           println!("NOT: from {} ->  to {}", from, to);

  //           match self.wires.get(to) {
  //             Some(x) => {
  //               // println!("NOT: from {} ->  to {}", from, to);

  //               // println!("{}", signals.get(from).unwrap());
  //               // signals.insert(to, !*signals.get(from).unwrap());

  //               todo!();
  //             }
  //             None => {
  //               if self.wires.contains_key(from) {
  //                 self.wires.entry(to).or_insert(*signals.get(from).unwrap());
  //               }
  //             }
  //           }
  //         }
  //         None => unreachable!(),
  //       },
  //       _ => match match_operator_rest.captures(input) {
  //         Some(x) => {
  //           let first = x.name("first").unwrap().as_str();
  //           let second = x.name("second").unwrap().as_str();
  //           let to = x.name("to").unwrap().as_str();
  //           let operator = x.name("operator").unwrap().as_str();
  //           println!(
  //             "OPERATOR: {}, FIRST: {}, SECOND: {}, TO: {}",
  //             operator, first, second, to
  //           );
  //         }
  //         None => unreachable!(),
  //       },
  //     },
  //     None => {
  //       // match assign
  //       match match_assign.captures(input) {
  //         Some(x) => {
  //           let from = x.name("from").unwrap().as_str();
  //           let to = x.name("to").unwrap().as_str();
  //           println!("From: {:?} -> To: {:?}", from, to);
  //           match from.parse::<i32>() {
  //             Ok(n) => *self.wires.entry(to).or_insert(0) = n,
  //             Err(_) => {
  //               if self.wires.contains_key(&from) {
  //                 *self.wires.entry(to).or_insert(0) = self.wires[&from];
  //               } else {
  //                 problems += 1
  //               }
  //             }
  //           }
  //         }
  //         None => println!("Problem"),
  //       }
  //     }
  //   }
  // }

  pub fn parse_line(&mut self, input: &str) -> i32 {
    let mut problems = 0;

    let mut args = input
      .split_whitespace()
      .map(str::trim)
      .collect::<Vec<&str>>();

    let right_hand_side = args.pop().unwrap();
    let _ = args.pop();

    if args.len() == 1 {
      match args[0].parse::<i32>() {
        Ok(n) => *self.wires.entry(right_hand_side.to_string()).or_insert(0) = n,
        Err(_) => {
          if self.wires.contains_key(args[0]) {
            *self.wires.entry(right_hand_side.to_string()).or_insert(0) = self.wires[args[0]];
          } else {
            problems += 1
          }
        }
      };
    } else if args.len() == 2 {
      match args[1].parse::<i32>() {
        Ok(n) => {
          *self
            .wires
            .entry(right_hand_side.trim().to_string())
            .or_insert(0) = !n
        }
        Err(_) => {
          if self.wires.contains_key(args[1]) {
            *self.wires.entry(right_hand_side.to_string()).or_insert(0) = !self.wires[args[1]];
          } else {
            problems += 1;
          }
        }
      };
    } else {
      let arg_1 = args[0].parse::<i32>();
      let mut result1 = 0;
      if arg_1.is_ok() {
        result1 = arg_1.unwrap();
      } else if self.wires.contains_key(args[0]) {
        result1 = self.wires[args[0]];
      } else {
        problems += 1;
      }

      let arg_2 = args[2].parse::<i32>();
      let mut result2 = 0;
      if arg_2.is_ok() {
        result2 = arg_2.unwrap();
      } else if self.wires.contains_key(args[2]) {
        result2 = self.wires[args[2]];
      } else {
        problems += 1;
      }

      if problems == 0 {
        *self.wires.entry(right_hand_side.to_string()).or_insert(0) = match args[1] {
          "AND" => result1 & result2,
          "OR" => result1 | result2,
          "LSHIFT" => result1 << result2,
          "RSHIFT" => result1 >> result2,
          _ => panic!("Unexpected pattern found ... "),
        };
      }
    }

    problems
  }
}

fn part_one() -> i32 {
  // Get the challenge data
  let data = read_input();
  let mut data_vector: Vec<&str> = data.split("\n").collect();
  data_vector.sort();

  let mut circuit = Circuit::new();
  let mut total = -1;
  while total != 0 {
    total = 0;
    for line in data_vector.iter() {
      total += circuit.parse_line(line);
    }
  }
  let part1 = circuit.wires["a"];
  println!("Result/part_one: {:?}", part1);
  part1
}

fn part_two(input: i32) {
  // Get the challenge data
  let data = read_input();
  let mut data_vector: Vec<&str> = data.split("\n").collect();
  data_vector.sort();

  let mut circuit = Circuit::new();
  circuit.wires.insert("b".to_string(), input);
  let mut total = -1;
  while total != 0 {
    total = 0;
    for line in data_vector.iter() {
      if !line.contains("19138 -> b") {
        total += circuit.parse_line(line);
      }
    }
  }

  println!("Result/part_two: {:?}", circuit.wires["a"]);
}

fn main() {
  let part1 = part_one();
  part_two(part1);
}
