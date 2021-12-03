use std::fs;

fn main() {
  let mut gamma = String::from("");
  let mut epsilon = String::from("");
  let mut b_0 = 0;
  let mut b_1 = 0;

  let v_input = read_input("input.txt");
  let bits_number = v_input[0].len();

  println!("Number of bits : {}", bits_number);

  for i in 0..bits_number {
    for v in &v_input {
      match v.chars().nth(i).unwrap() {
        '0' => b_0 += 1,
        '1' => b_1 += 1,
        _   => println!("Not considered")
      }
    }
    if b_0 > b_1 {
      gamma.push('0');
      epsilon.push('1');
    }
    else{
      gamma.push('1');
      epsilon.push('0');
    }
    b_0 = 0;
    b_1 = 0;
  }

  println!("Gamma : {}", gamma);
  println!("Epsilon : {}", epsilon);

  let gamma_int = isize::from_str_radix(&gamma, 2).unwrap();
  let epsilon_int = isize::from_str_radix(&epsilon, 2).unwrap();

  println!("Gamma int: {}", gamma_int);
  println!("Epsilon int: {}", epsilon_int);

  println!("Power = {}", gamma_int * epsilon_int);
}

fn read_input(filename: &str)-> Vec<String> {
  println!("In file {}", filename);

  let contents = fs::read_to_string(filename)
    .expect("Unable to open");

  contents.split('\n').map(|s| s.to_string()).collect()
}