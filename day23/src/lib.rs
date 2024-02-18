#[derive(Debug, PartialEq, Clone)]
struct Coord {
    row: usize,
    col: usize,
}

pub fn day23_1(text: &str) -> u32 {
    let mut matrix = build_matrix(text);

    let mut start = Coord { row: 0, col: 0 };
    for (i, c) in matrix[0].iter().enumerate() {
        if *c == '.' {
            start.col = i;
            break;
        }
    }
    let mut end = Coord {
        row: matrix.len() - 1,
        col: 0,
    };
    for (i, c) in matrix[matrix.len() - 1].iter().enumerate() {
        if *c == '.' {
            end.col = i;
            break;
        }
    }

    let mut vertices = vec![start.clone(), end.clone()];
    // Search the maze for intersections
    // Assume all intersections are surrounded by "v<>" one-way paths
    // Assume accessing a non-existent index will not happen
    for row in 0..matrix.len() {
        for col in 0..matrix[0].len() {
            // Only search down and to the right to prevent duplicates
            if matrix[row][col] == 'v' && matrix[row + 2][col] == 'v' {
                vertices.push(Coord {
                    row: row + 1,
                    col: col,
                });
            } else if matrix[row][col] == '>' && matrix[row][col + 2] == '>' {
                add_if_distinct(
                    Coord {
                        row: row,
                        col: col + 1,
                    },
                    &mut vertices,
                );
            }
        }
    }

    // Prevent recursing out of the matrix
    matrix[start.row][start.col] = 'x';
    start.row += 1;

    // TODO: Can't prevent all back-tracking b/c the longest path may use some of the same squares
    // as a shorter path
    // New approach: make a graph, each intersection of <>v is a vertex,
    // and the path b/w each intersection is a edge with a weight (distance)
    let mut edge_weights: Vec<Vec<u32>> = vec![vec![0; vertices.len()]; vertices.len()];
    let result = step(
        1,
        start,
        &mut matrix,
        &end,
        &mut vertices,
        &mut edge_weights,
        0,
    );

    /*
    for row in matrix {
        println!("{:?}", row);
    }
    */
    println!("{:?}", edge_weights);

    return 0;
}

fn step(
    mut count: u32,
    position: Coord,
    matrix: &mut Vec<Vec<char>>,
    end: &Coord,
    vertices: &mut Vec<Coord>,
    edge_weights: &mut Vec<Vec<u32>>,
    mut last_visited_idx: usize,
) {
    if matrix[position.row][position.col] == '#' {
        return;
    } else if matrix[position.row][position.col] == 'x' {
        return;
    }

    println!("{:?}", position);

    count += 1;

    if vertices.contains(&position) {
        for (i, vertex) in vertices.iter().enumerate() {
            if &position == vertex {
                edge_weights[i][last_visited_idx] = count;
                edge_weights[last_visited_idx][i] = count;
                last_visited_idx = i;
            }
        }
        // once a weight is added, remove all 'x' from the matrix
        for row in 0..matrix.len() {
            for col in 0..matrix[0].len() {
                if matrix[row][col] == 'x' {
                    matrix[row][col] = '.';
                }
            }
        }
        if end == &position {
            return;
        }
        count = 0;
    }

    /*
    for x in &mut *matrix {
        println!("{:?}", x);
    }
    */

    match matrix[position.row][position.col] {
        // omit '^' implementation because it is not present in the input
        '^' => {
            panic!("'^' should not be present")
        }
        '>' => {
            let new_pos = Coord {
                row: position.row,
                col: position.col + 1,
            };
            return step(count, new_pos, matrix, end, vertices, edge_weights, last_visited_idx);
        }
        'v' => {
            let new_pos = Coord {
                row: position.row + 1,
                col: position.col,
            };
            return step(count, new_pos, matrix, end, vertices, edge_weights, last_visited_idx);
        }
        '<' => {
            let new_pos = Coord {
                row: position.row,
                col: position.col - 1,
            };
            return step(count, new_pos, matrix, end, vertices, edge_weights, last_visited_idx);
        }
        _ => {}
    }

    matrix[position.row][position.col] = 'x';

    // North, east, south, west
    //let mut path_lengths = [0; 4];

    let mut new_pos = Coord {
        row: position.row - 1,
        col: position.col,
    };
    //path_lengths[0] = step(count, new_pos, matrix, end, vertices, edge_weights, last_visited_idx.clone());
    step(count, new_pos, matrix, end, vertices, edge_weights, last_visited_idx.clone());

    new_pos = Coord {
        row: position.row,
        col: position.col + 1,
    };
    //path_lengths[1] = step(count, new_pos, matrix, end, vertices, edge_weights, last_visited_idx.clone());
    step(count, new_pos, matrix, end, vertices, edge_weights, last_visited_idx.clone());

    new_pos = Coord {
        row: position.row + 1,
        col: position.col,
    };
    //path_lengths[2] = step(count, new_pos, matrix, end, vertices, edge_weights, last_visited_idx.clone());
    step(count, new_pos, matrix, end, vertices, edge_weights, last_visited_idx.clone());

    new_pos = Coord {
        row: position.row,
        col: position.col - 1,
    };
    //path_lengths[3] = step(count, new_pos, matrix, end, vertices, edge_weights, last_visited_idx.clone());
    step(count, new_pos, matrix, end, vertices, edge_weights, last_visited_idx.clone());
}

fn add_if_distinct(vertex: Coord, vertices: &mut Vec<Coord>) {
    for v in &mut *vertices {
        if vertex == *v {
            return;
        }
    }
    vertices.push(vertex);
}

fn build_matrix(text: &str) -> Vec<Vec<char>> {
    let mut matrix = vec![];

    for line in text.lines() {
        let row = line.chars().collect::<Vec<char>>();
        matrix.push(row);
    }

    return matrix;
}
