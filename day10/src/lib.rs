pub fn day_10_1(text: &str) -> u32 {
    let mut matrix: Vec<Vec<char>> = vec![];
    let mut start = (0, 0);

    for (i, line) in text.lines().enumerate() {
        let mut row: Vec<char> = vec![];
        row.push('.');
        for (j, c) in line.chars().enumerate() {
            if c == 'S' {
                // Adjust start indices because of added '.' rows and columns
                start = (i+1, j+1);
                row.push('.');
            } else {
                row.push(c);
            }
        }
        row.push('.');
        matrix.push(row);
    }
    matrix.insert(0, vec!['.'; matrix[0].len()]);
    matrix.push(vec!['.'; matrix[0].len()]);

    return find_midpoint_distance(&mut matrix, start);
}

fn find_midpoint_distance(matrix: &mut Vec<Vec<char>>, start: (usize, usize)) -> u32 {
    // Erase path to location loc1 so loc2 must follow a different path
    let moves = moves_from_start(&matrix, start);
    matrix[start.0][start.1] = '.';

    let mut loc1 = moves[0];
    let mut loc2 = moves[1];
    
    let mut count_steps = 1;
    while loc1 != loc2 {
        loc1 = step(matrix, loc1);
        loc2 = step(matrix, loc2);
        count_steps += 1;
    }

    return count_steps;
}

fn step(matrix: &mut Vec<Vec<char>>, curr: (usize, usize)) -> (usize, usize) {
    let pipes = "L|7J-F";
    let next;

    match matrix[curr.0][curr.1] {
        'L' => {
            if pipes.contains(matrix[curr.0][curr.1 + 1]) {
                next = (curr.0, curr.1 + 1);
            } else {
                next = (curr.0 - 1, curr.1);
            }
        }
        '|' => {
            if pipes.contains(matrix[curr.0 - 1][curr.1]) {
                next = (curr.0 - 1, curr.1);
            } else {
                next = (curr.0 + 1, curr.1);
            }
        }
        '7' => {
            if pipes.contains(matrix[curr.0][curr.1 - 1]) {
                next = (curr.0, curr.1 - 1);
            } else {
                next = (curr.0 + 1, curr.1);
            }
        }
        'J' => {
            if pipes.contains(matrix[curr.0][curr.1 - 1]) {
                next = (curr.0, curr.1 - 1);
            } else {
                next = (curr.0 - 1, curr.1);
            }
        }
        '-' => {
            if pipes.contains(matrix[curr.0][curr.1 - 1]) {
                next = (curr.0, curr.1 - 1);
            } else {
                next = (curr.0, curr.1 + 1);
            }
        }
        'F' => {
            if pipes.contains(matrix[curr.0][curr.1 + 1]) {
                next = (curr.0, curr.1 + 1);
            } else {
                next = (curr.0 + 1, curr.1);
            }
        }
        _ => panic!("invalid current char"),
    }

    matrix[curr.0][curr.1] = '.';
    return next;
}

fn moves_from_start(matrix: &Vec<Vec<char>>, start: (usize, usize)) -> [(usize, usize); 2] {
    let mut moves = [(0, 0); 2];
    let mut empty_index = 0;
    // North
    if "7F|".contains(matrix[start.0 - 1][start.1]) {
        moves[empty_index] = (start.0 - 1, start.1);
        empty_index += 1;
    }
    // South
    if "LJ|".contains(matrix[start.0 + 1][start.1]) {
        moves[empty_index] = (start.0 + 1, start.1);
        empty_index += 1;
    }
    // East
    if "-J7".contains(matrix[start.0][start.1 + 1]) {
        moves[empty_index] = (start.0, start.1 + 1);
        empty_index += 1;
    }
    // West
    if "-FL".contains(matrix[start.0][start.1 - 1]) {
        moves[empty_index] = (start.0, start.1 - 1);
        empty_index += 1;
    }

    if empty_index > 2 {
        panic!("empty_index {empty_index} should be <= 2");
    }

    return moves;
}
