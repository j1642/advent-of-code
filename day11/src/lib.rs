pub fn day_11_1(text: &str) -> i32 {
    // Return sum of shortest distances between galaxies (#)
    let mut are_cols_empty: Vec<bool> = vec![];
    let mut are_rows_empty: Vec<bool> = vec![];
    let mut galaxy_coords: Vec<(i32, i32)> = vec![];

    for (row, line) in text.lines().enumerate() {
        let mut is_row_empty = true;
        for (col, chr) in line.chars().enumerate() {
            if col == are_cols_empty.len() {
                are_cols_empty.push(true);
            }
            if chr == '#' {
                is_row_empty = false;
                are_cols_empty[col] = false;
                galaxy_coords.push((row as i32, col as i32));
            }
        }

        are_rows_empty.push(is_row_empty);
    }

    // Adjust for new, empty rows
    for (row, is_row_empty) in are_rows_empty.iter().enumerate().rev() {
        if *is_row_empty {
            for i in 0..galaxy_coords.len() {
                if galaxy_coords[i].0 > (row as i32) {
                    galaxy_coords[i].0 += 1;
                }
            }
        }
    }

    // Adjust for new, empty cols
    for (col, is_col_empty) in are_cols_empty.iter().enumerate().rev() {
        if *is_col_empty {
            for i in 0..galaxy_coords.len() {
                if galaxy_coords[i].1 > (col as i32) {
                    galaxy_coords[i].1 += 1;
                }
            }
        }
    }

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

    return distance_sum;
}
