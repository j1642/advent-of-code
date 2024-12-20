pub fn day06_1(text: &str) -> i32 {
    let mut matrix = get_matrix(text);
    let (mut cur_direction, mut cur_row, mut cur_col) = find_cur_direction_location(&matrix).unwrap();

    matrix[cur_row][cur_col] = 'X';

    while cur_row < matrix.len() && cur_col < matrix[0].len() {
        if cur_direction == Direction::Up && cur_row == 0 {
            break;
        } else if cur_direction == Direction::Left && cur_col == 0 {
            break;
        }

        (cur_row, cur_col, cur_direction) = move_along_cur_direction(&matrix, cur_row, cur_col, cur_direction);

        if cur_row < matrix.len() && cur_col < matrix[0].len() {
            matrix[cur_row][cur_col] = 'X';
        }
    }

    // Count locations visited by the patrolling guard
    let mut count = 0;
    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
            if matrix[i][j] == 'X' {
                count += 1;
            }
        }
    }

    return count;
}

#[derive(PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn find_cur_direction_location(matrix: &Vec<Vec<char>>) -> Option<(Direction, usize, usize)> {
    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
            let c = matrix[i][j];
            match c {
                'v' => return Some((Direction::Down, i, j)),
                '<' => return Some((Direction::Left, i, j)),
                '^' => return Some((Direction::Up, i, j)),
                '>' => return Some((Direction::Right, i, j)),
                _ => {}
            }
        }
    }
    return None
}

// Rotate clockwise to next direction
fn change_direction(cur: Direction) -> Direction {
    return match cur {
        Direction::Up => Direction::Right,
        Direction::Right => Direction::Down,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
    }
}

fn move_along_cur_direction(matrix: &Vec<Vec<char>>, mut cur_row: usize, mut cur_col: usize, mut cur_direction: Direction) -> (usize, usize, Direction) {
    // usize < 0 edge cases should be precluded already
    let mut next_row = cur_row;
    let mut next_col = cur_col;
    match cur_direction {
        Direction::Up => next_row -= 1,
        Direction::Down => next_row += 1,
        Direction::Left => next_col -= 1,
        Direction::Right => next_col +=1,
    };

    if next_row >= matrix.len() || next_col >= matrix[0].len() {
        return (next_row, next_col, cur_direction);
    } else if matrix[next_row][next_col] == '#' {
        cur_direction = change_direction(cur_direction);
    } else {
        cur_row = next_row;
        cur_col = next_col;
    }

    return (cur_row, cur_col, cur_direction);
}

fn get_matrix(text: &str) -> Vec<Vec<char>> {
    let mut rows: Vec<Vec<char>> = vec![];
    for line in text.lines() {
        let row = line.chars().collect::<Vec<char>>();
        rows.push(row);
    }
    return rows;
}
