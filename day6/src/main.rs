use day6::{day_6_1, day_6_2};
use std::fs;

fn main() {
    let text = fs::read_to_string("day6.txt").expect("Should have read the file");
    let short_text = "Time:      7  15   30
Distance:  9  40  200";

    assert_eq!(day_6_1(short_text), 288);

    let ans_6_1 = day_6_1(&text);
    assert_eq!(ans_6_1, 2_449_062);
    println!("day 6_1 = {}", ans_6_1);

    // Part 2
    assert_eq!(day_6_2(short_text), 71503);

    let ans_6_2 = day_6_2(&text);
    assert_eq!(ans_6_2, 33_149_631);
    println!("day 6_2 = {}", ans_6_2);
}
