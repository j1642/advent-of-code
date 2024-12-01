use day01::{day01_1, day01_2};
use std::fs;

fn main() {
    let text = fs::read_to_string("day01.txt").expect("Should have read the file");
    let day1_1_ans = day01_1(&text);
    println!("day 1.1 = {day1_1_ans}");
    assert_eq!(day1_1_ans, 3574690);

    let day01_2_ans = day01_2(&text);
    println!("day 1.2 = {day01_2_ans}");
}
