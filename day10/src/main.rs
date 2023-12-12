use day10::day_10_1; //, day_10_2};
use std::fs;

fn main() {
    let text = fs::read_to_string("day10.txt").expect("Should have read the file");
    let short_text = ".....
.S-7.
.|.|.
.L-J.
.....
";

    assert_eq!(day_10_1(short_text), 4);

    let ans_10_1 = day_10_1(&text);
    assert_eq!(ans_10_1, 7093);
    println!("day 10_1 = {}", ans_10_1);

    // Part 2
    //assert_eq!(day_10_2(short_text), );
    //env::set_var("RUST_BACKTRACE", "1");

    /*
    let ans_10_2 = day_10_2(&text);
    assert_eq!(ans_10_2, 1208);
    println!("day 10_2 = {}", ans_10_2);
    */
}
