use day5::{day_5_1, day_5_2};
use std::fs;
use std::time::Instant;

fn main() {
    let text = fs::read_to_string("day5.txt").expect("Should have read the file");
    let short_text =
"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    assert_eq!(day_5_1(short_text), 35);

    let ans_5_1 = day_5_1(&text);
    assert_eq!(ans_5_1, 3_374_647);
    println!("day 5_1 = {}", ans_5_1);

    // Part 2
    let start = Instant::now();
    for _ in 0..100 {
        day_5_2(short_text);
    }
    let elapsed = start.elapsed();
    println!("day_5_2() took {}µs", elapsed.as_micros());
    assert_eq!(day_5_2(short_text), 46);
    /*
    let start = Instant::now();
    let ans_5_2 = day_5_2(&text);
    let elapsed = start.elapsed();
    println!("day_5_2() took {}s", elapsed.as_secs());
    //assert_eq!(ans_5_2, );
    println!("day 5_2 = {}", ans_5_2);
    */
}
