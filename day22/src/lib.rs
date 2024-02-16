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
// - blocks have dimensions 1x1xN, where N is >= 1

pub fn day22_1(text: &str) -> u32 {
    let mut blocks = find_blocks(text);
    /*
    if blocks.len() > 15 {
        println!("fresh blocks:");
        for i in 1400..1410 {
            println!("{:?}", blocks[i]);
        }
    }
    */
    let lower_neighbor_idxs = drop_blocks(&mut blocks);

    let mut upper_neighbor_idxs = vec![vec![]; blocks.len()];
    for i in 0..lower_neighbor_idxs.len() {
        for j in 0..lower_neighbor_idxs[i].len() {
            upper_neighbor_idxs[lower_neighbor_idxs[i][j]].push(i);
        }
    }

    /*
    if blocks.len() > 15 {
        println!("dropped and sorted blocks:");
        for i in 1400..1410 {
            println!("{:?}", blocks[i]);
        }
    }
    */
    /*
    for i in 0..neighbor_idxs.len() {
        println!("{:?}", blocks[i]);
        println!("{:?}", neighbor_idxs[i]);
    }
    */
    return count_destructible_blocks(blocks, upper_neighbor_idxs, lower_neighbor_idxs);
}

fn count_destructible_blocks(
    blocks: Vec<Block>,
    upper_neighbor_idxs: Vec<Vec<usize>>,
    lower_neighbor_idxs: Vec<Vec<usize>>,
) -> u32 {
    // Determine which bricks can be destroyed without other bricks falling down
    let mut hist = vec![0; blocks.len()];

    for lower_lower_neighbors in &lower_neighbor_idxs {
        for lower_lower_neighbor in lower_lower_neighbors {
            hist[*lower_lower_neighbor] += 1;
        }
    }
    if hist.len() < 10 {
        assert!(hist == vec![2, 2, 2, 1, 1, 1, 0]);
    }

    // Second possible way to determine which blocks to remove. Still wrong
    // Check each block's upper neighbors. If an upper neighbor has only one
    // supporting block, do not destory that single supporting block.
    let mut destructible_block_count = 0;
    for (i, top_neighbors) in upper_neighbor_idxs.iter().enumerate() {
        println!(
            "upper: {:?}, lower: {:?}",
            top_neighbors, lower_neighbor_idxs[i]
        );
        let mut destroy = true;
        for blk_idx in top_neighbors {
            if lower_neighbor_idxs[*blk_idx].len() == 1 {
                destroy = false;
                break;
            }
        }
        if destroy {
            println!("destroy {i}");
            destructible_block_count += 1;
        }
    }
    return destructible_block_count;
    // `hist` is (or is similar to) counting each block's top neighbors

    /*
    let mut have_match_in_hist: Vec<bool> = vec![false; blocks.len()];

    for i in 0..lower_neighbor_idxs.len() - 1 {
        if lower_neighbor_idxs[i].len() == 0 {
            continue;
        }
        // TODO: lower and upper lower_neighbors need to be the same, not just lower
        for j in (i + 1)..lower_neighbor_idxs.len() {
            if lower_neighbor_idxs[i] == lower_neighbor_idxs[j]
                && upper_neighbor_idxs[i] == upper_neighbor_idxs[j]
                && hist[i] == hist[j]
                && hist[i] != 0
            {
                have_match_in_hist[j] = true;
                have_match_in_hist[i] = true;
            }
        }
    }

    // Find blocks with no blocks on top of them
    for i in 0..hist.len() {
        if hist[i] == 0 {
            destructible_block_count += 1;
        }
    }

    for tf in have_match_in_hist {
        if tf {
            destructible_block_count += 1;
        }
    }

    return destructible_block_count;
    */
}

fn drop_blocks(blocks: &mut Vec<Block>) -> Vec<Vec<usize>> {
    // Mutate `blocks` argument and return pseudo-adjacency matrix (lower neighbors only).
    // Decrease each block's elevation until it rests on another block or z=1,
    // and find adjacent, lower blocks in same iterations

    // Adjust the lowest elevation blocks first after sorting
    // What about blocks aligned on z-axis? - use lower z
    // TODO: switch to sort_unstable_by if no bugs, more efficient
    blocks.sort_by(|a, b| {
        min(a.start.z, a.end.z)
            .partial_cmp(min(&b.start.z, &b.end.z))
            .unwrap()
    });
    /*
    if blocks.len() > 15 {
        println!("sorted blocks:");
        for i in 1400..1410 {
            println!("{:?}", blocks[i]);
        }
    }
    */

    let mut neighbor_idxs: Vec<Vec<usize>> = vec![vec![]; blocks.len()];

    for i in 0..blocks.len() {
        let mut block = blocks[i].clone();

        // lowest elevation block starts at z=1 because of the examples
        if i == 0 {
            if block.start.z > 1 {
                let start_end_diff = block.end.z - block.start.z;
                block.start.z = 1;
                block.end.z = start_end_diff;
            }
            blocks[i] = block;
            continue;
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
                        && top_cube.z > bottom_cube.z
                    {
                        if !can_fall_more {
                            if block.start.z == bottom_cube.z + 1 {
                                neighbor_idxs[i].push(j);
                            }
                            break;
                        }
                        neighbor_idxs[i].push(j);
                        can_fall_more = false;

                        // Block falls to just above its lower neighbor
                        //println!("i = {i}");
                        //println!("b.start: {:?}, b.end: {:?}", block.start, block.end);
                        let start_end_diff = block.end.z - block.start.z;
                        block.start.z = bottom_cube.z + 1;
                        block.end.z = block.start.z + start_end_diff;

                        // Break because we already know blocks[j] is a neighbor
                        break;
                    }
                }
                if !can_fall_more {
                    // Break because we already know blocks[j] is a neighbor
                    break;
                }
            }
            if block.start.z == 1 {
                break;
            }

            if can_fall_more {
                for k in 0..top_cubes.len() {
                    if top_cubes[k].z == 1 {
                        continue;
                    }
                    top_cubes[k].z -= 1;
                }
            } else {
                // Check if next block is a lower neighbor to blocks[i]
                continue;
            }
        }

        // Replace old block with the changed clone. Avoids mixing (im)mutable borrowing
        if neighbor_idxs[i].len() == 0 {
            let start_end_diff: usize = block.end.z - block.start.z;
            block.start.z = 1;
            block.end.z = block.start.z + start_end_diff;
        }
        if block.start.z > block.end.z {
            println!("i={i} {:?}", block);
            panic!("start.z = {}, end.z = {}", block.start.z, block.end.z);
        }
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
