use std::cmp::min;

#[derive(Debug, Clone, PartialEq)]
struct Point {
    x: usize,
    y: usize,
    z: usize,
}

#[derive(Debug, Clone)]
struct Block {
    start: Point,
    end: Point,
}

// Assumptions:
// - blocks have dimensions 1x1xN

// TODO: determine which bricks touch each other (naive n^2) - sort of implemented
// TODO: determine which bricks can be destroyed without other
//    bricks falling down
pub fn day22_1(text: &str) -> u32 {
    let mut blocks = find_blocks(text);
    for (i, block) in blocks.iter().enumerate() {
        println!("i: {i},  {:?}", block);
    }
    drop_blocks(&mut blocks);
    for (i, block) in blocks.iter().enumerate() {
        println!("i: {i},  {:?}", block);
    }
    return 0;
}

fn drop_blocks(blocks: &mut Vec<Block>) {
    // Decrease each block's elavation until it rests on another block or z=1

    // Adjust the lowest elavation blocks first after sorting
    // What about blocks aligned on z-axis? - use lower z
    blocks.sort_by(|a, b| {
        min(a.start.z, a.end.z)
            .partial_cmp(min(&b.start.z, &b.end.z))
            .unwrap()
    });

    for i in 0..blocks.len() {
        let mut block = blocks[i].clone();
        if block.start == block.end {
            panic!("block start == block end");
        }
        // lowest elavation block starts at z=1 because of the examples
        if i == 0 {
            if block.start.z > 1 {
                let start_end_diff = block.end.z - block.start.z;
                block.start.z = 1;
                block.end.z = start_end_diff;
            }
            blocks[i] = block;
            continue;
        }

        // Block falls to just above its lower neighbor
        if block.start.z > (1 + blocks[i - 1].end.z) {
            let start_end_diff = block.end.z - block.start.z;
            block.start.z = 1 + blocks[i - 1].end.z;
            block.end.z = block.start.z + start_end_diff;
        }
        // TODO: remove at the end
        if block.start == block.end {
            panic!("block start == block end: {:?}", block.start);
        }

        let mut top_cubes: Vec<Point> = vec![];
        if block.start.x != block.end.x {
            for x in block.start.x..=block.end.x {
                top_cubes.push(Point {
                    x: x,
                    y: block.start.y,
                    z: block.start.z,
                });
            }
        } else if block.start.y != block.end.y {
            for y in block.start.y..=block.end.y {
                top_cubes.push(Point {
                    x: block.start.x,
                    y: y,
                    z: block.start.z,
                });
            }
        } else if block.start.z != block.end.z {
            top_cubes.push(Point {
                x: block.start.x,
                y: block.start.y,
                z: block.start.z,
            });
        }

        // while there is no touched block below, decrement start and end z
        for j in (0..i).rev() {
            let mut bottom_cubes: Vec<Point> = vec![];
            if blocks[j].start.x != blocks[j].end.x {
                for x in blocks[j].start.x..=blocks[j].end.x {
                    bottom_cubes.push(Point {
                        x: x,
                        y: blocks[j].start.y,
                        z: blocks[j].start.z,
                    });
                }
            } else if blocks[j].start.y != blocks[j].end.y {
                for y in blocks[j].start.y..=blocks[j].end.y {
                    bottom_cubes.push(Point {
                        x: blocks[j].start.x,
                        y: y,
                        z: blocks[j].start.z,
                    });
                }
            } else if blocks[j].start.z != blocks[j].end.z {
                bottom_cubes.push(Point {
                    x: blocks[j].start.x,
                    y: blocks[j].start.y,
                    z: blocks[j].start.z,
                });
            }

            // Consider intersection of sets instead of n^2 search
            let mut can_fall_more = true;
            for top_cube in &top_cubes {
                for bottom_cube in &bottom_cubes {
                    if top_cube.x == bottom_cube.x && top_cube.y == bottom_cube.y {
                        //assert!(top_cube.z == bottom_cube.z + 1);
                        //println!("curr: {:?}", block);
                        println!("top: {:?}, bott: {:?}", top_cube, bottom_cube);
                        can_fall_more = false;
                        break;
                        // TODO: instead of breaking, continue to find all neighbors
                    }
                }
                if !can_fall_more {
                    break;
                }
            }
            if !can_fall_more {
                break;
            }

            if can_fall_more {
                block.start.z -= 1;
                block.end.z -= 1;
                println!(
                    "decrementing block {i}, start.z={}, end.z={}",
                    block.start.z, block.end.z
                );
            }
            if block.start.z == 1 {
                break;
            }
        }

        // Replace old block with the changed clone. Avoids mixing (im)mutable borrowing
        blocks[i] = block;
    }
}

fn find_blocks(text: &str) -> Vec<Block> {
    // Design decision: should the differing coord ALWAYS be lower in start?
    let mut blocks = vec![];
    for line in text.lines() {
        let (left, right) = line
            .split_once('~')
            .expect("Input should have one `~` in each line");
        let left = left.split(',');
        let right = right.split(',');

        let mut coords = [0; 6];
        let combined = left.chain(right);
        for (i, coord) in combined.enumerate() {
            coords[i] = coord.parse::<usize>().unwrap();
        }

        let mut block = Block {
            start: Point {
                x: coords[0],
                y: coords[1],
                z: coords[2],
            },
            end: Point {
                x: coords[3],
                y: coords[4],
                z: coords[5],
            },
        };
        // start.z is always <= end.z
        if block.start.z > block.end.z {
            (block.start, block.end) = (block.end, block.start);
        }
        blocks.push(block);
    }

    return blocks;
}
