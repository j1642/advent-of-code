use day04::{day04_1, day04_2};
use std::fs;

fn main() {
    let text = fs::read_to_string("day04.txt").unwrap();
    let check = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
    assert_eq!(day04_1(check), 18);

    let day04_1_ans = day04_1(&text);
    println!("day 4.1 = {}", day04_1_ans);
    assert_eq!(day04_1_ans, 2599);

    assert_eq!(day04_2(check), 9);
    let day04_2_ans = day04_2(&text);
    println!("day 4.2 = {}", day04_2_ans);
    assert_eq!(day04_2_ans, 1948);
}
