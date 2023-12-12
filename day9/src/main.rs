use day9::day_9_1;
//use std::env;
use std::fs;

fn main() {
    let text = fs::read_to_string("day9.txt").expect("Should have read the file");
    let short_text = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    assert_eq!(day_9_1(short_text), 114);

    let ans_9_1 = day_9_1(&text);
    assert_eq!(ans_9_1, 2_174_807_968);
    println!("day 9_1 = {}", ans_9_1);

    assert_eq!(day_9_2(short_text), 2);
    /*

    //env::set_var("RUST_BACKTRACE", "1");

    let ans_9_2 = day_9_2(&text);
    assert_eq!(ans_9_2, 12_324_145_107_121);
    println!("day 9_2 = {}", ans_9_2);
    */
}
