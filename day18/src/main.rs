use day18::day_18_1; //, day_18_2};
use std::fs;

fn main() {
    let text = fs::read_to_string("day18.txt").expect("Should have read the file");
    let example = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";

    assert_eq!(day_18_1(example), 62);

    let ans_18_1 = day_18_1(&text);
    println!("day 18_1 = {}", ans_18_1);
    assert_eq!(ans_18_1, 40761);

    /*
    // Part 2
    //env::set_var("RUST_BACKTRACE", "1");
    //assert_eq!(day_18_2(example), );

    let ans_18_2 = day_18_2(&text);
    println!("day 18_2 = {}", ans_18_2);
    //assert_eq!(ans_18_2, 7154);
    */
}
