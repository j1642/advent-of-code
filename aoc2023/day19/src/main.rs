use day19::{day19_1, day19_2};
use std::fs;

fn main() {
    let text = fs::read_to_string("day19.txt").expect("Should have read the file");
    let example = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";

    assert_eq!(day19_1(example), 19114);
    let ans_19_1 = day19_1(&text);
    println!("day 19_1 = {}", ans_19_1);
    assert_eq!(ans_19_1, 401674);

    // Part 2
    //env::set_var("RUST_BACKTRACE", "1");
    assert_eq!(day19_2(example), 167409079868000);

    let ans_19_2 = day19_2(&text);
    println!("day 19_2 = {}", ans_19_2);
    assert_eq!(ans_19_2, 134906204068564);
}
