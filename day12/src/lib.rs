pub fn day_12_1(text: &str) -> u32 {
    let mut count_perms = 0;
    for line in text.lines() {
        let (springs, nums) = line.split_once(' ').unwrap();
        let spring_groups = springs
            .split('.')
            .filter(|s| !s.is_empty())
            .collect::<Vec<&str>>();
        let nums = nums
            .split(',')
            // TODO: maybe add 1 to include half of the buffer zone between each num
            // This may cause issues when the buffer overlaps a removed '.'
            .map(|s| s.parse().unwrap_or_default())
            .collect::<Vec<usize>>();

        let mut perms = 1;

        let mut only_contains_pound = Vec::with_capacity(spring_groups.len());
        for i in 0..spring_groups.len() {
            only_contains_pound.push(spring_groups[i].trim_end_matches('#') == "");
        }

        // TODO: get number of combos/perms
        // 1. everything matches nicely
        if spring_groups.len() == nums.len() {
            // Everything matches perfectly, 1 permutation
            // Other
            // Remove and count #, num - num#
        }
        // 2. everything sort of matches
        // 3. few/no matches
        println!("{:?}, {:?}, {}", spring_groups, nums, perms);
        count_perms += perms;
    }
    return count_perms;
}

fn factorial(n: usize) -> usize {
    if n == 0 || n == 1 {
        return 1;
    }
    let mut res = 1;
    for i in 1..=n {
        res *= i;
    }
    return res;
}
