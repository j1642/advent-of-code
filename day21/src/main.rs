use day21::{day21_1, day21_2};
use std::fs;
//use std::time;

fn main() {
    let text = fs::read_to_string("day21.txt").expect("Should have read the file");
    let example1 = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";

    assert_eq!(day21_1(example1, 1), 2);
    assert_eq!(day21_1(example1, 2), 4);
    assert_eq!(day21_1(example1, 3), 6);
    assert_eq!(day21_1(example1, 6), 16);

    let ans_21_1 = day21_1(&text, 64);
    println!("day 21_1 = {}", ans_21_1);
    assert_eq!(ans_21_1, 3503);

    // Part 2
    //env::set_var("RUST_BACKTRACE", "1");
    assert_eq!(day21_2(example1, 6), 16);
    assert_eq!(day21_2(example1, 10), 50);
    assert_eq!(day21_2(example1, 50), 1594);
    /*
    assert_eq!(day21_2(example1, 100), 6536);
    assert_eq!(day21_2(example1, 500), 167004);
    assert_eq!(day21_2(example1, 1000), 668697);
    assert_eq!(day21_2(example1, 5000), 16733044);

    let ans_21_2 = day21_2(&text);
    println!("day 21_2 = {}", ans_21_2);
    println!("           4021 < ans");
    assert_eq!(ans_21_2, 134906214068564);
    */
}
