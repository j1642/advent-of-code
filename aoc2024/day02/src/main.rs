use day02::{day02_1, day02_2};
use std::fs;

fn main() {
    let text = fs::read_to_string("day02.txt").expect("Should have read the file");
    let test = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
    assert_eq!(day02_1(test), 2);

    let day2_1_ans = day02_1(&text);
    println!("day 2.1 = {day2_1_ans}");
    assert_eq!(day2_1_ans, 326);

    assert_eq!(day02_2(test), 4);
    /*
    let day02_2_ans = day02_2(&text);
    println!("day 2.2 = {day02_2_ans}");
    */
}
