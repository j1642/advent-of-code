use std::collections::VecDeque;

pub fn day21_1(text: &str, steps: u32) -> usize {
    // Return the amount of possible positions after taking X orthogonal
    // steps from the starting position
    let mut matrix: Vec<Vec<char>> = build_matrix(text);
    let start_coord = find_start_coord(&matrix).unwrap();
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
                return Some(Coord { row: i, col: j });
            }
        }
    }
    return None;
}
