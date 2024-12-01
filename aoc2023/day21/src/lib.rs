use std::collections::{HashSet, VecDeque};

pub fn day21_1(text: &str, steps: u32, dimension: u32) -> usize {
    // Return the amount of possible positions after taking X orthogonal
    // steps from the starting position
    //let mut matrix: Vec<Vec<u8>> = build_matrix(text);
    let mut matrix: Vec<Vec<u8>> = build_n_by_n_matrix(text, dimension);
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
        if matrix[row][col] == 3 {
            continue;
        }
        matrix[row][col] = 3;
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

pub fn day21_2(text: &str, steps: u32, matrix_dimension: u32) -> usize {
    // Return the amount of possible positions after taking X orthogonal
    // steps from the starting position, assuming the input is
    // infinitely repeating horizontally and vertically
    let matrix: Vec<Vec<u8>> = build_n_by_n_matrix(text, matrix_dimension);
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
    let mut new_coord: Coord;
    // TODO: limit travel directions (keep it in first quadrant) and multiply
    // to get final answer
    while let Some((coord, steps_remaining)) = q.pop_front() {
        orig_row = coord.row;
        orig_col = coord.col;

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

        // North
        new_coord = Coord {
            row: orig_row - 1,
            col: orig_col,
        };
        if !checked.contains(&new_coord) && !coord_is_wall(&new_coord, &matrix) {
            q.push_back((new_coord, steps_remaining - 1));
        }
        // South
        new_coord = Coord {
            row: orig_row + 1,
            col: orig_col,
        };
        if !checked.contains(&new_coord) && !coord_is_wall(&new_coord, &matrix) {
            q.push_back((new_coord, steps_remaining - 1));
        }
        // East
        new_coord = Coord {
            row: orig_row,
            col: orig_col + 1,
        };
        if !checked.contains(&new_coord) && !coord_is_wall(&new_coord, &matrix) {
            q.push_back((new_coord, steps_remaining - 1));
        }
        // West
        new_coord = Coord {
            row: orig_row,
            col: orig_col - 1,
        };
        if !checked.contains(&new_coord) && !coord_is_wall(&new_coord, &matrix) {
            q.push_back((new_coord, steps_remaining - 1));
        }
    }

    return count;
}

fn coord_is_wall(coord: &Coord, matrix: &Vec<Vec<u8>>) -> bool {
    let matrix_len = matrix.len() as i32;
    let mut row = coord.row % matrix_len;
    if row < 0 {
        // Change -1 to width_height - 1
        row += matrix_len;
    }

    let mut col = coord.col % matrix_len;
    if col < 0 {
        col += matrix_len;
    }

    return matrix[row as usize][col as usize] == 1;
}

fn add_nsew_to_queue(
    matrix: &Vec<Vec<u8>>,
    q: &mut VecDeque<(Coord, u32)>,
    coord: Coord,
    steps_remaining: u32,
) {
    let row = coord.row as usize;
    let col = coord.col as usize;
    let mut new_coord;
    // North
    if row > 0 && matrix[row - 1][col] == 0 {
        new_coord = Coord {
            row: (row - 1) as i32,
            col: col as i32,
        };
        q.push_back((new_coord, steps_remaining - 1));
    }
    // South
    if row < matrix.len() - 1 && matrix[row + 1][col] == 0 {
        new_coord = Coord {
            row: (row + 1) as i32,
            col: col as i32,
        };
        q.push_back((new_coord, steps_remaining - 1));
    }
    // East
    if col < matrix[0].len() - 1 && matrix[row][col + 1] == 0 {
        new_coord = Coord {
            row: row as i32,
            col: (col + 1) as i32,
        };
        q.push_back((new_coord, steps_remaining - 1));
    }
    // West
    if col > 0 && matrix[row][col - 1] == 0 {
        new_coord = Coord {
            row: row as i32,
            col: (col - 1) as i32,
        };
        q.push_back((new_coord, steps_remaining - 1));
    }
}

fn build_matrix(text: &str) -> Vec<Vec<u8>> {
    let mut matrix = vec![];
    for line in text.lines() {
        let mut row: Vec<u8> = vec![];
        for c in line.chars() {
            match c {
                '.' => {
                    row.push(0);
                }
                '#' => {
                    row.push(1);
                }
                'S' => {
                    row.push(0);
                }
                _ => {
                    panic!();
                }
            }
        }
        matrix.push(row);
    }
    return matrix;
}

fn build_n_by_n_matrix(text: &str, mut n: u32) -> Vec<Vec<u8>> {
    n -= 1;
    let mut matrix = build_matrix(text);
    // Change start coord from 'S' to '.'
    let orig_width_height = matrix.len();

    // Increase width
    for i in 0..matrix.len() {
        for _ in 0..n {
            matrix[i].extend_from_within(0..orig_width_height);
        }
    }
    // Increase height
    for _ in 0..n {
        matrix.extend_from_within(0..orig_width_height);
    }

    return matrix;
}

#[derive(Debug, Hash, Eq, PartialEq)]
struct Coord {
    row: i32,
    col: i32,
}
