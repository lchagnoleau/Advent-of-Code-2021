pub struct Day3_2<'a> {
    pub input: &'a str
}

impl Day3_2<'_> {
    pub fn result(&self) -> i32 {
        let v_input = read_input(&self.input);
        let bits_number = v_input[0].len();

        let mut v_oxy = v_input.clone();
        let mut v_co2 = v_input.clone();

        let mut v_oxy_0: Vec<String> = Vec::new();
        let mut v_oxy_1: Vec<String> = Vec::new();
        let mut v_co2_0: Vec<String> = Vec::new();
        let mut v_co2_1: Vec<String> = Vec::new();

        for i in 0..bits_number {
            if v_oxy.len() > 1 {
                for v in &v_oxy {
                    match v.chars().nth(i).unwrap() {
                        '0' => v_oxy_0.push(v.to_owned()),
                        '1' => v_oxy_1.push(v.to_owned()),
                        _  => println!("Not considered")
                    }
                }
                v_oxy.clear();
                if v_oxy_0.len() > v_oxy_1.len() {
                    v_oxy = v_oxy_0.clone();
                }
                else if v_oxy_1.len() > v_oxy_0.len() {
                    v_oxy = v_oxy_1.clone();
                }
                else {
                    v_oxy = v_oxy_1.clone();
                }
                v_oxy_0.clear();
                v_oxy_1.clear();
            }

            if v_co2.len() > 1 {
                for v in &v_co2 {
                    match v.chars().nth(i).unwrap() {
                        '0' => v_co2_0.push(v.to_owned()),
                        '1' => v_co2_1.push(v.to_owned()),
                        _  => println!("Not considered")
                    }
                }
                v_co2.clear();
                if v_co2_0.len() < v_co2_1.len() {
                    v_co2 = v_co2_0.clone();
                }
                else if v_co2_1.len() < v_co2_0.len() {
                    v_co2 = v_co2_1.clone();
                }
                else {
                    v_co2 = v_co2_0.clone();
                }
                v_co2_0.clear();
                v_co2_1.clear();
            }
        }

        let oxy = isize::from_str_radix(&v_oxy[0], 2).unwrap();
        let co2 = isize::from_str_radix(&v_co2[0], 2).unwrap();

        let result = oxy as i32 * co2 as i32;

        println!("Day 3 Part 2 result : {}", result);
        result
    }
}

fn read_input(input_string: &str)-> Vec<String> {
    input_string.split('\n').map(|s| s.to_string()).collect()
}

#[test]
fn result_test() {
    let test_input: &str = r#"00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010"#;

    let t = Day3_2 {input: test_input};
    assert_eq!(t.result(), 230);
}