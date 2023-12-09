pub fn day_5_1(text: &str) -> u64 {
    // Return the sum of the location ints, mapped from the seed ints
    let mut seeds: Vec<u64>;
    let mut mapped_seeds: Vec<u64> = vec![];
    let mut dst_src_range: Vec<u64>;
    let mut changed: Vec<bool> = vec![];

    for (i, section) in text.split("\n\n").enumerate() {
        let (_, nums) = section.split_once(':').unwrap();
        if i == 0 {
            seeds = nums.split(' ')
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .map(|s| s.parse::<u64>().unwrap_or_default())
                .collect::<Vec<u64>>();
            mapped_seeds = seeds.clone();
            for _ in 0..seeds.len() {
                changed.push(false);
            }
            continue;
        }
        for i in 0..changed.len() {
            changed[i] = false;
        }
        for map_nums in nums.split('\n').filter(|s| !s.is_empty()) {
            dst_src_range = map_nums.split(' ')
                .map(|s| s.trim())
                //.filter(|s| !s.is_empty())
                .map(|s| s.parse::<u64>().unwrap_or_default())
                .collect();
            let dst = dst_src_range[0];
            let src = dst_src_range[1];
            let range = dst_src_range[2];
            for i in 0..mapped_seeds.len() {
                if src <= mapped_seeds[i] && mapped_seeds[i] < (src + range) && !changed[i]{
                    mapped_seeds[i] = dst + (mapped_seeds[i] - src);
                    changed[i] = true;
                }
            }
        }
    }
    return *mapped_seeds.iter().min().unwrap();
}

pub fn day_5_2(text: &str) -> u64 {
    // Return the sum of the location ints, mapped from the seed ints
    let mut seeds: Vec<u64>;
    let mut mapped_seeds: Vec<u64> = vec![];
    let mut dst_src_range: Vec<u64>;
    let mut changed: Vec<bool> = vec![];

    for (i, section) in text.split("\n\n").rev().enumerate() {
        let (_, nums) = section.split_once(':').unwrap();
        if i == 0 {
            seeds = nums.split(' ')
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .map(|s| s.parse::<u64>().unwrap_or_default())
                .collect::<Vec<u64>>();
            let mut ranges: Vec<u64> = Vec::with_capacity(seeds.len() / 2);
            for i in 0..seeds.len() / 2 {
                ranges.push(seeds.remove(i + 1))
            }
            //println!("ranges sum: {}", ranges.iter().sum::<u64>());
            for (i, range) in ranges.iter().enumerate() {
                for j in 1..*range {
                    seeds.push(seeds[i] + j);
                }
            }
            //println!("seeds len: {}", seeds.len());
            mapped_seeds = seeds.clone();
            for _ in 0..seeds.len() {
                changed.push(false);
            }
            continue;
        }
        for i in 0..changed.len() {
            changed[i] = false;
        }
        for map_nums in nums.split('\n').filter(|s| !s.is_empty()) {
            dst_src_range = map_nums.split(' ')
                .map(|s| s.trim())
                .map(|s| s.parse::<u64>().unwrap_or_default())
                .collect();
            let dst = dst_src_range[0];
            let src = dst_src_range[1];
            let range = dst_src_range[2];
            for i in 0..mapped_seeds.len() {
                if src <= mapped_seeds[i] && mapped_seeds[i] < (src + range) && !changed[i]{
                    mapped_seeds[i] = dst + (mapped_seeds[i] - src);
                    changed[i] = true;
                }
            }
        }
    }
    return *mapped_seeds.iter().min().unwrap();
}
