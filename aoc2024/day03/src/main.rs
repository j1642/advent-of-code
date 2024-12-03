use day03::{day03_1, day03_2};
use std::fs;

fn main() {
    let text = fs::read_to_string("day03.txt").expect("Should have read the file");

    let day3_1_ans = day03_1(&text);
    println!("day 3.1 = {day3_1_ans}");
    assert_eq!(day3_1_ans, 173731097);

    let test = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
    assert_eq!(day03_2(test), 48);
    /*
    let day03_3_ans = day03_3(&text);
    println!("day 3.3 = {day03_3_ans}");
    assert_eq!(day03_3_ans, 381);
    */
}
