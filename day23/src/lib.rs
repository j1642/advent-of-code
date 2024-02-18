use std::collections::VecDeque;

#[derive(Debug, PartialEq, Clone)]
struct Coord {
    row: usize,
    col: usize,
}

struct Args {
    count: u32,
    position: Coord,
    last_visited_idx: usize,
}

pub fn day23_1(text: &str) -> u32 {
    let mut matrix = build_matrix(text);
    let vertices = find_vertices(&matrix);
    let mut start = vertices[0].clone();
    let end = vertices[1].clone();

    // Prevent recursing out of the matrix
    matrix[start.row][start.col] = 'x';
    start.row += 1;

    // TODO: Can't prevent all back-tracking b/c the longest path may use some of the same squares
    // as a shorter path
    // New approach: make a graph, each intersection of <>v is a vertex,
    // and the path b/w each intersection is a edge with a weight (distance)
    let mut edge_weights: Vec<Vec<u32>> = vec![vec![0; vertices.len()]; vertices.len()];
    let mut q: VecDeque<Args> = VecDeque::new();
    q.push_back(Args {
        count: 1,
        position: start,
        last_visited_idx: 0,
    });

    while let Some(args) = q.pop_front() {
        let (mut count, position, mut last_visited_idx) =
            (args.count, args.position, args.last_visited_idx);

        if matrix[position.row][position.col] == '#' {
            continue;
        } else if matrix[position.row][position.col] == 'x' {
            continue;
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
            if end == position {
                println!("returning from end");
                continue;
            }
            count = 0;

            matrix[position.row][position.col] = '#';
        }

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
                q.push_back(Args {
                    count: count,
                    position: new_pos,
                    last_visited_idx: last_visited_idx,
                });
                matrix[position.row][position.col] = '#';
                continue;
            }
            'v' => {
                let new_pos = Coord {
                    row: position.row + 1,
                    col: position.col,
                };
                q.push_back(Args {
                    count: count,
                    position: new_pos,
                    last_visited_idx: last_visited_idx,
                });
                matrix[position.row][position.col] = '#';
                continue;
            }
            '<' => {
                let new_pos = Coord {
                    row: position.row,
                    col: position.col - 1,
                };
                q.push_back(Args {
                    count: count,
                    position: new_pos,
                    last_visited_idx: last_visited_idx,
                });
                matrix[position.row][position.col] = '#';
                continue;
            }
            _ => {}
        }

        // North, east, south, west
        let mut new_positions = vec![];
        new_positions.push(Coord {
            row: position.row - 1,
            col: position.col,
        });
        new_positions.push(Coord {
            row: position.row,
            col: position.col + 1,
        });
        new_positions.push(Coord {
            row: position.row + 1,
            col: position.col,
        });
        new_positions.push(Coord {
            row: position.row,
            col: position.col - 1,
        });
        for new_pos in new_positions {
            q.push_back(Args {
                count: count,
                position: new_pos,
                last_visited_idx: last_visited_idx,
            });
        }

        if matrix[position.row][position.col] == '.' {
            matrix[position.row][position.col] = 'x';
        }
    }

    println!("{:?}", edge_weights);
    return 0;
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

fn find_vertices(matrix: &Vec<Vec<char>>) -> Vec<Coord> {
    // Search the maze for intersections
    // Assume all intersections are surrounded orthogonally by "v<>" one-way paths or walls, '#'
    // Assume accessing a non-existent index will not happen
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
    return vertices;
}
