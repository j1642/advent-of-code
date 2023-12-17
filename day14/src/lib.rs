pub fn day_14_1(text: &str) -> usize {
    // Return total "load" after all round rocks roll north
    let mut load = 0;
    let mut matrix = build_matrix(text);

    for row in 0..matrix.len() {
        for col in 0..matrix[0].len() {
            if matrix[row][col] == 'O' && row > 0 {

                let mut prev_row = row - 1;
                let mut curr_row = row;
                while matrix[prev_row][col]  == '.' {
                    matrix[prev_row][col] = 'O';
                    matrix[curr_row][col] = '.';
                    if prev_row == 0 {
                        break;
                    }
                    prev_row -= 1;
                    curr_row -= 1;
                }
                // Not working yet
                //println!("{} - {row}", matrix.len());
                //load += matrix.len() - curr_row;
            }
        }
    }
    for row in 0..matrix.len() {
        for col in 0..matrix[0].len() {
            if matrix[row][col] == 'O' {
                load += matrix.len() - row;
            }
        }
    }
    for row in matrix {
        println!("{:?}", row);
    }
    return load;
}

fn build_matrix(text: &str) -> Vec<Vec<char>> {
    let mut matrix: Vec<Vec<char>> = vec![];
    for line in text.lines() {
        let row: Vec<char> = line.chars().collect();
        matrix.push(row);
    }
    return matrix;
}
