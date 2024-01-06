use std::collections::HashMap;

#[derive(Debug)]
struct Part {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}
#[derive(Debug)]
struct Check<'a> {
    letter: char,
    compare: char,
    num: u32,
    result: &'a str,
}

pub fn day19_1(text: &str) -> u32 {
    let mut total = 0;
    let (rules, parts) = text.split_once("\n\n").unwrap();
    let parts = parse_parts(parts);
    let workflows = parse_workflows(rules);

    for part in parts.iter() {
        let mut is_accepted = false;
        let mut is_rejected = false;
        let mut key = "in";
        // Refactor. Not sure how
        // Can't find a std lib like Python's operator lib
        while !is_accepted && !is_rejected {
            for check in workflows[key].iter() {
                if check.compare == '<' {
                    match check.letter {
                        'x' => {
                            if part.x < check.num {
                                key = check.result;
                                break;
                            }
                        }
                        'm' => {
                            if part.m < check.num {
                                key = check.result;
                                break;
                            }
                        }
                        'a' => {
                            if part.a < check.num {
                                key = check.result;
                                break;
                            }
                        }
                        's' => {
                            if part.s < check.num {
                                key = check.result;
                                break;
                            }
                        }
                        _ => {
                            panic!();
                        }
                    }
                } else if check.compare == '>' {
                    match check.letter {
                        'x' => {
                            if part.x > check.num {
                                key = check.result;
                                break;
                            }
                        }
                        'm' => {
                            if part.m > check.num {
                                key = check.result;
                                break;
                            }
                        }
                        'a' => {
                            if part.a > check.num {
                                key = check.result;
                                break;
                            }
                        }
                        's' => {
                            if part.s > check.num {
                                key = check.result;
                                break;
                            }
                        }
                        _ => {
                            panic!();
                        }
                    }
                } else if check.compare == 'z' {
                    key = check.result;
                } else {
                    panic!();
                }
            }
            if key == "R" {
                is_rejected = true;
            } else if key == "A" {
                is_accepted = true;
            }
        }
        if is_accepted {
            total += part.x + part.m + part.a + part.s;
        }
    }

    return total;
}

fn parse_workflows(rules: &str) -> HashMap<&str, Vec<Check>> {
    let mut workflows = HashMap::new();
    for line in rules.lines() {
        let (label, outcomes) = line.split_once('{').unwrap();
        let outcomes = outcomes.split(',');

        for mut outcome in outcomes {
            outcome = outcome.trim_end_matches('}');
            let mut check = Check {
                letter: 'z',
                compare: 'z',
                num: 0,
                result: "placeholder",
            };

            if outcome.contains(':') {
                let (require, result) = outcome.split_once(':').unwrap();
                check.result = result;
                let mut chars = require.chars();
                check.letter = chars.next().unwrap();
                check.compare = chars.next().unwrap();

                for c in chars {
                    check.num *= 10;
                    check.num += c.to_digit(10).unwrap();
                }
            } else {
                check.result = outcome;
            }
            if !workflows.contains_key(label) {
                workflows.insert(label, vec![]);
            }
            workflows.get_mut(label).map(|val| val.push(check));
        }
    }
    return workflows;
}

fn parse_parts(items: &str) -> Vec<Part> {
    // Assumes order of letters in each part is XMAS
    let mut parts: Vec<Part> = vec![];
    let removeable = &['x', 'm', 'a', 's', '='];
    for item in items.lines() {
        let item = item.trim_matches(|c| c == '{' || c == '}');
        let item = item.split(",");

        let mut part = Part {
            x: 0,
            m: 0,
            a: 0,
            s: 0,
        };

        for (i, mut num) in item.enumerate() {
            num = num.trim_start_matches(removeable);
            match i {
                0 => {
                    part.x = num.parse::<u32>().unwrap();
                }
                1 => {
                    part.m = num.parse::<u32>().unwrap();
                }
                2 => {
                    part.a = num.parse::<u32>().unwrap();
                }
                3 => {
                    part.s = num.parse::<u32>().unwrap();
                }
                _ => {
                    panic!();
                }
            }
        }
        parts.push(part);
    }
    return parts;
}
