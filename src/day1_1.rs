use std::fs;

pub struct Day1_1 {
    pub input_file: String
}

impl Day1_1 {
    pub fn result(&self) {
        let v_input = read_input(&self.input_file);

        let mut nb_of_element_sup_as_prev = 0;
        for i in 1..v_input.len() {
            if v_input[i] > v_input[i-1] {
                nb_of_element_sup_as_prev += 1;
            }
        }
        println!("Day 1 Part 1 result : {}", nb_of_element_sup_as_prev);
    }
}

fn read_input(filename: &str)-> Vec<i32> {
    let contents = fs::read_to_string(filename)
        .expect("Unable to open");

    contents.split('\n').map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>()
}
