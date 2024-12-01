use day14::build_matrix;

#[derive(Clone)]
enum Direction {
    NS = 0,
    EW,
}

//#[derive(Copy)]
#[derive(Clone)]
//#[derive(Debug)]
struct Coord {
    row: usize,
    col: usize,
    // direction of travel, the opposite of the direction entered from
    dir: usize,
}

impl std::fmt::Debug for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({},{})", self.row, self.col)
    }
}

pub fn day_17_1(text: &str) -> u32 {
    let mut matrix = build_matrix(text);
    // Add buffer edge to easily prevent "index out of range" error
    matrix.insert(0, vec!['.'; matrix[0].len()]);
    for i in 0..matrix.len() {
        matrix[i].insert(0, '.');
        matrix[i].push('.');
    }
    matrix.push(vec!['.'; matrix[0].len()]);

    let start = Coord {
        row: 1,
        col: 1,
        dir: Direction::NS as usize, // entered coord on the North/South axis
    };
    matrix[1][1] = '0';

    return dijkstra(&matrix, start);
}

fn dijkstra(matrix: &Vec<Vec<char>>, start: Coord) -> u32 {
    // Cannot move in same direction 3 times
    // dist and prev are 3D matrices of row, column, and axis of travel
    let mut dist: Vec<Vec<Vec<u32>>> = Vec::with_capacity(matrix.len());
    let mut prev: Vec<Vec<Vec<Coord>>> = Vec::with_capacity(matrix.len());
    let mut q: Vec<Coord> = Vec::with_capacity(matrix.len() * matrix[0].len());

    for row in 0..matrix.len() {
        dist.push(vec![vec![std::u32::MAX; "ns".len()]; matrix[0].len()]);
        prev.push(vec![
            vec![
                Coord {
                    row: 100,
                    col: 100,
                    dir: Direction::NS as usize
                };
                "ns".len()
            ];
            matrix[0].len()
        ]);

        if 0 < row && row < matrix.len() - 1 {
            for col in 1..matrix[0].len() - 1 {
                if row == start.row && col == start.col {
                    q.push(Coord {
                        row: start.row,
                        col: start.col,
                        dir: Direction::NS as usize,
                    });
                    continue;
                }
                q.push(Coord {
                    row: row,
                    col: col,
                    dir: Direction::NS as usize,
                });
                q.push(Coord {
                    row: row,
                    col: col,
                    dir: Direction::EW as usize,
                });
            }
        }
    }
    for i in 0..dist[start.row][start.col].len() {
        dist[start.row][start.col][i] = 0;
    }

    // Not sure if sorting matters
    q.sort_by_key(|k| matrix[k.row][k.col].to_digit(10).unwrap());

    let mut idx;
    let mut removed;

    let mut neighbor_row;
    let mut neighbor_col;
    let mut alt;
    let mut nbr_removed_row_diff: isize;
    let mut nbr_removed_col_diff: isize;
    let mut dir: usize;

    while !q.is_empty() {
        if q.len() % 1000 == 0 {
            println!("q len: {}", q.len());
        }
        idx = get_min_dist_vertex_idx(&q, &dist);
        if idx == std::usize::MAX {
            break;
        }
        //removed = q.swap_remove(idx);
        removed = q.remove(idx);
        for neighbor in get_neighbor_idx_in_q(&matrix, &prev, &q, &removed) {
            neighbor_row = q[neighbor].row;
            neighbor_col = q[neighbor].col;

            nbr_removed_row_diff = neighbor_row as isize - removed.row as isize;
            nbr_removed_col_diff = neighbor_col as isize - removed.col as isize;
            dir = match (nbr_removed_row_diff, nbr_removed_col_diff) {
                (-1, 0) => Direction::NS as usize,
                (0, 1) => Direction::EW as usize,
                (1, 0) => Direction::NS as usize,
                (0, -1) => Direction::EW as usize,
                _ => panic!(
                    "invalid (row_diff, col_diff): ({}, {})",
                    nbr_removed_row_diff, nbr_removed_col_diff
                ),
            };

            alt = dist[removed.row][removed.col][removed.dir]
                + matrix[neighbor_row][neighbor_col].to_digit(10).unwrap();

            if alt < dist[neighbor_row][neighbor_col][dir]
                && !moved_this_dir_3_times(&prev, &removed, (neighbor_row, neighbor_col))
            {
                dist[neighbor_row][neighbor_col][dir] = alt;
                prev[neighbor_row][neighbor_col][dir] = removed.clone();
            }
        }
    }

    // Find the third index of the return value
    let mut min = std::u32::MAX;
    let mut min_idx = std::usize::MAX;
    // -2 instead of -1 because of the added, artificial perimeter
    println!("{:?}", dist[matrix.len() - 2][matrix[0].len() - 2]);
    for (i, d) in dist[matrix.len() - 2][matrix[0].len() - 2]
        .iter()
        .enumerate()
    {
        if d < &min {
            min = *d;
            min_idx = i;
        }
    }
    // Mark the path in prev
    /*
    let mut path: Vec<Vec<u32>> = vec![vec![0; matrix[0].len() - 2]; matrix.len() - 2];
    let mut coord = &Coord {
        row: matrix.len() - 2,
        col: matrix[0].len() - 2,
        dir: min_idx,
    };
    while coord.row != 1 || coord.col != 1 {
        path[coord.row - 1][coord.col - 1] = matrix[coord.row][coord.col].to_digit(10).unwrap();
        coord = &prev[coord.row][coord.col][coord.dir];
    }
    for row in 0..path.len() {
        println!();
        for col in 0..path[0].len() {
            print!("{} ", path[row][col]);
        }
    }
    println!();
    println!("{:?}", dist[2][6]);
    */

    return dist[matrix.len() - 2][matrix[0].len() - 2][min_idx];
}

fn moved_this_dir_3_times(
    prev: &Vec<Vec<Vec<Coord>>>,
    cur_vertex: &Coord,
    next_row_col: (usize, usize),
) -> bool {
    // Consecutive moves are limited to <= 3 times in the same direction

    let next_cur_row_diff = next_row_col.0 as isize - cur_vertex.row as isize;
    let next_cur_col_diff = next_row_col.1 as isize - cur_vertex.col as isize;

    let mut ancestor = &prev[cur_vertex.row][cur_vertex.col][cur_vertex.dir];
    let mut child = cur_vertex;

    // TODO: use .dir instead of .row and .col
    for _ in 0..3 {
        if child.row as isize - ancestor.row as isize != next_cur_row_diff {
            return false;
        }
        if child.col as isize - ancestor.col as isize != next_cur_col_diff {
            return false;
        }
        child = &ancestor;
        ancestor = &prev[ancestor.row][ancestor.col][ancestor.dir];
    }
    return true;
}

fn get_neighbor_idx_in_q(
    matrix: &Vec<Vec<char>>,
    prev: &Vec<Vec<Vec<Coord>>>,
    q: &Vec<Coord>,
    vertex: &Coord,
) -> Vec<usize> {
    // Return the set of adjacent, queued vertices of a given vertex, excluding
    // the reverse direction
    let parent = &prev[vertex.row][vertex.col][vertex.dir];
    let mut reverse_dir = '.';

    if vertex.row == parent.row - 1 {
        // Vertex north of parent
        reverse_dir = 's';
    } else if vertex.row == parent.row + 1 {
        // Vertex south of parent
        reverse_dir = 'n';
    } else if vertex.col == parent.col + 1 {
        // Vertex east of parent
        reverse_dir = 'w';
    } else if vertex.col < parent.col - 1 {
        // Vertex west of parent
        reverse_dir = 'e';
    }

    let mut neighbors: Vec<usize> = vec![];
    for (i, v) in q.iter().enumerate() {
        if matrix[v.row][v.col] == '.' {
            continue;
        }

        if v.row == vertex.row - 1 && v.col == vertex.col && reverse_dir != 'n' {
            // North
            neighbors.push(i);
        } else if v.row == vertex.row && v.col == vertex.col + 1 && reverse_dir != 'e' {
            // East
            neighbors.push(i);
        } else if v.row == vertex.row + 1 && v.col == vertex.col && reverse_dir != 's' {
            // South
            neighbors.push(i);
        } else if v.row == vertex.row && v.col == vertex.col - 1 && reverse_dir != 'w' {
            // West
            neighbors.push(i);
        }
    }
    return neighbors;
}

fn get_min_dist_vertex_idx(q: &Vec<Coord>, dist: &Vec<Vec<Vec<u32>>>) -> usize {
    // Return the queue index of the vertex with the lowest accumulated cost
    let mut min_dist = std::u32::MAX;
    let mut min_dist_idx = std::usize::MAX;
    for (i, vert) in q.iter().enumerate() {
        if dist[vert.row][vert.col][vert.dir] < min_dist {
            min_dist = dist[vert.row][vert.col][vert.dir];
            min_dist_idx = i;
        }
    }
    return min_dist_idx;
}
