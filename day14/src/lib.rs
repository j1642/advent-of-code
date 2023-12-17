pub fn day_14_1(text: &str) -> usize {
    // Return total structural load after all round rocks roll north
    let mut load = 0;
    let mut matrix = build_matrix(text);

    roll_north(&mut matrix);

    for row in 0..matrix.len() {
        for col in 0..matrix[0].len() {
            if matrix[row][col] == 'O' {
                load += matrix.len() - row;
            }
        }
    }

    return load;
}

pub fn day_14_2(text: &str) -> usize {
    // Return total structural load after all round rocks roll N,W,S,E 1 billion times
    let mut load = 0;
    let mut matrix = build_matrix(text);

    let mut empty_ind: usize;

    // The final state is likely identical for 1000 cycles and 1 billion cycles
    for _ in 0..1000 {
        roll_north(&mut matrix);
        // Roll west
        for row in 0..matrix.len() {
            empty_ind = 0;
            for col in 0..matrix[0].len() {
                if matrix[row][col] == '#' {
                    if col == matrix[0].len() - 1 {
                        break;
                    }
                    empty_ind = col + 1;
                } else if matrix[row][col] == 'O' {
                    matrix[row][col] = '.';
                    matrix[row][empty_ind] = 'O';
                    if empty_ind == matrix[0].len() - 1 {
                        break;
                    }
                    empty_ind += 1;
                }
            }
        }
        // Roll south, starting with the southern rows to avoid collisions
        for col in 0..matrix[0].len() {
            empty_ind = matrix.len() - 1;
            for row in (0..matrix.len()).rev() {
                if matrix[row][col] == '#' {
                    if row == 0 {
                        break;
                    }
                    empty_ind = row - 1;
                } else if matrix[row][col] == 'O' {
                    matrix[row][col] = '.';
                    matrix[empty_ind][col] = 'O';
                    if empty_ind == 0 {
                        break;
                    }
                    empty_ind -= 1;
                }
            }
        }
        // Roll east, starting with the eastern columns to avoid collision
        for row in 0..matrix.len() {
            empty_ind = matrix[0].len() - 1;
            for col in (0..matrix[0].len()).rev() {
                if matrix[row][col] == '#' {
                    if col == 0 {
                        break;
                    }
                    empty_ind = col - 1;
                } else if matrix[row][col] == 'O' {
                    matrix[row][col] = '.';
                    matrix[row][empty_ind] = 'O';
                    if empty_ind == 0 {
                        break;
                    }
                    empty_ind -= 1;
                }
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

    return load;
}

fn roll_north(matrix: &mut Vec<Vec<char>>) {
    // Roll north, starting with the northern rocks
    let mut empty_ind;
    for col in 0..matrix[0].len() {
        empty_ind = 0;
        for row in 0..matrix.len() {
            if empty_ind > matrix.len() - 1 {
                break;
            }
            if matrix[row][col] == '#' {
                empty_ind = row + 1;
            } else if matrix[row][col] == 'O' {
                matrix[row][col] = '.';
                matrix[empty_ind][col] = 'O';
                empty_ind += 1;
            }
        }
    }
}

fn build_matrix(text: &str) -> Vec<Vec<char>> {
    let mut matrix: Vec<Vec<char>> = vec![];
    for line in text.lines() {
        let row: Vec<char> = line.chars().collect();
        matrix.push(row);
    }
    return matrix;
}
