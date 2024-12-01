use std::collections::HashMap;

fn get_sorted_nums(text: &str) -> (Vec<i32>, Vec<i32>) {
    let mut left_nums: Vec<i32> = vec![];
    let mut right_nums: Vec<i32> = vec![];

    for line in text.lines() {
        let nums = line.split_once("   ").unwrap();
        let left_num = nums.0.parse::<i32>().unwrap();
        let right_num = nums.1.parse::<i32>().unwrap();
        left_nums.push(left_num);
        right_nums.push(right_num);
    }

    left_nums.sort();
    right_nums.sort();
    assert_eq!(right_nums.len(), left_nums.len());

    return (left_nums, right_nums)
}

pub fn day01_1(text: &str) -> i32 {
    let (left_nums, right_nums) = get_sorted_nums(text);
    let mut sum = 0;

    for i in 0..left_nums.len() {
        sum += (left_nums[i] - right_nums[i]).abs();
    }

    return sum;
}

pub fn day01_2(text: &str) -> i32 {
    let (left_nums, right_nums) = get_sorted_nums(text);
    let mut right_num_freq: HashMap<i32, i32> = HashMap::new();

    for i in 0..right_nums.len() {
        if right_num_freq.contains_key(&right_nums[i]) {
            if let Some(count) = right_num_freq.get_mut(&right_nums[i]) {
                *count += 1;
            } else {
                panic!("failed to increment count");
            }
        } else {
            right_num_freq.insert(right_nums[i], 1);
        }
    }

    let mut similarity_score = 0;

    for i in 0..left_nums.len() {
        if let Some(count) = right_num_freq.get(&left_nums[i]) {
            similarity_score += left_nums[i] * *count;
        }
    }

    return similarity_score;
}
