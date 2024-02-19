use std::collections::VecDeque;
use std::{thread, time};

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
    matrix[start.row][start.col] = 'O';
    start.row += 1;

    // TODO: Can't prevent all back-tracking b/c the longest path may use some of the same squares
    // as a shorter path
    // New approach: make a graph, each intersection of <>v is a vertex,
    // and the path b/w each intersection is a edge with a weight (distance)
    //
    // Use DFS over BFS so only one path of 'O' is in use at any time
    let mut edge_weights: Vec<Vec<u32>> = vec![vec![0; vertices.len()]; vertices.len()];
    let mut stack: Vec<Args> = Vec::new();
    stack.push(Args {
        count: 0,
        position: start,
        last_visited_idx: 0,
    });

    while let Some(args) = stack.pop() {
        let (mut count, position, mut last_visited_idx) =
            (args.count, args.position, args.last_visited_idx);

        if matrix[position.row][position.col] == 'O' {
            continue;
        }

        /*
        println!("{:?}, {}", position, matrix[position.row][position.col]);
        for row in &mut *matrix {
            println!("{:?}", row);
        }
        thread::sleep(time::Duration::from_millis(200));
        */

        count += 1;

        let mut is_position_vertex = false;
        if vertices.contains(&position) {
            is_position_vertex = true;
            println!("found vertex");
            for (i, vertex) in vertices.iter().enumerate() {
                if &position == vertex {
                    edge_weights[i][last_visited_idx] = count;
                    edge_weights[last_visited_idx][i] = count;
                    last_visited_idx = i;
                }
            }
            if end == position {
                println!("returning from end");
                continue;
            }

            // once a weight is added, remove all 'O' from the matrix
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
                    println!("added: {:?}, {}", new_pos, matrix[new_pos.row][new_pos.col]);
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
                    println!("added: {:?}, {}", new_pos, matrix[new_pos.row][new_pos.col]);
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
                    println!("added: {:?}, {}", new_pos, matrix[new_pos.row][new_pos.col]);
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
                println!("added: {:?}, {}", new_pos, matrix[new_pos.row][new_pos.col]);
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

    // TODO: add vertices to stack even if they are #
    for i in 0..vertices.len() {
        println!("v: {:?}, e: {:?}", vertices[i], edge_weights[i]);
    }
    return max_path_dijkstra(edge_weights);
}

fn max_path_dijkstra(edge_weights: Vec<Vec<u32>>) -> u32 {
    let mut dists = vec![u32::MAX; edge_weights.len()];
    let mut prev_nodes = vec![0; edge_weights.len()];

    let mut q: VecDeque<usize> = VecDeque::new();
    for vertex_idx in 0..edge_weights.len() {
        q.push_back(vertex_idx);
    }
    dists[0] = 0;

    while q.len() > 0 {
        // Find max dist vertex still in queue
        let mut min_dist = u32::MAX;
        let mut min_dist_idx: usize = 0;
        for vert_idx in &q {
            if dists[*vert_idx] < min_dist {
                min_dist = dists[*vert_idx as usize];
                min_dist_idx = *vert_idx;
            }
        }

        // Remove max dist vertex from queue
        let mut q_idx_to_remove = 0;
        for (i, vertex) in q.iter().enumerate() {
            if *vertex == min_dist_idx {
                q_idx_to_remove = i;
            }
        }
        q.remove(q_idx_to_remove);

        // Find removed vertex's neighbors that are in queue
        let mut neighbors = vec![];
        for (i, weight) in edge_weights[min_dist_idx].iter().enumerate() {
            if weight > &0 {
                neighbors.push(i);
            }
        }
        println!("neighbors: {:?}", neighbors);

        for neighbor_idx in neighbors {
            println!("{} + {}", dists[min_dist_idx], edge_weights[min_dist_idx][neighbor_idx]);
            let alt = dists[min_dist_idx] + edge_weights[min_dist_idx][neighbor_idx];
            if alt < dists[neighbor_idx] {
                dists[neighbor_idx] = alt;
                prev_nodes[neighbor_idx] = min_dist_idx;
            }
        }
    }
    println!("dists: {:?}", dists);

    return dists[1];
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
