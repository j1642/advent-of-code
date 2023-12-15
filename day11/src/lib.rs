pub fn day_11_1(text: &str) -> u32 {
    // Return sum of shortest distances between galaxies (#)
    // Find empty rows/cols and add a new one after each one
    let mut are_cols_empty: Vec<bool> = vec![];
    // Lots of room to minimize loops over the input
    let mut matrix: Vec<Vec<char>> = vec![];

    // O(text.len()) insertion of new, empty rows
    for line in text.lines() {
        let mut is_row_empty = true;
        let mut row = vec![];
        for (j, c) in line.chars().enumerate() {
            row.push(c);
            if j == are_cols_empty.len() {
                are_cols_empty.push(true);
            }
            if c == '#' {
                is_row_empty = false;
                are_cols_empty[j] = false;
            }
        }

        if is_row_empty {
            matrix.push(row.clone());
            matrix.push(row);
        } else {
            matrix.push(row);
        }
    }

    // Get list of all galaxy coordinates
    let mut galaxy_coords: Vec<(i32, i32)> = vec![];
    for row in 0..matrix.len() {
        for col in 0..matrix[0].len() {
            if matrix[row][col] == '#' {
                galaxy_coords.push((row as i32, col as i32));
            }
        }
    }

    // O(cols) to O(cols*galaxies) adjust for new, empty cols
    for (col, is_col_empty) in are_cols_empty.iter().enumerate().rev() {
        if *is_col_empty {
            for i in 0..galaxy_coords.len() {
                if galaxy_coords[i].1 > (col as i32) {
                    galaxy_coords[i].1 += 1;
                }
            }
        }
    }
    /*
    for row in 0..matrix.len() {
        println!("{:?}", matrix[row]);
    }
    */
    // Loop over each pair of galaxies (1/2 n^2 style) to find distance
    let mut distance_sum = 0;
    for i in 0..galaxy_coords.len() - 1 {
        for j in (i + 1)..galaxy_coords.len() {
            let mut x_diff = galaxy_coords[i].0 - galaxy_coords[j].0;
            if x_diff < 0 {
                x_diff *= -1;
            }
            let mut y_diff = galaxy_coords[i].1 - galaxy_coords[j].1;
            if y_diff < 0 {
                y_diff *= -1;
            }
            distance_sum += x_diff + y_diff
        }
    }
    // Return sum of distances
    return distance_sum as u32;
}
