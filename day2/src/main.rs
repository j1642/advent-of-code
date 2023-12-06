use day2::{day_2_1, day_2_2};
use std::fs;

fn main() {
    let text = fs::read_to_string("day2.txt").expect("Should have read the file");
    let short_text = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    let day2_1_short = day_2_1(short_text);
    println!("day 2.1 (short) expected=8, got={day2_1_short}");
    let day2_1_ans = day_2_1(&text);
    println!("day 2.1 = {day2_1_ans}");

    let day2_2_short = day_2_2(short_text);
    println!("day 2.2 (short) expected=2286, got={day2_2_short}");
    let day2_2_ans = day_2_2(&text);
    println!("day 2.2 = {day2_2_ans}");
}
