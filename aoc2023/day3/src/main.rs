use day3::{day_3_1, day_3_2};
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
    assert_eq!(
        day_3_1(
            "12.......*..
+.........34
.......-12..
..78........
..*....60...
78.........9
.5.....23..$
8...90*12...
............
2.2......12.
.*.........*
1.1..503+.56"
        ),
        925
    );

    let ans_3_1 = day_3_1(&text);
    assert_eq!(ans_3_1, 517021);
    println!("day 3_1 = {}", ans_3_1);

    // Part 2
    assert_eq!(day_3_2(short_text), 467835);
    let ans_3_2 = day_3_2(&text);
    assert_eq!(ans_3_2, 81_296_995);
    println!("day 3_2 = {}", ans_3_2);
}
