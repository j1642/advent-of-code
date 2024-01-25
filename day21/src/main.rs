use day21::day21_1;//, day21_2};
use std::fs;
use std::time;

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

    let start = time::Instant::now();
    let ans_21_1 = day21_1(&text, 20);
    let elapsed = start.elapsed();
    println!("elapsed: {}ms", elapsed.as_millis());
    println!("day 21_1 = {}", ans_21_1);
    /*
    assert_eq!(ans_21_1, 812_609_846);

    // Part 2
    //env::set_var("RUST_BACKTRACE", "1");

    let ans_21_2 = day21_2(&text);
    println!("day 21_2 = {}", ans_21_2);
    println!("           4021 < ans");
    assert_eq!(ans_21_2, 134906214068564);
    */
}
