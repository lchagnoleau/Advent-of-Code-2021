pub struct Day3_1<'a> {
    pub input: &'a str
}

impl Day3_1<'_> {
    pub fn result(&self) -> i32 {
        let v_input = read_input(&self.input);

        let mut gamma = String::from("");
        let mut epsilon = String::from("");
        let mut b_0 = 0;
        let mut b_1 = 0;
        let bits_number = v_input[0].len();

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
        let gamma_int = isize::from_str_radix(&gamma, 2).unwrap();
        let epsilon_int = isize::from_str_radix(&epsilon, 2).unwrap();

        let result: i32 = gamma_int as i32 * epsilon_int as i32;
        println!("Day 3 Part 1 result : {}", result);
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

    let t = Day3_1 {input: test_input};
    assert_eq!(t.result(), 198);
}