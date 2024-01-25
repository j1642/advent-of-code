use std::collections::{HashSet, VecDeque};
pub fn day21_1(text: &str, steps: u32) -> usize {
    // Return the amount of possible positions after taking X orthogonal
    // steps from the starting position
    let mut matrix: Vec<Vec<char>> = build_matrix(text);
    let start_coord = find_start_coord(&matrix).unwrap();
    matrix[start_coord.row][start_coord.col] = 'X';

    let mut unique_coords: HashSet<Coord> = HashSet::new();
    if steps % 2 == 0 {
        unique_coords.insert(Coord{row:start_coord.row, col: start_coord.col});
    }

    let mut q: VecDeque<(Coord, u32)> = VecDeque::new();
    q.push_back((Coord{row: start_coord.row, col: start_coord.col}, steps));

    while q.len() > 0 {
        let (coord, steps_remaining) = q.pop_front().unwrap();
        let row = coord.row;
        let col = coord.col;
        matrix[row][col] = 'X';
        if (steps - steps_remaining) % 2 == steps % 2 {
            unique_coords.insert(Coord{row: row, col: col});
        }
        if steps_remaining < 1 {
            continue;
        }

        let mut new_coord;
        // North
        if row > 0 && matrix[row - 1][col] == '.' {
            new_coord = Coord{row: row - 1, col: col};
            q.push_back((new_coord, steps_remaining - 1));
        }
        // South
        if row < matrix.len() - 1 && matrix[row + 1][col] == '.' {
            new_coord = Coord{row: row + 1, col: col};
            q.push_back((new_coord, steps_remaining - 1));
        }
        // East
        if col < matrix[0].len() - 1 && matrix[row][col + 1] == '.' {
            new_coord = Coord{row: row, col: col + 1};
            q.push_back((new_coord, steps_remaining - 1));
        }
        // West
        if col > 0 && matrix[row][col - 1] == '.' {
            new_coord = Coord{row: row, col: col - 1};
            q.push_back((new_coord, steps_remaining - 1));
        }
    }

    return unique_coords.len();
}

fn build_matrix(text: &str) -> Vec<Vec<char>> {
    let mut matrix = vec![];
    for line in text.lines() {
        let row = line.chars().collect::<Vec<char>>();
        matrix.push(row);
    }
    return matrix;
}

#[derive(Debug, Hash, Eq, PartialEq)]
struct Coord {
    row: usize,
    col: usize,
}

fn find_start_coord(matrix: &Vec<Vec<char>>) -> Option<Coord> {
    for (i, row) in matrix.iter().enumerate() {
        for j in 0..row.len() {
            if matrix[i][j] == 'S' {
                return Some(Coord{row: i, col: j});
            }
        }
    }
    return None;
}
