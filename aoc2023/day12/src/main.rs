use day12::day_12_1; //, day_12_2};
use std::fs;

fn main() {
    let text = fs::read_to_string("day12.txt").expect("Should have read the file");
    let short_text = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

    assert_eq!(day_12_1(short_text), 21);

    //let ans_12_1 = day_12_1(&text);
    //assert_eq!(ans_12_1, 9_543_156);
    //println!("day 12_1 = {}", ans_12_1);

    /*
    // Part 2
    //env::set_var("RUST_BACKTRACE", "1");

    let ans_12_2 = day_12_2(&text);
    assert_eq!(ans_12_2, 625_243_292_686);
    println!("day 12_2 = {}", ans_12_2);
    */
}
