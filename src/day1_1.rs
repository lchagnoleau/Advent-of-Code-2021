pub struct Day1_1<'a> {
    pub input: &'a str
}

impl Day1_1<'_> {
    pub fn result(&self) -> i32 {
        let v_input = read_input(&self.input);

        let mut nb_of_element_sup_as_prev = 0;
        for i in 1..v_input.len() {
            if v_input[i] > v_input[i-1] {
                nb_of_element_sup_as_prev += 1;
            }
        }
        println!("Day 1 Part 1 result : {}", nb_of_element_sup_as_prev);
        nb_of_element_sup_as_prev
    }
}

fn read_input(input_string: &str)-> Vec<i32> {
    input_string.to_string().split('\n').map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>()
}

#[test]
fn result_test() {
    let test_input: &str = r#"199
200
208
210
200
207
240
269
260
263"#;

    let t = Day1_1 {input: test_input};
    assert_eq!(t.result(), 7);
}
