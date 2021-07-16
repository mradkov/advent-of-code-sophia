use std::fs;

fn read_input() -> String {
  fs::read_to_string("./../../../data/day06.txt").expect("Something went wrong reading the file")
}

fn part_one() {
  // Get the challenge data
  let data = read_input();
  let data_vector: Vec<&str> = data.split("\n").collect();

  let mut vec = vec![vec![false; 1000]; 1000];

  for el in data_vector.iter() {
    let action: Vec<&str> = el.split(" ").collect();
    match action[0] {
      "turn" => {
        let point_start: Vec<usize> = action[2]
          .split(",")
          .map(|x| x.parse::<usize>().unwrap())
          .collect();
        let point_end: Vec<usize> = action[4]
          .split(",")
          .map(|x| x.parse::<usize>().unwrap())
          .collect();

        if action[1].contains("on") {
          // on
          for x in point_start[0]..=point_end[0] {
            for y in point_start[1]..=point_end[1] {
              vec[x][y] = true;
            }
          }
        } else {
          // off
          for x in point_start[0]..=point_end[0] {
            for y in point_start[1]..=point_end[1] {
              vec[x][y] = false;
            }
          }
        }
      }
      "toggle" => {
        let point_start: Vec<usize> = action[1]
          .split(",")
          .map(|x| x.parse::<usize>().unwrap())
          .collect();
        let point_end: Vec<usize> = action[3]
          .split(",")
          .map(|x| x.parse::<usize>().unwrap())
          .collect();

        for x in point_start[0]..=point_end[0] {
          for y in point_start[1]..=point_end[1] {
            vec[x][y] = !vec[x][y];
          }
        }
      }
      _ => println!("Nothing"),
    }
  }

  let mut total_lights_on = 0;
  for x in 0..=999 {
    for y in 0..=999 {
      if vec[x][y] == true {
        total_lights_on += 1;
      }
    }
  }

  println!("Result/part_one: {:?}", total_lights_on);
}

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
struct Light {
  pub state: bool,
  pub brightness: i32,
}

impl Light {
  pub fn new(state: bool, brightness: i32) -> Self {
    Self { state, brightness }
  }
}

fn part_two() {
  // Get the challenge data
  let data = read_input();
  let data_vector: Vec<&str> = data.split("\n").collect();

  let mut vec = vec![vec![Light::new(false, 0); 1000]; 1000];

  for el in data_vector.iter() {
    let action: Vec<&str> = el.split(" ").collect();
    match action[0] {
      "turn" => {
        let point_start: Vec<usize> = action[2]
          .split(",")
          .map(|x| x.parse::<usize>().unwrap())
          .collect();
        let point_end: Vec<usize> = action[4]
          .split(",")
          .map(|x| x.parse::<usize>().unwrap())
          .collect();

        if action[1].contains("on") {
          // on
          for x in point_start[0]..=point_end[0] {
            for y in point_start[1]..=point_end[1] {
              vec[x][y].state = true;
              vec[x][y].brightness += 1;
            }
          }
        } else {
          // off
          for x in point_start[0]..=point_end[0] {
            for y in point_start[1]..=point_end[1] {
              vec[x][y].state = false;
              if vec[x][y].brightness >= 1 {
                vec[x][y].brightness -= 1;
              }
            }
          }
        }
      }
      "toggle" => {
        let point_start: Vec<usize> = action[1]
          .split(",")
          .map(|x| x.parse::<usize>().unwrap())
          .collect();
        let point_end: Vec<usize> = action[3]
          .split(",")
          .map(|x| x.parse::<usize>().unwrap())
          .collect();

        for x in point_start[0]..=point_end[0] {
          for y in point_start[1]..=point_end[1] {
            vec[x][y].state = !vec[x][y].state;
            vec[x][y].brightness += 2;
          }
        }
      }
      _ => println!("Nothing"),
    }
  }

  let mut total_brightness = 0;
  for x in 0..=999 {
    for y in 0..=999 {
      total_brightness += vec[x][y].brightness;
    }
  }

  println!("Result/part_two: {:?}", total_brightness);
}

fn main() {
  part_one();
  part_two();
}
