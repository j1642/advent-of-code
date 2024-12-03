#[derive(PartialEq)]
enum Order {
    Increasing,
    Decreasing,
    Unassigned,
}

fn is_safe_line(nums: &[i32]) -> bool {
    let mut prev_num = -1;
    let mut cur_num;
    let mut order = Order::Unassigned;
    let mut is_report_safe = true;

    for num in nums {
        cur_num = *num;
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
                    break;
                }
            }
        }

        // Check increasing/decreasing
        let diff = cur_num - prev_num;
        if diff < 0 && order == Order::Increasing {
            is_report_safe = false;
            break;
        } else if diff > 0 && order == Order::Decreasing {
            is_report_safe = false;
            break;
        }

        // Check difference magnitude
        let abs_diff = diff.abs();
        if abs_diff < 1 || 3 < abs_diff {
            is_report_safe = false;
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
        if is_safe_line(&get_nums(line)) {
            safe_report_count += 1
        }
    }
    return safe_report_count;
}

pub fn day02_2(text: &str) -> i32 {
    // two approaches: systematically remove each num from a row, one at a time, and test.
    // Or complex logic.
    let mut safe_report_count = 0;
    for line in text.lines() {
        let orig_nums = get_nums(line);

        let nums_minus_first = &orig_nums[1..];
        let nums_minus_last = &orig_nums[..orig_nums.len()-1];
        if is_safe_line(nums_minus_first) || is_safe_line(nums_minus_last) {
            safe_report_count += 1;
            continue;
        }

        let mut interior_partial_nums: Vec<Vec<i32>> = vec![];
        for i in 1..orig_nums.len() - 1 {
            interior_partial_nums.push(orig_nums[..].to_vec());
            interior_partial_nums[i-1].remove(i);
        }

        for nums in interior_partial_nums {
            if is_safe_line(&nums) {
                safe_report_count += 1;
                break;
            }
        }
    }

    return safe_report_count;
}

fn get_nums(line: &str) -> Vec<i32> {
    return line
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap_or_default())
        .collect::<Vec<i32>>();
}
