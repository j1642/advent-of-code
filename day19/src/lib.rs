use std::collections::HashMap;

#[derive(Debug)]
struct Part {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

#[derive(Debug, Clone)]
struct PartRange<'a> {
    key: &'a str,
    x_min: u32,
    x_max: u32,
    m_min: u32,
    m_max: u32,
    a_min: u32,
    a_max: u32,
    s_min: u32,
    s_max: u32,
}

#[derive(Debug)]
struct Check<'a> {
    letter: char,
    compare: char,
    num: u32,
    result: &'a str,
}

pub fn day19_2(text: &str) -> u64 {
    // Return the amount of possible accepted combos for all X,
    // M, A, and S values are in [1, 4000], inclusive
    let mut total = 0;
    let (rules, _) = text.split_once("\n\n").unwrap();
    let workflows = parse_workflows(rules);

    let mut stack: Vec<PartRange> = vec![
        // Inclusive mins and maxes
        PartRange {
            key: "in",
            x_min: 1,
            x_max: 4000,
            m_min: 1,
            m_max: 4000,
            a_min: 1,
            a_max: 4000,
            s_min: 1,
            s_max: 4000,
        },
    ];

    let mut min_or_max_fn: fn(_, _) -> _;
    while stack.len() > 0 {
        let mut orig_parts = stack.pop().unwrap();

        if orig_parts.key == "R" {
            continue;
        } else if orig_parts.key == "A" {
            // + 1 because upper bounds are inclusive
            total += (orig_parts.x_max - orig_parts.x_min + 1) as u64
                * (orig_parts.m_max - orig_parts.m_min + 1) as u64
                * (orig_parts.a_max - orig_parts.a_min + 1) as u64
                * (orig_parts.s_max - orig_parts.s_min + 1) as u64;
            continue;
        }

        for check in workflows[orig_parts.key].iter() {
            let mut parts = orig_parts.clone();
            if check.compare == '<' {
                min_or_max_fn = std::cmp::min::<u32>;
            } else if check.compare == '>' {
                min_or_max_fn = std::cmp::max::<u32>;
            } else if check.compare == 'z' {
                parts.key = check.result;
                stack.push(parts);
                break;
            } else {
                panic!();
            }

            let min;
            let max;
            match check.letter {
                'x' => {
                    min = parts.x_min;
                    max = parts.x_max;
                }
                'm' => {
                    min = parts.m_min;
                    max = parts.m_max;
                }
                'a' => {
                    min = parts.a_min;
                    max = parts.a_max;
                }
                's' => {
                    min = parts.s_min;
                    max = parts.s_max;
                }
                _ => {
                    panic!()
                }
            }

            if min_or_max_fn(check.num, min) != min && min_or_max_fn(check.num, max) != max {
                // Full range is rejected
                continue;
            } else if min_or_max_fn(check.num, min) == min {
                if min_or_max_fn(check.num, max) == max {
                    // Full range is accepted
                    parts.key = check.result;
                    stack.push(parts);
                    break;
                }
                // Min accepted, max rejected
                match check.letter {
                    'x' => {
                        // modified orig_parts used in next checks
                        if check.compare == '<' {
                            parts.x_max = check.num - 1;
                            orig_parts.x_min = check.num;
                        } else {
                            panic!();
                        }
                    }
                    'm' => {
                        if check.compare == '<' {
                            parts.m_max = check.num - 1;
                            orig_parts.m_min = check.num;
                        } else {
                            panic!();
                        }
                    }
                    'a' => {
                        if check.compare == '<' {
                            parts.a_max = check.num - 1;
                            orig_parts.a_min = check.num;
                        } else {
                            panic!();
                        }
                    }
                    's' => {
                        if check.compare == '<' {
                            parts.s_max = check.num - 1;
                            orig_parts.s_min = check.num;
                        } else {
                            panic!();
                        }
                    }
                    _ => {
                        panic!()
                    }
                }
                parts.key = check.result;
                stack.push(parts);
                // modified orig_parts is used in the next iteration
            } else {
                // Max is in the range, min rejected
                if min_or_max_fn(check.num, max) != max {
                    panic!();
                }
                match check.letter {
                    'x' => {
                        if check.compare == '>' {
                            parts.x_min = check.num + 1;
                            orig_parts.x_max = check.num;
                        } else {
                            panic!();
                        }
                    }
                    'm' => {
                        if check.compare == '>' {
                            parts.m_min = check.num + 1;
                            orig_parts.m_max = check.num;
                        } else {
                            panic!();
                        }
                    }
                    'a' => {
                        if check.compare == '>' {
                            parts.a_min = check.num + 1;
                            orig_parts.a_max = check.num;
                        } else {
                            panic!();
                        }
                    }
                    's' => {
                        if check.compare == '>' {
                            parts.s_min = check.num + 1;
                            orig_parts.s_max = check.num;
                        } else {
                            panic!();
                        }
                    }
                    _ => {
                        panic!()
                    }
                }
                parts.key = check.result;
                stack.push(parts);
                // modified orig_parts is used in the next iteration
            }
        }
    }

    return total;
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

        let mut min_or_max_fn: fn(_, _) -> _;

        while !is_accepted && !is_rejected {
            for check in workflows[key].iter() {
                if check.compare == 'z' {
                    key = check.result;
                    continue;
                } else if check.compare == '<' {
                    min_or_max_fn = std::cmp::min::<u32>;
                } else if check.compare == '>' {
                    min_or_max_fn = std::cmp::max::<u32>;
                } else {
                    panic!();
                }

                match check.letter {
                    'x' => {
                        if min_or_max_fn(part.x, check.num) == part.x {
                            key = check.result;
                            break;
                        }
                    }
                    'm' => {
                        if min_or_max_fn(part.m, check.num) == part.m {
                            key = check.result;
                            break;
                        }
                    }
                    'a' => {
                        if min_or_max_fn(part.a, check.num) == part.a {
                            key = check.result;
                            break;
                        }
                    }
                    's' => {
                        if min_or_max_fn(part.s, check.num) == part.s {
                            key = check.result;
                            break;
                        }
                    }
                    _ => {
                        panic!();
                    }
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
