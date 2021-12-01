use std::fs;

fn main() {
  let v_input = read_input("input.txt");

  let mut nb_of_element_sup_as_prev = 0;
  for i in 1..v_input.len() {
    if v_input[i] > v_input[i-1] {
      nb_of_element_sup_as_prev += 1;
    }
  }

  println!("Number of numbers : {}", v_input.len());
  println!("Number of numbers superior as previous : {}", nb_of_element_sup_as_prev);
}

fn read_input(filename: &str)-> Vec<i32> {
  println!("In file {}", filename);

  let contents = fs::read_to_string(filename)
    .expect("Unable to open");

  contents.split('\n').map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>()
}