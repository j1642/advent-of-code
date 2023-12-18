use day15::{day_15_1, day_15_2};
use std::fs;

fn main() {
    let text = fs::read_to_string("day15.txt").expect("Should have read the file");
    let example = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    assert_eq!(day_15_1(example), 1320);

    let ans_15_1 = day_15_1(&text);
    assert_eq!(ans_15_1, 507291);
    println!("day 15_1 = {}", ans_15_1);

    // Part 2
    //env::set_var("RUST_BACKTRACE", "1");
    assert_eq!(day_15_2(example), 145);

    let ans_15_2 = day_15_2(&text);
    println!("day 15_2 = {}", ans_15_2);
    assert_eq!(ans_15_2, 296921);
}
