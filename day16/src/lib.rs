use day14::build_matrix;
use std::collections::{HashSet, VecDeque};

pub fn day_16_1(text: &str, dir_coords: (char, usize, usize)) -> usize {
    // Return the amount of squares where the beam passes.
    // dir_coords is (travel direction, start_row, start_col)
    let mut matrix: Vec<Vec<char>> = build_matrix(text);
    let mut hit_tiles: HashSet<(usize, usize)> = HashSet::new();
    let mut queue: VecDeque<(char, usize, usize)> = VecDeque::new();
    queue.push_back(dir_coords);

    while queue.len() > 0 {
        let dir_coords = queue.pop_front().unwrap();
        traverse_q(&mut matrix, &mut hit_tiles, &mut queue, dir_coords);
    }

    return hit_tiles.len();
}

pub fn day_16_2(text: &str) -> usize {
    // Return the maximum amount of hit tiles starting from
    // any edge and moving away from that edge
    let mut max_hit_tiles = 0;
    let mut tiles;
    let matrix: Vec<Vec<char>> = build_matrix(text);

    for col in 0..matrix[0].len() {
        // From the top edge
        tiles = day_16_1(text, ('s', 0, col));
        if tiles > max_hit_tiles {
            max_hit_tiles = tiles;
        }
        // From the bottom edge
        tiles = day_16_1(text, ('n', matrix.len() - 1, col));
        if tiles > max_hit_tiles {
            max_hit_tiles = tiles;
        }
    }

    for row in 0..matrix.len() {
        // From the west edge
        tiles = day_16_1(text, ('e', row, 0));
        if tiles > max_hit_tiles {
            max_hit_tiles = tiles;
        }
        // From the east edge
        tiles = day_16_1(text, ('w', row, matrix[0].len() - 1));
        if tiles > max_hit_tiles {
            max_hit_tiles = tiles;
        }
    }

    return max_hit_tiles;
}

fn traverse_q(
    matrix: &mut Vec<Vec<char>>,
    hit_tiles: &mut HashSet<(usize, usize)>,
    q: &mut VecDeque<(char, usize, usize)>,
    dir_coords: (char, usize, usize),
) {
    // Replace splitters with impassable squares after each is hit
    let (direction, cur_row, cur_col) = dir_coords;

    if direction == 'n' {
        let mut chr;
        for row in (0..=cur_row).rev() {
            chr = matrix[row][cur_col];
            hit_tiles.insert((row, cur_col));
            match chr {
                '-' => {
                    matrix[row][cur_col] = 'X';
                    if cur_col < matrix[0].len() - 1 {
                        q.push_back(('e', row, cur_col + 1));
                    }
                    if cur_col > 0 {
                        q.push_back(('w', row, cur_col - 1));
                    }
                    break;
                }
                '|' => {
                    continue;
                }
                '\\' => {
                    if cur_col > 0 {
                        q.push_back(('w', row, cur_col - 1));
                    }
                    break;
                }
                '/' => {
                    if cur_col < matrix[0].len() - 1 {
                        q.push_back(('e', row, cur_col + 1));
                    }
                    break;
                }
                'X' => {
                    break;
                }
                _ => {
                    continue;
                }
            }
        }
    } else if direction == 'e' {
        let mut chr;
        for col in cur_col..matrix[0].len() {
            hit_tiles.insert((cur_row, col));
            chr = matrix[cur_row][col];
            match chr {
                '-' => {
                    continue;
                }
                '|' => {
                    matrix[cur_row][col] = 'X';
                    if cur_row > 0 {
                        q.push_back(('n', cur_row - 1, col));
                    }
                    if cur_row < matrix.len() - 1 {
                        q.push_back(('s', cur_row + 1, col));
                    }
                    break;
                }
                '\\' => {
                    if cur_row < matrix.len() - 1 {
                        q.push_back(('s', cur_row + 1, col));
                    }
                    break;
                }
                '/' => {
                    if cur_row > 0 {
                        q.push_back(('n', cur_row - 1, col));
                    }
                    break;
                }
                'X' => {
                    break;
                }
                _ => {
                    continue;
                }
            }
        }
    } else if direction == 's' {
        let mut chr;
        for row in cur_row..matrix.len() {
            chr = matrix[row][cur_col];
            hit_tiles.insert((row, cur_col));
            match chr {
                '-' => {
                    matrix[row][cur_col] = 'X';
                    if cur_col < matrix[0].len() - 1 {
                        q.push_back(('e', row, cur_col + 1));
                    }
                    if cur_col > 0 {
                        q.push_back(('w', row, cur_col - 1));
                    }
                    break;
                }
                '|' => {
                    continue;
                }
                '\\' => {
                    if cur_col < matrix[0].len() - 1 {
                        q.push_back(('e', row, cur_col + 1));
                    }
                    break;
                }
                '/' => {
                    if cur_col > 0 {
                        q.push_back(('w', row, cur_col - 1));
                    }
                    break;
                }
                'X' => {
                    break;
                }
                _ => {
                    continue;
                }
            }
        }
    } else if direction == 'w' {
        let mut chr;
        for col in (0..=cur_col).rev() {
            chr = matrix[cur_row][col];
            hit_tiles.insert((cur_row, col));
            match chr {
                '-' => {
                    continue;
                }
                '|' => {
                    matrix[cur_row][col] = 'X';
                    if cur_row > 0 {
                        q.push_back(('n', cur_row - 1, col));
                    }
                    if cur_row < matrix.len() - 1 {
                        q.push_back(('s', cur_row + 1, col));
                    }
                    break;
                }
                '\\' => {
                    if cur_row > 0 {
                        q.push_back(('n', cur_row - 1, col));
                    }
                    break;
                }
                '/' => {
                    if cur_row < matrix.len() - 1 {
                        q.push_back(('s', cur_row + 1, col));
                    }
                    break;
                }
                'X' => {
                    break;
                }
                _ => {
                    continue;
                }
            }
        }
    } else {
        panic!("invalid direction: {direction}");
    }
}
