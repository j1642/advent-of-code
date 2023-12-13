use day10::{day_10_1, day_10_2};
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
    assert_eq!(day_10_2(short_text), 1);
    //env::set_var("RUST_BACKTRACE", "1");
    let area_example = "..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
..........";
    assert_eq!(day_10_2(area_example), 4);
    let example_2 = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...
";
    assert_eq!(day_10_2(example_2), 8);
    let example_3 = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";
    assert_eq!(day_10_2(example_3), 10);

    let ans_10_2 = day_10_2(&text);
    assert_eq!(ans_10_2, 407);
    println!("day 10_2 = {}", ans_10_2);
}
