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
    assert_eq!(day21_2(example1, 100), 6536);

    lagrange_interpolation(example1);
    /*
    assert_eq!(day21_2(example1, 500), 167004);
    assert_eq!(day21_2(example1, 1000), 668697);
    assert_eq!(day21_2(example1, 5000), 16733044);

    let ans_21_2 = day21_2(&text, 26501365);
    println!("day 21_2 = {}", ans_21_2);
    assert_eq!(ans_21_2, );
    */
}

fn lagrange_interpolation(text: &str) {
    let (first_line, _) = text.split_once('\n').unwrap();
    let width_height = first_line.chars().count() as u64;

    let mut x_vals: Vec<u64> = vec![];
    for i in 0..6 {
        x_vals.push(width_height / 2 + width_height * i);
    }
    let mut y_vals: Vec<u64> = vec![];
    for x in &x_vals {
        y_vals.push(day21_2(text, *x as u32) as u64);
    }
    println!("y_vals: {:?}", y_vals);
    assert_eq!(x_vals.len(), y_vals.len());

    // solve for lagrange interp. with input 500
    let x = 500;
    let mut pdt_input_minus_x_vals = 1;
    for i in 0.. x_vals.len() {
        pdt_input_minus_x_vals *= x - x_vals[i]
    }
    let mut result: i64 = 0;

    for i in 0..x_vals.len() {
        let numerator = pdt_input_minus_x_vals / (x - x_vals[i]) * y_vals[i];

        let mut denom: i64 = 1;
        for j in 0..x_vals.len() {
            if j == i {
                continue;
            }
            denom *= x_vals[i] as i64 - x_vals[j] as i64;
        }
        if numerator < 0 {
            println!("i={i} neg numer");
        }
        if denom < 0 {
            println!("i={i} neg denom");
        }
        result += numerator as i64 / denom;
    }

    println!("{}", result);
}
