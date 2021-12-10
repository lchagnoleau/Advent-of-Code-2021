pub struct Day1_1<'a> {
    pub input: &'a str
}

impl Day1_1<'_> {
    pub fn result(&self) {
        let v_input = read_input(&self.input);

        let mut nb_of_element_sup_as_prev = 0;
        for i in 1..v_input.len() {
            if v_input[i] > v_input[i-1] {
                nb_of_element_sup_as_prev += 1;
            }
        }
        println!("Day 1 Part 1 result : {}", nb_of_element_sup_as_prev);
    }
}

fn read_input(input_string: &str)-> Vec<i32> {
    input_string.to_string().split('\n').map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>()
}

//#[test]
//fn result_test() {
//    let t = Day1_1
//    assert_eq!(2 + 2, 4);
//}
