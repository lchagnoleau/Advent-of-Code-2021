use std::fs;

pub struct Day1_2 {
    pub input_file: String
}

impl Day1_2 {
    pub fn result(&self) {
        let v_input = read_input(&self.input_file);
        let mut three_measurement_sliding = vec![v_input[0] + v_input[1] + v_input[2]];

        for i in 1..v_input.len() {
            if i < v_input.len() - 2 {
                three_measurement_sliding.push(v_input[i] + v_input[i+1] + v_input[i+2]);
            }
            else if i < v_input.len() - 1 {
                three_measurement_sliding.push(v_input[i] + v_input[i+1]);
            }
            else {
                three_measurement_sliding.push(v_input[i]);
            }
        }

        let mut nb_of_element_sup_as_prev = 0;
        for i in 1..three_measurement_sliding.len() {
            if three_measurement_sliding[i] > three_measurement_sliding[i-1] {
                nb_of_element_sup_as_prev += 1;
            }
        }

        println!("Day 1 Part 2 result : {}", nb_of_element_sup_as_prev);
    }
}

fn read_input(filename: &str)-> Vec<i32> {
    let contents = fs::read_to_string(filename)
        .expect("Unable to open");

    contents.split('\n').map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>()
}
