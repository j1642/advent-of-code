pub fn day_3_1(text: &str) -> i32 {
    // Return sum of numbers which touch symbols in a 2D matrix
    let mut total: i32 = 0;
    let mut rows: Vec<Vec<i32>> = Vec::with_capacity(10);

    for line in text.lines() {
        let mut row = Vec::with_capacity(line.len());
        let mut num = 0;
        for c in line.chars() {
            if c.is_ascii_digit() {
                num = num * 10 + (c.to_digit(10).unwrap() as i32);
                row.push(num);
                // Set all indexes containing a digit of a number to the number
                for i in (1..row.len()).rev() {
                    // was overwriting -1s by using "!= 0" comparison
                    if row[i] > 0 && row[i - 1] > 0 {
                        row[i - 1] = num;
                    } else {
                        break;
                    }
                }
            } else if c == '.' {
                num = 0;
                row.push(0);
            } else {
                num = 0;
                row.push(-1);
            }
        }
        rows.push(row);
    }
    // Looking for weird values to fix off-by-16 error
    for i in 0..rows.len() {
        for j in 0..rows[0].len() {
            //print!("{} ", rows[i][j]);
            if (rows[i][j] - 16) % 100 == 0 {
                //println!("{:?}", rows[i]);
                break;
            }
        }
        //println!("");
    }

    for row in 0..rows.len() {
        for col in 0..rows[0].len() {
            if rows[row][col] == -1 {
                // Check north-east, east, SE, SW, W, and NW
                // Avoid north and south because of redundancy errors
                let mut directions: Vec<&str> = vec!["ne", "e", "se", "sw", "w", "nw"];
                if col == 0 {
                    let (no_west_directions, _) = directions.split_at_mut(3);
                    directions = no_west_directions.to_vec();
                } else if col == rows[0].len() {
                    let (_, no_east_directions) = directions.split_at_mut(3);
                    directions = no_east_directions.to_vec();
                }
                if row == 0 {
                    directions.retain(|&x| !x.contains("n"));
                } else if row == rows.len() {
                    directions.retain(|&x| !x.contains("s"));
                }
                // A num directly above/below a symbol is counted once
                if row > 0 {
                    let north = rows[row - 1][col];
                    let ne = rows[row - 1][col + 1];
                    let nw = rows[row - 1][col - 1];
                    if north == ne && ne == nw {
                        // Arbitrarily remove NW or NE
                        directions.retain(|&x| x != "nw");
                    }
                }
                if row < rows[0].len() {
                    let south = rows[row + 1][col];
                    let se = rows[row + 1][col + 1];
                    let sw = rows[row + 1][col - 1];
                    if south == se && se == sw {
                        // Arbitrarily remove SW or SE
                        directions.retain(|&x| x != "sw");
                    }
                }
                for direction in directions {
                    let val;
                    match direction {
                        "ne" => val = rows[row - 1][col + 1],
                        "e" => val = rows[row][col + 1],
                        "se" => val = rows[row + 1][col + 1],
                        "sw" => val = rows[row + 1][col - 1],
                        "w" => val = rows[row][col - 1],
                        "nw" => val = rows[row - 1][col - 1],
                        &_ => panic!("invalid direction"),
                    }
                    if val > 0 {
                        total += val;
                    }
                }
            }
        }
    }
    return total;
}
