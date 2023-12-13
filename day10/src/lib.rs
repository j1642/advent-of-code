pub fn day_10_1(text: &str) -> u32 {
    let mut matrix: Vec<Vec<char>> = vec![];
    let mut start = (0, 0);

    for (i, line) in text.lines().enumerate() {
        let mut row: Vec<char> = vec![];
        row.push('.');
        for (j, c) in line.chars().enumerate() {
            if c == 'S' {
                // Adjust start indices because of added '.' rows and columns
                start = (i + 1, j + 1);
                row.push('X');
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

pub fn day_10_2(text: &str) -> u32 {
    // Find squares bounded by the path
    let mut matrix: Vec<Vec<char>> = vec![];
    let mut start = (0, 0);

    for (i, line) in text.lines().enumerate() {
        let mut row: Vec<char> = vec![];
        row.push('.');
        for (j, c) in line.chars().enumerate() {
            if c == 'S' {
                // Adjust start indices because of added '.' rows and columns
                start = (i + 1, j + 1);
                row.push('|');
            } else {
                row.push(c);
            }
        }
        row.push('.');
        matrix.push(row);
    }
    matrix.insert(0, vec!['.'; matrix[0].len()]);
    matrix.push(vec!['.'; matrix[0].len()]);

    let orig_matrix = matrix.clone();

    // 1. follow the path, mark with X
    _ = find_midpoint_distance(&mut matrix, start);
    // 2. flood fill the outside border, including diagonally
    // matrix[0][0] is in an artificial row and column, not real data
    flood_fill_exterior(&mut matrix, 0, 0);
    // TODO: 3. get the original matrix with the junk chars outside the loop removed
    let mut cleaned_matrix = matrix.clone();
    for row in 0..matrix.len() {
        for col in 0..matrix[0].len() {
            if matrix[row][col] == 'X' {
                cleaned_matrix[row][col] = orig_matrix[row][col];
            }
        }
    }

    let mut count_bounded_squares = 0;
    for row in 0..matrix.len() {
        for col in 0..matrix[0].len() {
            if matrix[row][col] != 'X' && matrix[row][col] != 'O' {
                // Even-odd rule for 2D graphics
                if bounded_by_trail(&mut matrix, &cleaned_matrix, row, col) {
                    count_bounded_squares += 1;
                } else {
                    matrix[row][col] = 'O';
                }
            }
        }
    }

    return count_bounded_squares;
}

fn bounded_by_trail(
    matrix: &mut Vec<Vec<char>>,
    cleaned_matrix: &Vec<Vec<char>>,
    row: usize,
    col: usize,
) -> bool {
    // Return whether a point is bounded by the path using the even-odd rule
    let directions: [&str; 2] = ["east", "west"];
    let mut wall_counts: [bool; 2] = [false; 2];
    let mut empty_wc_index = 0;

    for dir in directions {
        if dir == "east" && col == matrix[0].len() - 1 {
            continue;
        } else if dir == "west" && col == 0 {
            continue;
        }

        let mut indices = vec![];
        if dir == "west" {
            indices = (0..col).rev().collect::<Vec<usize>>();
        } else if dir == "east" {
            indices = ((col + 1)..matrix[0].len()).collect::<Vec<usize>>();
        }

        let mut count_walls = 0;
        for i in indices {
            if matrix[row][i] == 'X' {
                // Only |, F...J, and L...7 are walls for the even-odd rule
                if cleaned_matrix[row][i] == '|' {
                    count_walls += 1;
                    continue;
                }
                let wall_pairs = [['F', 'J'], ['L', '7']];
                for wall_pair in wall_pairs {
                    if cleaned_matrix[row][i] == wall_pair[0] {
                        if cleaned_matrix[row][i + 1] == wall_pair[1] {
                            count_walls += 1;
                            continue;
                        }
                        let mut count_dashes = 1;
                        while cleaned_matrix[row][i + count_dashes] == '-' {
                            count_dashes += 1;
                        }
                        if cleaned_matrix[row][i + count_dashes] == wall_pair[1] {
                            count_walls += 1;
                        }
                    }
                }
            } else if matrix[row][i] == 'O' {
                break;
            }
        }
        if count_walls % 2 == 0 {
            wall_counts[empty_wc_index] = false;
            empty_wc_index += 1;
        } else {
            wall_counts[empty_wc_index] = true;
            empty_wc_index += 1;
        }
    }
    return wall_counts.contains(&true);
}

fn flood_fill_exterior(matrix: &mut Vec<Vec<char>>, row: usize, col: usize) {
    matrix[row][col] = 'O';
    // Check all 8 directions for chars besides O and X
    let directions: Vec<&str>;
    if row == 0 {
        if col == 0 {
            directions = vec!["e", "se", "s"];
        } else if col == matrix[0].len() - 1 {
            directions = vec!["s", "sw", "w"];
        } else {
            directions = vec!["e", "se", "s", "sw", "w"];
        }
    } else if row == matrix.len() - 1 {
        if col == 0 {
            directions = vec!["n", "ne", "e"];
        } else if col == matrix[0].len() - 1 {
            directions = vec!["n", "w", "nw"];
        } else {
            directions = vec!["n", "ne", "e", "w", "nw"];
        }
    } else if col == 0 {
        directions = vec!["n", "ne", "e", "se", "s"];
    } else if col == matrix[0].len() - 1 {
        directions = vec!["n", "s", "sw", "w", "nw"];
    } else {
        directions = vec!["n", "ne", "e", "se", "s", "sw", "w", "nw"];
    }

    let mut chr: char;
    let mut next_row: usize;
    let mut next_col: usize;
    for direction in directions {
        match direction {
            "ne" => {
                next_row = row - 1;
                next_col = col + 1;
            }
            "e" => {
                next_row = row;
                next_col = col + 1;
            }
            "se" => {
                next_row = row + 1;
                next_col = col + 1;
            }
            "sw" => {
                next_row = row + 1;
                next_col = col - 1;
            }
            "w" => {
                next_row = row;
                next_col = col - 1;
            }
            "nw" => {
                next_row = row - 1;
                next_col = col - 1;
            }
            "n" => {
                next_row = row - 1;
                next_col = col;
            }
            "s" => {
                next_row = row + 1;
                next_col = col;
            }
            &_ => panic!("invalid direction"),
        }
        chr = matrix[next_row][next_col];
        if chr != 'O' && chr != 'X' && chr != '*' {
            flood_fill_exterior(matrix, next_row, next_col);
        }
    }
}

fn find_midpoint_distance(matrix: &mut Vec<Vec<char>>, start: (usize, usize)) -> u32 {
    // Erase path to location loc1 so loc2 must follow a different path
    let moves = moves_from_start(&matrix, start);
    matrix[start.0][start.1] = 'X';

    let mut loc1 = moves[0];
    let mut loc2 = moves[1];

    let mut count_steps = 1;
    while loc1 != loc2 {
        loc1 = step(matrix, loc1);
        loc2 = step(matrix, loc2);
        count_steps += 1;
    }
    matrix[loc1.0][loc1.1] = 'X';

    return count_steps;
}

fn step(matrix: &mut Vec<Vec<char>>, curr: (usize, usize)) -> (usize, usize) {
    let pipes = "L|7J-F";
    let next;
    let choice1;
    let choice2;

    match matrix[curr.0][curr.1] {
        'L' => {
            choice1 = (curr.0, curr.1 + 1);
            choice2 = (curr.0 - 1, curr.1);
        }
        '|' => {
            choice1 = (curr.0 - 1, curr.1);
            choice2 = (curr.0 + 1, curr.1);
        }
        '7' => {
            choice1 = (curr.0, curr.1 - 1);
            choice2 = (curr.0 + 1, curr.1);
        }
        'J' => {
            choice1 = (curr.0, curr.1 - 1);
            choice2 = (curr.0 - 1, curr.1);
        }
        '-' => {
            choice1 = (curr.0, curr.1 - 1);
            choice2 = (curr.0, curr.1 + 1);
        }
        'F' => {
            choice1 = (curr.0, curr.1 + 1);
            choice2 = (curr.0 + 1, curr.1);
        }
        _ => panic!("invalid current char"),
    }
    if pipes.contains(matrix[choice1.0][choice1.1]) {
        next = choice1;
    } else {
        next = choice2;
    }

    matrix[curr.0][curr.1] = 'X';

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
