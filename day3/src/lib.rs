fn build_2d_matrix(text: &str) -> Vec<Vec<i32>> {
    // Return 2D list of lists of a text block
    let mut rows: Vec<Vec<i32>> = Vec::with_capacity(10);

    for line in text.lines() {
        let mut row = Vec::with_capacity(line.len());
        row.push(0);
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
            } else if c == '*' {
                num = 0;
                row.push(-2);
            } else {
                num = 0;
                row.push(-1);
            }
        }
        row.push(0);
        rows.push(row);
    }
    return rows;
}

pub fn day_3_1(text: &str) -> i32 {
    // Return sum of numbers adjacent to negative numbers in a 2D matrix
    let mut total: i32 = 0;
    let rows = build_2d_matrix(text);

    for row in 0..rows.len() {
        if rows[row].len() != rows[0].len() {
            panic!("vectors are not the same length");
        }
        for col in 0..rows[0].len() {
            if rows[row][col] >= 0 {
                continue;
            }
            // Need north and south for single digit numbers
            let mut directions: Vec<&str> = vec!["ne", "e", "se", "n", "s", "sw", "w", "nw"];
            if col == 0 {
                let (no_west_directions, _) = directions.split_at_mut(5);
                directions = no_west_directions.to_vec();
            } else if col == rows[0].len() - 1 {
                let (_, no_east_directions) = directions.split_at_mut(3);
                directions = no_east_directions.to_vec();
            }
            if row == 0 {
                directions.retain(|&x| !x.contains("n"));
            }
            if row == rows.len() - 1 {
                directions.retain(|&x| !x.contains("s"));
            }
            // Bugs are likely
            // A multi-digit num directly above/below a symbol is counted once
            if row > 0 && 0 < col && col < rows[0].len() {
                let north = rows[row - 1][col];
                let ne = rows[row - 1][col + 1];
                let nw = rows[row - 1][col - 1];
                if north == ne && ne == nw {
                    // Arbitrarily remove NW or NE
                    directions.retain(|&x| x != "nw" && x != "n");
                } else if north == ne || north == nw {
                    directions.retain(|&x| x != "n");
                }
            }
            if row < rows.len() - 1 && 0 < col && col < rows[0].len() {
                let south = rows[row + 1][col];
                let se = rows[row + 1][col + 1];
                let sw = rows[row + 1][col - 1];
                if south == se && se == sw {
                    // Arbitrarily remove SW or SE
                    directions.retain(|&x| x != "sw" && x != "s");
                } else if south == se || south == sw {
                    directions.retain(|&x| x != "s");
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
                    "n" => val = rows[row - 1][col],
                    "s" => val = rows[row + 1][col],
                    &_ => panic!("invalid direction"),
                }
                if val > 0 {
                    total += val;
                }
            }
        }
    }
    return total;
}

pub fn day_3_2(text: &str) -> i32 {
    // Return sum of numbers adjacent to negative numbers in a 2D matrix
    let mut total: i32 = 0;
    let rows = build_2d_matrix(text);

    for row in 0..rows.len() {
        if rows[row].len() != rows[0].len() {
            panic!("vectors are not the same length");
        }
        for col in 0..rows[0].len() {
            if rows[row][col] != -2 {
                continue;
            }
            // Need north and south for single digit numbers
            let mut directions: Vec<&str> = vec!["ne", "e", "se", "n", "s", "sw", "w", "nw"];
            if col == 0 {
                let (no_west_directions, _) = directions.split_at_mut(5);
                directions = no_west_directions.to_vec();
            } else if col == rows[0].len() - 1 {
                let (_, no_east_directions) = directions.split_at_mut(3);
                directions = no_east_directions.to_vec();
            }
            if row == 0 {
                directions.retain(|&x| !x.contains("n"));
            }
            if row == rows.len() - 1 {
                directions.retain(|&x| !x.contains("s"));
            }
            // Bugs are likely
            // A multi-digit num directly above/below a symbol is counted once
            if row > 0 && 0 < col && col < rows[0].len() {
                let north = rows[row - 1][col];
                let ne = rows[row - 1][col + 1];
                let nw = rows[row - 1][col - 1];
                if north == ne && ne == nw {
                    // Arbitrarily remove NW or NE
                    directions.retain(|&x| x != "nw" && x != "n");
                } else if north == ne || north == nw {
                    directions.retain(|&x| x != "n");
                }
            }
            if row < rows.len() - 1 && 0 < col && col < rows[0].len() {
                let south = rows[row + 1][col];
                let se = rows[row + 1][col + 1];
                let sw = rows[row + 1][col - 1];
                if south == se && se == sw {
                    // Arbitrarily remove SW or SE
                    directions.retain(|&x| x != "sw" && x != "s");
                } else if south == se || south == sw {
                    directions.retain(|&x| x != "s");
                }
            }

            let mut count_adjacent_nums = 0;
            let mut pdt = 1;

            for direction in directions {
                let val;
                match direction {
                    "ne" => val = rows[row - 1][col + 1],
                    "e" => val = rows[row][col + 1],
                    "se" => val = rows[row + 1][col + 1],
                    "sw" => val = rows[row + 1][col - 1],
                    "w" => val = rows[row][col - 1],
                    "nw" => val = rows[row - 1][col - 1],
                    "n" => val = rows[row - 1][col],
                    "s" => val = rows[row + 1][col],
                    &_ => panic!("invalid direction"),
                }
                if val > 0 {
                    pdt *= val;
                    count_adjacent_nums += 1;
                }
            }
            if count_adjacent_nums == 2 {
                total += pdt;
            }
        }
    }
    return total;
}
