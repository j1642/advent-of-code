#[derive(PartialEq)]
enum Order {
    Increasing,
    Decreasing,
    Unassigned,
}

fn is_safe_line(line: &str) -> bool {
    let mut prev_num = -1;
    let mut cur_num;
    let mut order = Order::Unassigned;
    let mut is_report_safe = true;
    //let mut line_ind += 1;

    for num in line.split_whitespace() {
        cur_num = num.parse::<i32>().unwrap();
        if order == Order::Unassigned {
            if prev_num == -1 {
                prev_num = cur_num;
                continue
            } else {
                if prev_num < cur_num {
                    order = Order::Increasing
                } else if prev_num > cur_num {
                    order = Order::Decreasing
                } else {
                    is_report_safe = false;
                    //println!("bad line {}: repeat num {} to {}", line_ind, prev_num, cur_num);
                    break;
                }
            }
        }

        // Check increasing/decreasing
        let diff = cur_num - prev_num;
        if diff < 0 && order == Order::Increasing {
            is_report_safe = false;
            //println!("bad line {}: not inc. {} to {}", line_ind, prev_num, cur_num);
            break;
        } else if diff > 0 && order == Order::Decreasing {
            is_report_safe = false;
            //println!("bad line {}: not dec. {} to {}", line_ind, prev_num, cur_num);
            break;
        }

        // Check difference magnitude
        let abs_diff = diff.abs();
        if abs_diff < 1 || 3 < abs_diff {
            is_report_safe = false;
            //println!("bad line {}: bad abs_diff={}", line_ind, abs_diff);
            break;
        }

        prev_num = cur_num;
    }
    if is_report_safe {
        return true;
    }
    return false;
}

pub fn day02_1(text: &str) -> i32 {
    let mut safe_report_count = 0;

    for line in text.lines() {
        if is_safe_line(line) {
            safe_report_count += 1
        }
    }
    return safe_report_count;
}

pub fn day02_2(text: &str) -> i32 {
    let mut safe_report_count = 0;
    for line in text.lines() {
        let orig_nums = get_nums(line);
        let nums_minus_first = &orig_nums[1..];
        let nums_minus_last = &orig_nums[..orig_nums.len()-1];

        //if is_safe_line(nums_minus_first) || is_safe_line(nums_minus_last) {
            //safe_report_count += 1;
        //}
        //let mut partial_nums: Vec<i32> = Vec::with_capacity(orig_nums.len());
        //println!("orig {:?}", orig_nums);
        //println!("cut first {:?}", nums_minus_first);
        //println!("cut last {:?}", nums_minus_last);
        break;
    }

    return safe_report_count;
    // two approaches: systematically remove each num from a row, one at a time, and test.
    // Or complex logic.
    //
    // keep prev2, prev, cur
    // switch b/w Inc and Dec if one of the first two nums elided
    // remove "mountain/gorge" nums like 1-10-2... or 10-1-9...
}

fn get_nums(line: &str) -> Vec<i32> {
    return line
        .split_whitespace()
        //.map(|s| s.trim())
        .map(|s| s.parse::<i32>().unwrap_or_default())
        .collect::<Vec<i32>>();
}
