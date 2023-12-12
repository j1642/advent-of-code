pub fn day_9_1(text: &str) -> i64 {
    // Predict the next number of each sequence of ints and return their sum
    let mut total = 0;
    for line in text.lines() {
        // numbers may be negative
        let orig_nums: Vec<i64> = line
            .split(' ')
            .map(|s| s.trim())
            .map(|s| s.parse::<i64>().unwrap_or_default())
            .collect::<Vec<i64>>();

        let next_num = orig_nums[orig_nums.len() - 1] + recurse_pt1(&orig_nums);
        total += next_num;
    }
    return total;
}

fn recurse_pt1(nums: &Vec<i64>) -> i64 {
    if contains_only_zeroes(&nums) {
        return 0;
    }

    let mut differences: Vec<i64> = Vec::with_capacity(nums.len());
    for i in 0..nums.len() - 1 {
        differences.push(nums[i + 1] - nums[i]);
    }

    return differences[differences.len() - 1] + recurse(&differences);
}

pub fn day_9_2(text: &str) -> i64 {
    // Predict the preceding number of each sequence of ints and return their sum
    let mut total = 0;
    for line in text.lines() {
        // numbers may be negative
        let orig_nums: Vec<i64> = line
            .split(' ')
            .map(|s| s.trim())
            .map(|s| s.parse::<i64>().unwrap_or_default())
            .collect::<Vec<i64>>();

        let next_num = orig_nums[orig_nums.len() - 1] + recurse_pt2(&orig_nums);
        total += next_num;
    }
    return total;
}

fn recurse_pt2(nums: &Vec<i64>) -> i64 {
    if contains_only_zeroes(&nums) {
        return 0;
    }

    let mut differences: Vec<i64> = Vec::with_capacity(nums.len());
    for i in 0..nums.len() - 1 {
        differences.push(nums[i + 1] - nums[i]);
    }

    return differences[differences.len() - 1] + recurse(&differences);
}

fn contains_only_zeroes(nums: &Vec<i64>) -> bool {
    for i in 0..nums.len() {
        if nums[i] != 0 {
            return false;
        }
    }
    return true;
}
