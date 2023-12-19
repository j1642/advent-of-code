use day14::build_matrix;

//#[derive(Copy)]
#[derive(Clone)]
//#[derive(Debug)]
struct Coord {
    row: usize,
    col: usize,
}

impl std::fmt::Debug for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({},{})", self.row, self.col)
    }
}

// TODO: Dijkstra's
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
    };
    matrix[1][1] = '.';

    return dijkstra(&matrix, start);
}

fn dijkstra(matrix: &Vec<Vec<char>>, start: Coord) -> u32 {
    // TODO: cannot move in same direction 3 times
    let mut dist: Vec<Vec<u32>> = Vec::with_capacity(matrix.len());
    let mut prev: Vec<Vec<Coord>> = Vec::with_capacity(matrix.len());
    let mut q: Vec<Coord> = Vec::with_capacity(matrix.len() * matrix[0].len());

    for row in 0..matrix.len() {
        dist.push(vec![std::u32::MAX; matrix[0].len()]);
        prev.push(vec![Coord{row:100, col:100}; matrix[0].len()]);
        for col in 0..matrix[0].len() {
            q.push(Coord{row: row, col: col});
        }
    }
    dist[start.row][start.col] = 0;

    let mut idx;
    while !q.is_empty() {
       idx = get_min_dist_vertex_idx(&q, &dist);
       let removed = q.swap_remove(idx);
       for neighbor in get_neighbor_idx_in_q(&matrix, &q, &removed) {
           let neighbor_row = q[neighbor].row;
           let neighbor_col = q[neighbor].col;

           let alt = dist[removed.row][removed.col] + matrix[neighbor_row][neighbor_col].to_digit(10).unwrap();
           // TODO: cannot travel in same direction 3 times
           if alt < dist[neighbor_row][neighbor_col] && !moved_this_dir_3_times(&prev, &removed, (neighbor_row, neighbor_col)) {
               dist[neighbor_row][neighbor_col] = alt;
               prev[neighbor_row][neighbor_col] = removed.clone();
           }
       }
    }
    for row in &prev {
        println!("{:?}", row);
    }
    // -2 instead of -1 because of the added, artificial perimeter
    return dist[matrix.len() - 2][matrix[0].len() - 2];
}

fn moved_this_dir_3_times(prev: &Vec<Vec<Coord>>, cur_vertex: &Coord, next_row_col: (usize, usize)) -> bool {
    // Consecutive moves through the matrix are limited to <= 3 times in the same direction
    // Could use a ring buffer to track previous directions

    let next_cur_row_diff = next_row_col.0 as isize - cur_vertex.row as isize;
    let next_cur_col_diff = next_row_col.1 as isize - cur_vertex.col as isize;

    let mut ancestor = &prev[cur_vertex.row][cur_vertex.col];
    let mut child = cur_vertex;

    for _ in 0..3 {
        if child.row as isize - ancestor.row as isize != next_cur_row_diff {
            return false;
        }
        if child.col as isize - ancestor.col as isize != next_cur_col_diff {
            return false;
        }
        child = &ancestor;
        ancestor = &prev[ancestor.row][ancestor.col];
    }
    return true;
}

fn get_neighbor_idx_in_q(matrix: &Vec<Vec<char>>, q: &Vec<Coord>, vertex: &Coord) -> Vec<usize> {
    let mut neighbors: Vec<usize> = vec![];
    for (i, v) in q.iter().enumerate() {
        if matrix[v.row][v.col] == '.' {
            continue;
        }

        if v.row == vertex.row - 1
            && v.col == vertex.col {
            // North
            neighbors.push(i);
        } else if v.row == vertex.row
            && v.col == vertex.col + 1 {
            // East
            neighbors.push(i);
        } else if v.row == vertex.row + 1
            && v.col == vertex.col {
            // South
            neighbors.push(i);
        } else if v.row == vertex.row
            && v.col == vertex.col - 1 {
            // West
            neighbors.push(i);
        }
    }
    return neighbors;
}

fn get_min_dist_vertex_idx(q: &Vec<Coord>, dist: &Vec<Vec<u32>>) -> usize {
    let mut min_dist = std::u32::MAX;
    let mut min_dist_idx = 0;
    for (i, vert) in q.iter().enumerate() {
        if dist[vert.row][vert.col] < min_dist {
            min_dist = dist[vert.row][vert.col];
            min_dist_idx = i;
        }
    }
    return min_dist_idx;
}
