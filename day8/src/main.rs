use day8::day_8_1;
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

    /*
    // Part 2
    assert_eq!(day_8_2(short_text), 5905);

    let ans_8_2 = day_8_2(&text);
    //assert_eq!(ans_8_2, );
    println!("day 8_2 = {}", ans_8_2);
    */
}
