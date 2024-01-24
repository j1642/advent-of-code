use std::collections::HashSet;
pub fn day21_1(text: &str, steps: u32) -> usize {
    // Return the amount of possible positions after taking X orthogonal
    // steps from the starting position
    let mut matrix: Vec<Vec<char>> = build_matrix(text);
    let start_coord = find_start_coord(&matrix).unwrap();
    matrix[start_coord.row][start_coord.col] = '.';

    let mut coords = step(&matrix, start_coord, steps);
    let mut unique_coords: HashSet<Coord> = HashSet::new();

    for _ in 0..coords.len() {
        unique_coords.insert(coords.pop().unwrap());
    }

    //println!("coords: {:?}", unique_coords);

    return unique_coords.len();
}

fn step(matrix: &Vec<Vec<char>>, coord: Coord, steps_remaining: u32) -> Vec<Coord> {
    // Recursively step from the given coord in unobstructed orthogonal directions
    if steps_remaining < 1 {
        return vec![coord];
    }

    let mut recursive_coords = vec![];
    let mut new_coord;
    let row = coord.row;
    let col = coord.col;
    // North
    if row > 0 && matrix[row - 1][col] != '#' {
        new_coord = Coord{row: row - 1, col: col};
        recursive_coords.append(&mut step(matrix, new_coord, steps_remaining - 1));
    }
    // South
    if row < matrix.len() - 1 && matrix[row + 1][col] != '#' {
        new_coord = Coord{row: row + 1, col: col};
        recursive_coords.append(&mut step(matrix, new_coord, steps_remaining - 1));
    }
    // East
    if col < matrix[0].len() - 1 && matrix[row][col + 1] != '#' {
        new_coord = Coord{row: row, col: col + 1};
        recursive_coords.append(&mut step(matrix, new_coord, steps_remaining - 1));
    }
    // West
    if col > 0 && matrix[row][col - 1] != '#' {
        new_coord = Coord{row: row, col: col - 1};
        recursive_coords.append(&mut step(matrix, new_coord, steps_remaining - 1));
    }

    //final_coords.append(&mut recursive_coords)

    return recursive_coords;
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
