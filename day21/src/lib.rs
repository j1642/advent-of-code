use std::collections::{HashSet, VecDeque};

pub fn day21_1(text: &str, steps: u32) -> usize {
    // Return the amount of possible positions after taking X orthogonal
    // steps from the starting position
    let mut matrix: Vec<Vec<char>> = build_matrix(text);
    // Assume the input is a square with odd-length sides,
    // and the start coord is the middle coordinate
    assert_eq!(matrix.len(), matrix[0].len());
    let start_coord = Coord {
        row: (matrix.len() / 2) as i32,
        col: (matrix[0].len() / 2) as i32,
    };
    let mut count = 0;

    let mut q: VecDeque<(Coord, u32)> = VecDeque::new();
    q.push_back((start_coord, steps));

    let mut row;
    let mut col;
    while let Some((coord, steps_remaining)) = q.pop_front() {
        row = coord.row as usize;
        col = coord.col as usize;
        if matrix[row][col] == 'X' {
            continue;
        }
        matrix[row][col] = 'X';
        if (steps - steps_remaining) % 2 == steps % 2 {
            count += 1;
        }
        if steps_remaining < 1 {
            continue;
        }

        add_nsew_to_queue(&matrix, &mut q, coord, steps_remaining);
    }

    return count;
}

pub fn day21_2(text: &str, steps: u32) -> usize {
    // Return the amount of possible positions after taking X orthogonal
    // steps from the starting position, assuming the input is
    // infinitely repeating horizontally and vertically
    //
    // TODO: use u8 in matrix instead of 4-byte chars
    let matrix: Vec<Vec<char>> = build_5x5_matrix(text);
    assert_eq!(matrix.len(), matrix[0].len());
    let start_coord = Coord {
        row: (matrix.len() / 2) as i32,
        col: (matrix[0].len() / 2) as i32,
    };
    let mut count = 0;
    let width_height = matrix.len() as i32;

    let mut checked: HashSet<Coord> = HashSet::new();
    let mut q: VecDeque<(Coord, u32)> = VecDeque::new();
    q.push_back((start_coord, steps));

    let mut orig_row: i32;
    let mut orig_col: i32;
    while let Some((coord, steps_remaining)) = q.pop_front() {
        orig_row = coord.row;
        orig_col = coord.col;
        let mut row = orig_row % width_height;
        if row < 0 {
            // Change -1 to width_height - 1
            row += width_height;
        }
        let row = row as usize;

        let mut col = orig_col % width_height;
        if col < 0 {
            col += width_height;
        }
        let col = col as usize;

        if checked.contains(&coord) || matrix[row][col] == '#' {
            continue;
        }

        checked.insert(coord);
        if (steps - steps_remaining) % 2 == steps % 2 {
            count += 1;
        }
        if steps_remaining < 1 {
            continue;
        }

        let mut new_coord;
        // TODO: use modulo to wrap-around instead of hitting the matrix edge
        // TODO: hashset is getting the same coords several times b/c coords are "fixed" before
        // getting added to the queue, when coords need to be fixed after being popped
        //
        // Omit matrix[][] == '.' check b/c borders are always '.'
        // North
        new_coord = Coord {
            row: orig_row - 1,
            col: orig_col,
        };
        q.push_back((new_coord, steps_remaining - 1));
        // South
        new_coord = Coord {
            row: orig_row + 1,
            col: orig_col,
        };
        q.push_back((new_coord, steps_remaining - 1));
        // East
        new_coord = Coord {
            row: orig_row,
            col: orig_col + 1,
        };
        q.push_back((new_coord, steps_remaining - 1));
        // West
        new_coord = Coord {
            row: orig_row,
            col: orig_col - 1,
        };
        q.push_back((new_coord, steps_remaining - 1));
    }

    return count;
}

fn add_nsew_to_queue(
    matrix: &Vec<Vec<char>>,
    q: &mut VecDeque<(Coord, u32)>,
    coord: Coord,
    steps_remaining: u32,
) {
    let row = coord.row as usize;
    let col = coord.col as usize;
    let mut new_coord;
    // North
    if row > 0 && matrix[row - 1][col] == '.' {
        new_coord = Coord {
            row: (row - 1) as i32,
            col: col as i32,
        };
        q.push_back((new_coord, steps_remaining - 1));
    }
    // South
    if row < matrix.len() - 1 && matrix[row + 1][col] == '.' {
        new_coord = Coord {
            row: (row + 1) as i32,
            col: col as i32,
        };
        q.push_back((new_coord, steps_remaining - 1));
    }
    // East
    if col < matrix[0].len() - 1 && matrix[row][col + 1] == '.' {
        new_coord = Coord {
            row: row as i32,
            col: (col + 1) as i32,
        };
        q.push_back((new_coord, steps_remaining - 1));
    }
    // West
    if col > 0 && matrix[row][col - 1] == '.' {
        new_coord = Coord {
            row: row as i32,
            col: (col - 1) as i32,
        };
        q.push_back((new_coord, steps_remaining - 1));
    }
}

fn build_matrix(text: &str) -> Vec<Vec<char>> {
    let mut matrix = vec![];
    for line in text.lines() {
        let row = line.chars().collect::<Vec<char>>();
        matrix.push(row);
    }
    return matrix;
}

fn build_5x5_matrix(text: &str) -> Vec<Vec<char>> {
    let mut matrix = build_matrix(text);
    // Change start coord from 'S' to '.'
    let orig_width_height = matrix.len();
    let middle_row_idx = matrix.len() / 2;
    let middle_col_idx = matrix[0].len() / 2;
    matrix[middle_row_idx][middle_col_idx] = '.';

    // Change width to 5x original width
    for i in 0..matrix.len() {
        matrix[i].extend_from_within(0..orig_width_height);
        matrix[i].extend_from_within(0..orig_width_height);
        matrix[i].extend_from_within(0..orig_width_height);
        matrix[i].extend_from_within(0..orig_width_height);
    }
    // Change height to 5x original height
    matrix.extend_from_within(0..orig_width_height);
    matrix.extend_from_within(0..orig_width_height);
    matrix.extend_from_within(0..orig_width_height);
    matrix.extend_from_within(0..orig_width_height);

    return matrix;
}

#[derive(Debug, Hash, Eq, PartialEq)]
struct Coord {
    row: i32,
    col: i32,
}
