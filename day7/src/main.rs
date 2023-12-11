use day7::day_7_1;
use std::fs;

fn main() {
    let text = fs::read_to_string("day7.txt").expect("Should have read the file");
    let short_text = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    assert_eq!(day_7_1(short_text), 6440);
    /*
    assert_eq!(day_7_1("2345A 1
Q2KJJ 13
Q2Q2Q 19
T3T3J 17
T3Q33 11
2345J 3
J345A 2
32T3K 5
T55J5 29
KK677 7
KTJJT 34
QQQJA 31
JJJJJ 37
JAAAA 43
AAAAJ 59
AAAAA 61
2AAAA 23
2JJJJ 53
JJJJ2 41"), 6592);
*/
    //let ans_7_1 = day_7_1(&text);
    //assert_eq!(ans_7_1, );
    //println!("day 7_1 = {}", ans_7_1);
    //println!("too low:  252555683");

    /*
    // Part 2
    assert_eq!(day_7_2(short_text), 71503);

    let ans_7_2 = day_7_2(&text);
    assert_eq!(ans_7_2, 33_149_731);
    println!("day 7_2 = {}", ans_7_2);
    */
}
