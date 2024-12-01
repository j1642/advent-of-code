use day24::day24_1; //, day24_2};
use std::fs;

fn main() {
    let text = fs::read_to_string("day24.txt").expect("Should have read the file");
    let example = "19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3";

    // check for intersections within 7 <= (x, y) <= 27
    assert_eq!(day24_1(example, 7.0, 27.0), 2);

    let ans_24_1 = day24_1(&text, 200000000000000.0, 400000000000000.0);
    println!("day 24_1 = {}", ans_24_1);
    assert_eq!(ans_24_1, 21679);

    /*
    // Part 2
    //env::set_var("RUST_BACKTRACE", "1");
    assert_eq!(day24_2(example), 154);

    let ans_24_2 = day24_2(&text);
    println!("day 24_2 = {}", ans_24_2);
    assert_eq!(ans_24_2, 6546);
    */
}
