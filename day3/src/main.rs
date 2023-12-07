use day3::day_3_1;
use std::fs;

fn main() {
    let text = fs::read_to_string("day3.txt").expect("Should have read the file");
    let short_text = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    assert_eq!(day_3_1(short_text), 4361);
    //assert_eq!(day_3_1(&text), 517021);
    println!("{}", day_3_1(&text));
}
