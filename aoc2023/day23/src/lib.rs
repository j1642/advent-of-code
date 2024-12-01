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

    // Prevent escaping the matrix
    matrix[start.row][start.col] = 'O';
    start.row += 1;

    let edge_weights = find_edge_weights(matrix, vertices, start);
    return find_longest_dag_path(edge_weights);
}

pub fn day23_2(text: &str) -> u32 {
    let mut matrix = build_matrix(text);
    let vertices = find_vertices(&matrix);
    let mut start = vertices[0].clone();

    // Prevent escaping the matrix
    matrix[start.row][start.col] = 'O';
    start.row += 1;

    let mut edge_weights = find_edge_weights(matrix, vertices, start);
    // Graph is no longer directed
    for row in 0..edge_weights.len() {
        for col in 0..edge_weights[0].len() {
            if edge_weights[row][col] != 0 {
                edge_weights[col][row] = edge_weights[row][col]
            }
        }
    }
    // TODO: DFS to find all paths to the end (cannot visit a node twice)
    // arithmetic find all path lengths
    return find_longest_path(edge_weights);
}

fn find_edge_weights(
    mut matrix: Vec<Vec<char>>,
    vertices: Vec<Coord>,
    start: Coord,
) -> Vec<Vec<i32>> {
    let end = vertices[1].clone();
    let mut stack: Vec<Args> = Vec::new();
    stack.push(Args {
        count: 0,
        position: start,
        last_visited_idx: 0,
    });
    let mut edge_weights: Vec<Vec<i32>> = vec![vec![0; vertices.len()]; vertices.len()];

    while let Some(args) = stack.pop() {
        let (mut count, position, mut last_visited_idx) =
            (args.count, args.position, args.last_visited_idx);

        if matrix[position.row][position.col] == 'O' {
            continue;
        }

        count += 1;

        let mut is_position_vertex = false;
        if vertices.contains(&position) {
            is_position_vertex = true;
            for (i, vertex) in vertices.iter().enumerate() {
                if &position == vertex {
                    // Edges are one-way only
                    edge_weights[last_visited_idx][i] = count as i32;
                    last_visited_idx = i;
                }
            }
            if end == position {
                continue;
            }

            for row in 0..matrix.len() {
                for col in 0..matrix[0].len() {
                    if matrix[row][col] == 'O' {
                        matrix[row][col] = '.';
                    }
                }
            }

            count = 0;
            matrix[position.row][position.col] = '#';
        } else if matrix[position.row][position.col] == '#' {
            continue;
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
                if matrix[new_pos.row][new_pos.col] != 'O'
                    && (matrix[new_pos.row][new_pos.col] != '#' || vertices.contains(&new_pos))
                {
                    stack.push(Args {
                        count: count,
                        position: new_pos,
                        last_visited_idx: last_visited_idx,
                    });
                }
                continue;
            }
            'v' => {
                let new_pos = Coord {
                    row: position.row + 1,
                    col: position.col,
                };
                if matrix[new_pos.row][new_pos.col] != 'O'
                    && (matrix[new_pos.row][new_pos.col] != '#' || vertices.contains(&new_pos))
                {
                    stack.push(Args {
                        count: count,
                        position: new_pos,
                        last_visited_idx: last_visited_idx,
                    });
                }
                continue;
            }
            '<' => {
                let new_pos = Coord {
                    row: position.row,
                    col: position.col - 1,
                };
                if matrix[new_pos.row][new_pos.col] != 'O'
                    && (matrix[new_pos.row][new_pos.col] != '#' || vertices.contains(&new_pos))
                {
                    stack.push(Args {
                        count: count,
                        position: new_pos,
                        last_visited_idx: last_visited_idx,
                    });
                }
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

        // Avoid infinite loops by removing west and north movements
        if is_position_vertex {
            new_positions.pop();
            new_positions.remove(0);
        }
        if new_positions.len() == 4 && matrix[new_positions[0].row][new_positions[0].col] == 'v' {
            new_positions.remove(0);
        } else if new_positions.len() == 4
            && matrix[new_positions[3].row][new_positions[3].col] == '>'
        {
            new_positions.pop();
        }

        for new_pos in new_positions {
            if matrix[new_pos.row][new_pos.col] != 'O'
                && (matrix[new_pos.row][new_pos.col] != '#' || vertices.contains(&new_pos))
            {
                stack.push(Args {
                    count: count,
                    position: new_pos,
                    last_visited_idx: last_visited_idx,
                });
            }
        }

        if matrix[position.row][position.col] == '.' {
            matrix[position.row][position.col] = 'O';
        }
    }

    return edge_weights;
}

fn find_longest_path(edge_weights: Vec<Vec<i32>>) -> u32 {
    let mut path_lengths = vec![];
    let start_node = 0;
    let end_node = 1;

    let path = vec![];

    let mut stack = vec![];
    stack.push((start_node, path));

    while let Some(items) = stack.pop() {
        let (cur_node, mut path) = (items.0, items.1);
        if path.contains(&cur_node) {
            continue;
        }
        path.push(cur_node);
        if cur_node == end_node {
            let mut path_length = 0;
            for i in 0..path.len() - 1 {
                let node = path[i];
                let next_node = path[i + 1];
                path_length += edge_weights[node][next_node];
            }
            path_lengths.push(path_length);
            continue;
        }

        for i in 0..edge_weights[0].len() {
            if edge_weights[cur_node][i] != 0 {
                stack.push((i, path.clone()));
            }
        }
    }

    let mut max_path_length = 0;
    for path_length in path_lengths {
        if path_length > max_path_length {
            max_path_length = path_length;
        }
    }

    return max_path_length.try_into().unwrap();
}

fn find_longest_dag_path(mut edge_weights: Vec<Vec<i32>>) -> u32 {
    // Return longest path through a graph
    // Assume all edge weights are positive

    // Negate all edge weights so the longest path is the smallest,
    // most negative path length
    for i in 0..edge_weights.len() {
        for j in 0..edge_weights[0].len() {
            edge_weights[i][j] *= -1;
        }
    }

    let mut dists = vec![i32::MAX; edge_weights.len()];
    dists[0] = 0;
    let mut prev_nodes = vec![0; edge_weights.len()];

    let nodes = topological_sort(edge_weights.clone());

    for i in 0..nodes.len() {
        let node = nodes[i];

        let mut neighbors = vec![];
        for i in 0..edge_weights[node].len() {
            if edge_weights[node][i] != 0 {
                neighbors.push(i);
            }
        }

        for neighbor in neighbors {
            let alt = dists[node] + edge_weights[node][neighbor];
            if alt < dists[neighbor] {
                dists[neighbor] = alt;
                prev_nodes[neighbor] = node;
            }
        }
    }

    dists[1] *= -1;
    return dists[1].try_into().unwrap();
}

fn topological_sort(mut edge_weights: Vec<Vec<i32>>) -> Vec<usize> {
    // Kahns's algorithm, O(V + E)
    let mut sorted = vec![];

    // Find vertices without incoming edges. Must be at least 1 in a DAG
    let mut no_parents: VecDeque<usize> = VecDeque::new();
    for col in 0..edge_weights[0].len() {
        let mut no_incoming_edges = true;
        for row in 0..edge_weights.len() {
            if edge_weights[row][col] != 0 {
                no_incoming_edges = false;
                break;
            }
        }
        if no_incoming_edges {
            no_parents.push_back(col);
        }
    }

    while let Some(node) = no_parents.pop_front() {
        sorted.push(node);

        for i in 0..edge_weights[node].len() {
            if edge_weights[node][i] != 0 {
                let child = i;
                edge_weights[node][i] = 0;

                // Check if child has any other incoming edges
                let mut child_still_has_edge = false;
                for row in 0..edge_weights.len() {
                    if edge_weights[row][child] != 0 {
                        child_still_has_edge = true;
                        break;
                    }
                }
                if !child_still_has_edge {
                    no_parents.push_back(child);
                }
            }
        }
    }

    // Any remaining edges indicate a cycle is present
    for i in 0..edge_weights.len() {
        for j in 0..edge_weights[0].len() {
            if edge_weights[i][j] != 0 {
                panic!("graph has at least one cycle; cannot topologically sort");
            }
        }
    }

    return sorted;
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
