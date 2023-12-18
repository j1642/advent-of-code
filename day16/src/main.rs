use day16::{day_16_1, day_16_2};
use std::fs;

fn main() {
    let text = fs::read_to_string("day16.txt").expect("Should have read the file");
    let example = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";

    assert_eq!(day_16_1(example, ('e', 0, 0)), 46);

    let ans_16_1 = day_16_1(&text, ('e', 0, 0));
    assert_eq!(ans_16_1, 6795);
    println!("day 16_1 = {}", ans_16_1);

    // Part 2
    //env::set_var("RUST_BACKTRACE", "1");
    assert_eq!(day_16_2(example), 51);

    let ans_16_2 = day_16_2(&text);
    println!("day 16_2 = {}", ans_16_2);
    assert_eq!(ans_16_2, 7154);
}
