use day8::{day_8_1, day_8_2};
//use std::env;
use std::fs;

fn main() {
    let text = fs::read_to_string("day8.txt").expect("Should have read the file");
    let short_text = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    assert_eq!(day_8_1(short_text), 6);

    let ans_8_1 = day_8_1(&text);
    //assert_eq!(ans_8_1, );
    println!("day 8_1 = {}", ans_8_1);

    // Part 2
    let short_text_2 = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

    assert_eq!(day_8_2(short_text_2), 6);

    //env::set_var("RUST_BACKTRACE", "1");

    let ans_8_2 = day_8_2(&text);
    assert_eq!(ans_8_2, 12_324_145_107_121);
    println!("day 8_2 = {}", ans_8_2);
}
