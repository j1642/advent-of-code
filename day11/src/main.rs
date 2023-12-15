use day11::day_11_1; //, day_11_2};
use std::fs;

fn main() {
    let text = fs::read_to_string("day11.txt").expect("Should have read the file");
    let short_text = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    assert_eq!(day_11_1(short_text), 374);

    let ans_11_1 = day_11_1(&text);
    assert_eq!(ans_11_1, 9_543_156);
    println!("day 11_1 = {}", ans_11_1);

    // Part 2
    //assert_eq!(day_11_2(short_text), );
    /*
    //env::set_var("RUST_BACKTRACE", "1");

    let ans_11_2 = day_11_2(&text);
    assert_eq!(ans_11_2, 1208);
    println!("day 11_2 = {}", ans_11_2);
    */
}
