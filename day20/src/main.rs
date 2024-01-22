use day20::day20_1; //, day20_2};
use std::fs;

fn main() {
    let _text = fs::read_to_string("day20.txt").expect("Should have read the file");
    let example1 = "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";

    let example2 = "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";

    assert_eq!(day20_1(example1), 32000000);
    //assert_eq!(day20_1(example2), 11687500);
    //let ans_20_1 = day20_1(&text);
    //println!("day 20_1 = {}", ans_20_1);
    //assert_eq!(ans_20_1, 401674);

    /*
    // Part 2
    //env::set_var("RUST_BACKTRACE", "1");
    assert_eq!(day20_2(example), 167409079868000);

    let ans_20_2 = day20_2(&text);
    println!("day 20_2 = {}", ans_20_2);
    assert_eq!(ans_20_2, 134906204068564);
    */
}
