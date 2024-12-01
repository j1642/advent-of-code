use day14::{day_14_1, day_14_2};
use std::fs;

fn main() {
    let text = fs::read_to_string("day14.txt").expect("Should have read the file");
    let example = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

    assert_eq!(day_14_1(example), 136);

    let ans_14_1 = day_14_1(&text);
    assert_eq!(ans_14_1, 111339);
    println!("day 14_1 = {}", ans_14_1);

    // Part 2
    //env::set_var("RUST_BACKTRACE", "1");
    let example_ans_2 = day_14_2(example);
    assert_eq!(example_ans_2, 64);

    let ans_14_2 = day_14_2(&text);
    println!("day 14_2 = {}", ans_14_2);
    assert_eq!(ans_14_2, 93736);
}
