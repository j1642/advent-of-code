use aoc::{day_1_1, day_1_2};
use std::fs;

fn main() {
    let text = fs::read_to_string("day1.txt").expect("Should have read the file");
    let _short_text = "sevenine
        ";
    let day1_1_ans = day_1_1(&text);
    println!("day 1.1 ={day1_1_ans}");
    let day1_2_ans = day_1_2(&text);
    println!("day 1.2 ={day1_2_ans}");
    let day1_2_ans = day_1_2(_short_text);
    println!("day 1.2 ={day1_2_ans}");
}
