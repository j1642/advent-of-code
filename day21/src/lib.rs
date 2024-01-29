use std::collections::{HashSet, VecDeque};

pub fn day21_1(text: &str, steps: u32) -> usize {
    // Return the amount of possible positions after taking X orthogonal
    // steps from the starting position
    let mut matrix: Vec<Vec<char>> = build_matrix(text);
    // Assume the input is a square with odd-length sides,
    // and the start coord is the middle coordinate
    assert_eq!(matrix.len(), matrix[0].len());
    let start_coord = Coord {
        row: matrix.len() / 2,
        col: matrix[0].len() / 2,
    };
    let mut count = 0;

    let mut q: VecDeque<(Coord, u32)> = VecDeque::new();
    q.push_back((start_coord, steps));

    while let Some((coord, steps_remaining)) = q.pop_front() {
        let row = coord.row;
        let col = coord.col;
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
    let start_coord = Coord {
        row: matrix.len() / 2,
        col: matrix[0].len() / 2,
    };
    let mut count = 0;
    let width_height = matrix.len();

    let mut checked: HashSet<Coord> = HashSet::new();
    let mut q: VecDeque<(Coord, u32)> = VecDeque::new();
    q.push_back((start_coord, steps));

    while let Some((coord, steps_remaining)) = q.pop_front() {
        // TODO: use modulo to wrap-around instead of hitting the matrix edge
        let row = coord.row;
        let col = coord.col;
        if checked.contains(&coord) {
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
        // North
        if row > 0 && matrix[row - 1][col] == '.' {
            new_coord = Coord {
                row: row - 1,
                col: col,
            };
            q.push_back((new_coord, steps_remaining - 1));
        }
        // South
        if row < matrix.len() - 1 && matrix[row + 1][col] == '.' {
            new_coord = Coord {
                row: row + 1,
                col: col,
            };
            q.push_back((new_coord, steps_remaining - 1));
        }
        // East
        if col < matrix[0].len() - 1 && matrix[row][col + 1] == '.' {
            new_coord = Coord {
                row: row,
                col: col + 1,
            };
            q.push_back((new_coord, steps_remaining - 1));
        }
        // West
        if col > 0 && matrix[row][col - 1] == '.' {
            new_coord = Coord {
                row: row,
                col: col - 1,
            };
            q.push_back((new_coord, steps_remaining - 1));
        }
    }

    return count;
}

fn add_nsew_to_queue(
    matrix: &Vec<Vec<char>>,
    q: &mut VecDeque<(Coord, u32)>,
    coord: Coord,
    steps_remaining: u32,
) {
    let row = coord.row;
    let col = coord.col;
    let mut new_coord;
    // North
    if row > 0 && matrix[row - 1][col] == '.' {
        new_coord = Coord {
            row: row - 1,
            col: col,
        };
        q.push_back((new_coord, steps_remaining - 1));
    }
    // South
    if row < matrix.len() - 1 && matrix[row + 1][col] == '.' {
        new_coord = Coord {
            row: row + 1,
            col: col,
        };
        q.push_back((new_coord, steps_remaining - 1));
    }
    // East
    if col < matrix[0].len() - 1 && matrix[row][col + 1] == '.' {
        new_coord = Coord {
            row: row,
            col: col + 1,
        };
        q.push_back((new_coord, steps_remaining - 1));
    }
    // West
    if col > 0 && matrix[row][col - 1] == '.' {
        new_coord = Coord {
            row: row,
            col: col - 1,
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
    row: usize,
    col: usize,
}
