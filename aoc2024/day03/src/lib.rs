use regex::Regex;

pub fn day03_1(text: &str) -> i32 {
    let mut sum_of_pdts = 0;
    let re = Regex::new(r"mul\([0-9]+,[0-9]+\)").unwrap();
    let found: Vec<&str>= re.find_iter(text).map(|m| m.as_str()).collect();

    for expression in found {
        let (_, right) = expression.split_once("(").unwrap();
        let (left, mut right) = right.split_once(",").unwrap();
        right= right.trim_end_matches(")");

        let left_num = left.parse::<i32>().unwrap();
        let right_num = right.parse::<i32>().unwrap();

        sum_of_pdts += left_num * right_num;
    }

    return sum_of_pdts;
}
