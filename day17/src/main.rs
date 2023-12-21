use day17::day_17_1; //, day_17_2};
use std::fs;

fn main() {
    let text = fs::read_to_string("day17.txt").expect("Should have read the file");
    let example = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533"
        .trim();

    assert_eq!(day_17_1(example), 102);

    let ans_17_1 = day_17_1(&text);
    //assert_eq!(ans_17_1, );
    println!("day 17_1 = {}", ans_17_1);
    println!("ans < 1049");

    /*
    // Part 2
    //env::set_var("RUST_BACKTRACE", "1");
    //assert_eq!(day_17_2(example), );

    let ans_17_2 = day_17_2(&text);
    println!("day 17_2 = {}", ans_17_2);
    //assert_eq!(ans_17_2, 7154);
    */
}
