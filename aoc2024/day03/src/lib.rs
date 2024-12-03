use std::str::Chars;

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

pub fn day03_2(text: &str) -> u32 {
    let mut all_chars = text.chars();
    return recurse(&mut all_chars);
}

fn recurse(all_chars: &mut Chars) -> u32 {
    let mut sum_of_pdts = 0;
    let mut mode = Mode::Enabled;

    while let Some(c) = all_chars.next() {
    //for c in all_chars {
        if c == 'm' {
            match found_m(all_chars) {
                None=>{continue},
                Some(num) => {
                    if mode == Mode::Enabled {
                        sum_of_pdts += num;
                    }
                },
            }
        } else if c == 'd' {
            //look for o
            // is_enabled = found_d()
            match found_d(all_chars) {
                None=>{continue},
                Some(onoff) => {
                    mode = onoff;
                },
            }
        }
    }

    return sum_of_pdts;
}

fn found_m(all_chars: &mut Chars) -> Option<u32> {
    let mut is_valid = true;
    let keys = ['u', 'l', '('];
    for i in 0..keys.len() {
        let next = all_chars.next().unwrap();
        if next != keys[i] {
            is_valid = false;
            break;
        }
    }

    if is_valid {
        return get_product(all_chars);
    }
    return None
}

fn get_product(all_chars: &mut Chars) -> Option<u32> {
    let mut next: char;
    let mut is_valid = true;
    let mut num: u32 = 0;
    let mut first = 0;
    let mut second = 0;

    while is_valid {
        next = all_chars.next().unwrap();
        if next.is_digit(10) {
            num *= 10;
            num += next.to_digit(10).unwrap();
        } else {
            if next == ',' && first == 0 && second == 0 {
                first = num;
                num = 0;
            } else if next == ')' && first != 0 && second == 0 {
                second = num;
                // is_valid still true
                break;
            } else {
                is_valid = false;
            }
        }
    }

    return Some(first * second);
}

#[derive(PartialEq)]
enum Mode {
    Enabled,
    Disabled,
}

fn found_d(all_chars: &mut Chars) -> Option<Mode>{
    let mut found_do = false;
    let mut found_dont = false;
    let keys = ['o', 'n', '\'', 't'];
    for i in 0..keys.len() {
        let next = all_chars.next().unwrap();
        if next != keys[i] {
            break;
        } else if i == 0 {
            found_do = true;
        } else if i == 3 {
            found_dont = true;
        }
    }

    if found_dont {
        return Some(Mode::Disabled);
    } else if found_do {
        return Some(Mode::Enabled);
    }
    return None;
}
