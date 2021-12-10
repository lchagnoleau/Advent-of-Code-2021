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
    let day1_1 = Day1_1 {input_file: String::from("inputs/day1.txt")};
    let day1_2 = Day1_2 {input_file: String::from("inputs/day1.txt")};
    let day2_1 = Day2_1 {input_file: String::from("inputs/day2.txt")};
    let day2_2 = Day2_2 {input_file: String::from("inputs/day2.txt")};
    let day3_1 = Day3_1 {input_file: String::from("inputs/day3.txt")};
    let day3_2 = Day3_2 {input_file: String::from("inputs/day3.txt")};

    day1_1.result();
    day1_2.result();
    day2_1.result();
    day2_2.result();
    day3_1.result();
    day3_2.result();
}
