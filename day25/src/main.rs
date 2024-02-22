use day25::day25_1; //, day25_2};
use std::fs;

fn main() {
    let text = fs::read_to_string("day25.txt").expect("Should have read the file");
    let example = "jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr";

    // check for intersections within 7 <= (x, y) <= 27
    assert_eq!(day25_1(example), 54);

    /*
    let ans_25_1 = day25_1(&text);
    println!("day 25_1 = {}", ans_25_1);
    assert_eq!(ans_25_1, 21679);
    */

    /*
    // Part 2
    //env::set_var("RUST_BACKTRACE", "1");
    assert_eq!(day25_2(example), 154);

    let ans_25_2 = day25_2(&text);
    println!("day 25_2 = {}", ans_25_2);
    assert_eq!(ans_25_2, 6546);
    */
}
