pub struct Day2_1<'a> {
    pub input: &'a str
}

impl Day2_1<'_> {
    pub fn result(&self) -> i32 {
        let v_input = read_input(&self.input);
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
        let result = h_position * d_position;
        println!("Day 2 Part 1 result : {}", result);
        result
    }
}

fn read_input(input_string: &str) -> Vec<(String, i32)> {
    let v_lines = input_string.split('\n').collect::<Vec<&str>>();

    let mut v_cmd = Vec::new();
    for line in v_lines {
        let split = line.split(' ').collect::<Vec<&str>>();

        let cmd = split[0];
        let value = split[1].parse::<i32>().unwrap();

        v_cmd.push((cmd.to_owned(), value));
    }

    v_cmd
}

#[test]
fn result_test() {
    let test_input: &str = r#"forward 5
down 5
forward 8
up 3
down 8
forward 2"#;

    let t = Day2_1 {input: test_input};
    assert_eq!(t.result(), 150);
}