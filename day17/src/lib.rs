use day14::build_matrix;

#[derive(Clone)]
enum Direction {
    N = 0,
    E,
    S,
    W,
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
    for i in 0..matrix.len() {
        matrix[i].insert(0, '.');
        matrix[i].push('.');
    }
    matrix.insert(0, vec!['.'; matrix[0].len()]);
    matrix.push(vec!['.'; matrix[0].len()]);

    let start = Coord {
        row: 1,
        col: 1,
        dir: Direction::E as usize, // entered from west, going east
    };
    matrix[1][1] = '0';

    return dijkstra(&matrix, start);
}

fn dijkstra(matrix: &Vec<Vec<char>>, start: Coord) -> u32 {
    // Cannot move in same direction 3 times
    // 3D matrix of row, col, and direction
    let mut dist: Vec<Vec<Vec<u32>>> = Vec::with_capacity(matrix.len());
    let mut prev: Vec<Vec<Vec<Coord>>> = Vec::with_capacity(matrix.len());
    let mut q: Vec<Coord> = Vec::with_capacity(matrix.len() * matrix[0].len());

    for row in 0..matrix.len() {
        dist.push(vec![vec![std::u32::MAX; "nesw".len()]; matrix[0].len()]);
        prev.push(vec![
            vec![
                Coord {
                    row: 100,
                    col: 100,
                    dir: Direction::N as usize
                };
                "nesw".len()
            ];
            matrix[0].len()
        ]);
        //vec![Coord{row:100, col:100}; matrix[0].len()]);
        if 0 < row && row < matrix.len() - 1 {
            for col in 1..matrix[0].len() - 1 {
                //println!("row: {row}, col: {col}");
                //println!("{:?}", matrix[row][col]);
                // direction of travel, the opposite of the direction entered from
                if row == start.row && col == start.col {
                    q.push(Coord {
                        row: start.row,
                        col: start.col,
                        dir: Direction::E as usize,
                    });
                    continue;
                }
                if row > 1 {
                    q.push(Coord {
                        row: row,
                        col: col,
                        dir: Direction::S as usize,
                    });
                }
                if col < matrix[0].len() - 2 {
                    q.push(Coord {
                        row: row,
                        col: col,
                        dir: Direction::W as usize,
                    });
                }
                if row < matrix.len() - 2 {
                    q.push(Coord {
                        row: row,
                        col: col,
                        dir: Direction::N as usize,
                    });
                }
                if col > 1 {
                    q.push(Coord {
                        row: row,
                        col: col,
                        dir: Direction::E as usize,
                    });
                }
            }
        }
    }
    //dist[start.row][start.col] = 0;
    for i in 0..dist[start.row][start.col].len() {
        dist[start.row][start.col][i] = 0;
    }
    println!("{:?}", dist[start.row][start.col]);

    // Not sure if this matters
    q.sort_by_key(|k| matrix[k.row][k.col].to_digit(10).unwrap());
    //println!("q: {:?}", q);

    let mut idx;
    let mut removed;
    let mut neighbor_row;
    let mut neighbor_col;
    let mut alt;

    while !q.is_empty() {
        idx = get_min_dist_vertex_idx(&q, &dist);
        if idx == std::usize::MAX {
            break;
        }
        //removed = q.swap_remove(idx);
        removed = q.remove(idx);
        for neighbor in get_neighbor_idx_in_q(&matrix, &q, &removed) {
            neighbor_row = q[neighbor].row;
            neighbor_col = q[neighbor].col;

            alt = dist[removed.row][removed.col][removed.dir]
                + matrix[neighbor_row][neighbor_col].to_digit(10).unwrap();

            let nbr_removed_row_diff = neighbor_row as isize - removed.row as isize;
            let nbr_removed_col_diff = neighbor_col as isize - removed.col as isize;
            //let mut direction;
            let dir = match (nbr_removed_row_diff, nbr_removed_col_diff) {
                (-1, 0) => Direction::N as usize,
                (0, 1) => Direction::E as usize,
                (1, 0) => Direction::S as usize,
                (0, -1) => Direction::W as usize,
                _ => panic!(
                    "invalid (row_diff, col_diff): ({}, {})",
                    nbr_removed_row_diff, nbr_removed_col_diff
                ),
            };

            if alt < dist[neighbor_row][neighbor_col][dir]
                && !moved_this_dir_3_times(&prev, &removed, (neighbor_row, neighbor_col))
            {
                dist[neighbor_row][neighbor_col][dir] = alt;
                prev[neighbor_row][neighbor_col][dir] = removed.clone();
            }
        }
    }
    /*
    for row in &dist {
        if row[1] == std::u32::MAX {
            continue;
        }
        println!("{:?}", row);
    }*/
    /*
    // Mark the path in prev
    let mut coord = &Coord{row:matrix.len() - 2, col:matrix[0].len() - 2};
    while coord.row != 1 || coord.col != 1 {
        dist[coord.row][coord.col] = 0;
        coord = &prev[coord.row][coord.col];
    }
    */
    /*for row in &prev {
        println!("{:?}", row);
    }*/
    /*
    for row in &dist {
        if row[1] == std::u32::MAX {
            continue;
        }
        println!("{:?}", row);
    }*/
    let mut min = std::u32::MAX;
    let mut min_idx = std::usize::MAX;
    // -2 instead of -1 because of the added, artificial perimeter
    for (i, d) in dist[matrix.len() - 2][matrix[0].len() - 2]
        .iter()
        .enumerate()
    {
        if d < &min {
            min = *d;
            min_idx = i;
        }
    }
    return dist[matrix.len() - 2][matrix[0].len() - 2][min_idx];
}

fn moved_this_dir_3_times(
    prev: &Vec<Vec<Vec<Coord>>>,
    cur_vertex: &Coord,
    next_row_col: (usize, usize),
) -> bool {
    // Consecutive moves through the matrix are limited to <= 3 times in the same direction
    // Could use a ring buffer to track previous directions

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

fn get_neighbor_idx_in_q(matrix: &Vec<Vec<char>>, q: &Vec<Coord>, vertex: &Coord) -> Vec<usize> {
    let mut neighbors: Vec<usize> = vec![];
    for (i, v) in q.iter().enumerate() {
        if matrix[v.row][v.col] == '.' {
            continue;
        }

        // TODO: consider adding direction limitations
        if v.row == vertex.row - 1 && v.col == vertex.col {
            // North
            neighbors.push(i);
        } else if v.row == vertex.row && v.col == vertex.col + 1 {
            // East
            neighbors.push(i);
        } else if v.row == vertex.row + 1 && v.col == vertex.col {
            // South
            neighbors.push(i);
        } else if v.row == vertex.row && v.col == vertex.col - 1 {
            // West
            neighbors.push(i);
        }
    }
    return neighbors;
}

fn get_min_dist_vertex_idx(q: &Vec<Coord>, dist: &Vec<Vec<Vec<u32>>>) -> usize {
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
