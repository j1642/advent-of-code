// Advent of Code 2023 day 1.1
use std::fs;

fn main() {
    day_one_1()
}

fn day_one_1() {
    let text = fs::read_to_string("../../input/day1.1.txt")
        .expect("Should have read the file");

    let mut total = 0;
    let mut first_digit = 'x';
    let mut last_digit = 'x';

    for line in text.lines() {
        for (_, c) in line.char_indices() {
            if c.is_ascii_digit() {
                if first_digit == 'x' {
                    first_digit = c;
                }
                last_digit = c;
            }
        /*
            } else if text[i:i+3] == "one" || text[i:i+3] == "two" || text[i:i+3] == "six" {
                if first_digit == 'x' {
                    first_digit = item;
                }
                last_digit = item;
        */
        }
        let s = format!("{}{}", first_digit, last_digit);
        let num = s.parse::<i32>().unwrap();
        total += num;
        first_digit = 'x';
        last_digit = 'x';
    }
    println!("total={total}");
}
