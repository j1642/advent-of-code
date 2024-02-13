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

pub fn day22_1(text: &str) -> u32 {
    let mut blocks = find_blocks(text);
    for (i, block) in blocks.iter().enumerate() {
        println!("i: {i},  {:?}", block);
    }
    let neighbor_idxs = drop_blocks(&mut blocks);
    for i in &neighbor_idxs {
        println!("{:?}", i);
    }
    return count_destructible_blocks(blocks, neighbor_idxs);
}

fn count_destructible_blocks(blocks: Vec<Block>, neighbor_idxs: Vec<Vec<usize>>) -> u32 {
    // TODO: determine which bricks can be destroyed without other bricks falling down
    let mut removed_blocks_count = 0;
    let mut hist = vec![0; blocks.len()];

    for lower_neighbors in &neighbor_idxs {
        for lower_neighbor in lower_neighbors {
            hist[*lower_neighbor] += 1;
        }
    }
    // `hist` is (or is similar to) counting neighbors above
    println!("hist: {:?}", hist);

    for i in 0..neighbor_idxs.len() - 1 {
        let mut has_match = false;
        for j in (i + 1)..neighbor_idxs.len() {
            if neighbor_idxs[i] == neighbor_idxs[j] {
                removed_blocks_count += 1;
                has_match = true;
                println!("{i} matched {j}");
            }
        }
        // Include neighbor_idxs[i]
        if has_match {
            removed_blocks_count += 1;
        }
    }
    for i in 0..hist.len() {
        if hist[i] == 0 {
            removed_blocks_count += 1;
        }
    }
    /*
        } else if hist[i] > 1 {
            if neighbor_idxs[i].len() == 0 {
            // bottom-most, supporting block; do nothing
            } else if neighbor_idxs[i].len() > 0 {
                removed_blocks_count += 1;
            }
        }
    }
    */

    return removed_blocks_count;
}

fn drop_blocks(blocks: &mut Vec<Block>) -> Vec<Vec<usize>> {
    // Mutate `blocks` argument and return pseudo-adjacency matrix (lower neighbors only).
    // Decrease each block's elavation until it rests on another block or z=1,
    // and find adjacent, lower blocks in same iterations

    // Adjust the lowest elavation blocks first after sorting
    // What about blocks aligned on z-axis? - use lower z
    // TODO: switch to sort_unstable_by if no bugs, more efficient
    blocks.sort_by(|a, b| {
        min(a.start.z, a.end.z)
            .partial_cmp(min(&b.start.z, &b.end.z))
            .unwrap()
    });

    let mut neighbor_idxs: Vec<Vec<usize>> = vec![vec![]; blocks.len()];

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
        let mut can_fall_more = true;
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

            // TODO: Consider intersection of sets instead of 1/2 n^2 search
            for top_cube in &top_cubes {
                for bottom_cube in &bottom_cubes {
                    if top_cube.x == bottom_cube.x
                        && top_cube.y == bottom_cube.y
                        && top_cube.z == bottom_cube.z + 1
                    {
                        //println!("top z = {}, bot z = {}", top_cube.z, bottom_cube.z);
                        //assert!(top_cube.z == bottom_cube.z + 1);
                        //println!("top: {:?}, bott: {:?}", top_cube, bottom_cube);
                        can_fall_more = false;
                        neighbor_idxs[i].push(j);
                        // Break because we already know blocks[j] is a neighbor
                        break;
                    }
                }
                if !can_fall_more {
                    // Break because we already know blocks[j] is a neighbor
                    break;
                }
            }
            if !can_fall_more {
                //break;
                // Check if next block, blocks[j+1], is a neighbor to blocks[i]
                continue;
            }

            if block.start.z == 1 {
                break;
            }
            if can_fall_more {
                block.start.z -= 1;
                block.end.z -= 1;
                for k in 0..top_cubes.len() {
                    top_cubes[k].z -= 1;
                }
                println!(
                    "decrementing block {i}, start.z={}, end.z={}",
                    block.start.z, block.end.z
                );
            }
        }

        // Replace old block with the changed clone. Avoids mixing (im)mutable borrowing
        blocks[i] = block;
    }

    return neighbor_idxs;
}

fn find_blocks(text: &str) -> Vec<Block> {
    // Get block starting locations from the string
    // Guarantees block.start.z <= block.end.z
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
