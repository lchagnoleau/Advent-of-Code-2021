use std::fs;

pub struct Day2_1 {
    pub input_file: String
}

impl Day2_1 {
    pub fn result(&self) {
        let v_input = read_input(&self.input_file);
        let mut h_position = 0i32; // Horizontal position
        let mut d_position = 0i32; // Depth position
        for (cmd, value) in &v_input {
            match cmd.as_str() {
                "forward" => h_position += value,
                "up" => d_position -= value,
                "down" => d_position += value,
                _ => println!("cmd {} is not considered", cmd)
            }
        }
        println!("Day 2 Part 1 result : {}", h_position * d_position);
    }
}

fn read_input(filename: &str) -> Vec<(String, i32)> {
    let contents = fs::read_to_string(filename)
        .expect("Unable to open");

    let v_lines = contents.split('\n').collect::<Vec<&str>>();

    let mut v_cmd = Vec::new();
    for line in v_lines {
        let split = line.split(' ').collect::<Vec<&str>>();

        let cmd = split[0];
        let value = split[1].parse::<i32>().unwrap();

        v_cmd.push((cmd.to_owned(), value));
    }

    v_cmd
}
