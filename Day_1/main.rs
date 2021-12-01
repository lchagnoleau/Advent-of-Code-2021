use std::fs;

fn main() {
  let v_input = read_input("input.txt");
  for s in v_input {
    println!("{}", s)
  }
}

fn read_input(filename: &str)-> Vec<i32> {
  println!("In file {}", filename);

  let contents = fs::read_to_string(filename)
    .expect("Unable to open");

  contents.split('\n').map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>()
}