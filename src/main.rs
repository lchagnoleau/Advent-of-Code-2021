use std::fs;

use crate::day1_1::Day1_1;
use crate::day1_2::Day1_2;
use crate::day2_1::Day2_1;
use crate::day2_2::Day2_2;
use crate::day3_1::Day3_1;
use crate::day3_2::Day3_2;

mod day1_1;
mod day1_2;
mod day2_1;
mod day2_2;
mod day3_1;
mod day3_2;

fn main() {
    let day1_input = fs::read_to_string("inputs/day1.txt")
        .expect("Unable to open");
    let day2_input = fs::read_to_string("inputs/day2.txt")
        .expect("Unable to open");
    let day3_input = fs::read_to_string("inputs/day3.txt")
        .expect("Unable to open");

    let day1_1 = Day1_1 {input: &day1_input};
    let day1_2 = Day1_2 {input: &day1_input};
    let day2_1 = Day2_1 {input: &day2_input};
    let day2_2 = Day2_2 {input: &day2_input};
    let day3_1 = Day3_1 {input: &day3_input};
    let day3_2 = Day3_2 {input: &day3_input};

    day1_1.result();
    day1_2.result();
    day2_1.result();
    day2_2.result();
    day3_1.result();
    day3_2.result();
}
