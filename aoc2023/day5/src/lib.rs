pub fn day_5_1(text: &str) -> u64 {
    // Return the sum of the location ints, mapped from the seed ints
    let mut seeds: Vec<u64>;
    let mut mapped_seeds: Vec<u64> = vec![];
    let mut dst_src_range: Vec<u64>;
    let mut changed: Vec<bool> = vec![];

    for (i, section) in text.split("\n\n").enumerate() {
        let (_, nums) = section.split_once(':').unwrap();
        if i == 0 {
            seeds = nums
                .split(' ')
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
            dst_src_range = map_nums
                .split(' ')
                .map(|s| s.trim())
                //.filter(|s| !s.is_empty())
                .map(|s| s.parse::<u64>().unwrap_or_default())
                .collect();
            let dst = dst_src_range[0];
            let src = dst_src_range[1];
            let range = dst_src_range[2];
            for i in 0..mapped_seeds.len() {
                if src <= mapped_seeds[i] && mapped_seeds[i] < (src + range) && !changed[i] {
                    mapped_seeds[i] = dst + (mapped_seeds[i] - src);
                    changed[i] = true;
                }
            }
        }
    }
    return *mapped_seeds.iter().min().unwrap();
}

pub fn day_5_2(text: &str) -> u32 {
    // Return the sum of the location ints, mapped from the seed ints
    let (first_line, _) = text.split_once('\n').unwrap();
    let (_, nums) = first_line.split_once(':').unwrap();
    let seeds;
    seeds = nums
        .split(' ')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<u32>().unwrap_or_default())
        .collect::<Vec<u32>>();
    println!("seeds: {:?}", seeds);

    let mut d_s_ranges: Vec<Vec<[u32; 3]>> = vec![];
    let mut dst;
    let mut src;
    let mut range;
    let mut loc_to_seed: u32;

    for orig_loc in 0u32..u32::MAX {
        if orig_loc % 10_000 == 0 {
            println!("{}", orig_loc);
        }
        loc_to_seed = orig_loc;

        for (i, section) in text.rsplit("\n\n").enumerate() {
            // i = 7 for the seed section
            if i == 7 {
                for i in 0..seeds.len() - 1 {
                    if i % 2 == 1 {
                        continue;
                    }
                    if seeds[i] <= loc_to_seed && loc_to_seed < (seeds[i] + seeds[i + 1]) {
                        println!("final loc_to_seed: {loc_to_seed}");
                        println!("accepted seed start: {}", seeds[i]);
                        return orig_loc;
                    }
                }
                continue;
            }
            if i == d_s_ranges.len() {
                let (_, nums) = section.split_once(':').unwrap();
                let mut d_s_range_group: Vec<[u32; 3]> = vec![];
                for map_nums in nums.split('\n').filter(|s| !s.is_empty()) {
                    let dst_src_range: Vec<u32> = map_nums
                        .split(' ')
                        .map(|s| s.trim())
                        .map(|s| s.parse::<u32>().unwrap_or_default())
                        .collect();
                    let mut dsr_array: [u32; 3] = [0, 0, 0];
                    for j in 0..dst_src_range.len() {
                        dsr_array[j] = dst_src_range[j];
                    }
                    d_s_range_group.push(dsr_array);
                }
                d_s_ranges.push(d_s_range_group);
            }
            // Reverse dst and src to search backwards
            for j in 0..d_s_ranges[i].len() {
                dst = d_s_ranges[i][j][1];
                src = d_s_ranges[i][j][0];
                range = d_s_ranges[i][j][2];
                if src <= loc_to_seed && loc_to_seed < (src + range) {
                    // Using = instead of += avoids subtraction overflow
                    loc_to_seed = dst + loc_to_seed - src;
                    break;
                }
            }
        }
    }
    return 0;
}
