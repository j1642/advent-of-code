use day22::day22_1; //, day22_2};
use std::fs;
//use std::time;

fn main() {
    let text = fs::read_to_string("day22.txt").expect("Should have read the file");
    let example1 = "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";

    assert_eq!(day22_1(example1), 5);

    let ans_22_1 = day22_1(&text);
    println!("day 22_1 = {}", ans_22_1);
    //assert_eq!(ans_22_1, 3503);

    /*
    // Part 2
    //env::set_var("RUST_BACKTRACE", "1");
    assert_eq!(day22_2(example1, 500), 167004);
    assert_eq!(day22_2(example1, 1000), 668697);
    assert_eq!(day22_2(example1, 5000), 16733044);

    let ans_22_2 = day22_2(&text, 26501365);
    println!("day 22_2 = {}", ans_22_2);
    assert_eq!(ans_22_2, );
    */
}
