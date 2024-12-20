use day06::day06_1;
use std::fs::read_to_string;
fn main() {
    let text = read_to_string("day06.txt").unwrap();
    let check = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
    assert_eq!(day06_1(check), 41);

    let day06_1_ans = day06_1(&text);
    assert_eq!(day06_1_ans, 5145);
    println!("6.1 = {}", day06_1_ans);
}
