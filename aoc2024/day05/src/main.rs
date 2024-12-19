use day05::{day05_1, day05_2};
use std::fs;

fn main() {
    let text = fs::read_to_string("day05.txt").unwrap();
    let day05_1_ans = day05_1(&text);
    assert_eq!(day05_1_ans, 5509);
    println!("5.1 = {day05_1_ans}");

    let check = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    let check_ans = day05_1(check);
    assert_eq!(check_ans, 143);

    let day05_2_check = day05_2(&check);
    assert_eq!(day05_2_check, 123);

    let day05_2_ans = day05_2(&text);
    assert_eq!(day05_2_ans, 4407);
    println!("5.2 = {day05_2_ans}");
}
