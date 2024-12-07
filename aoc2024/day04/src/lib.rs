// Find all instances of "XMAS" in a word search
pub fn day04_1(text: &str) -> i32 {
    let matrix = str_to_matrix(text);
    let mut found_xmas_count = 0;

    for row in 0..matrix.len() {
        for col in 0..matrix[0].len() {
            if matrix[row][col] == 'X' {
                let count = count_xmas('M', 0, row, col, &matrix);
                found_xmas_count += count;
            }
        }
    }
    return found_xmas_count;
}

// row-1, col-1 | row-1, col | row-1, col+1
// row, col-1   | row, col   | row, col+1
// row+1, col-1 | row+1, col | row+1, col+1
// directions:
// 789
// 4X6
// 123
fn count_xmas(look_for: char, mut search_direction: i32, row: usize, col: usize, matrix: &Vec<Vec<char>>) -> i32 {
    assert!(matrix.len() > 1);
    assert!(matrix[0].len() > 1);

    let mut directions = vec![7, 8, 9, 4, 6, 1, 2, 3];
    if row == 0 {
        // remove 7, 8, 9
        directions.swap_remove(0);
        directions.swap_remove(1);
        directions.swap_remove(2);
    } else if row == matrix.len() - 1 {
        // keep first 5 items, remove [1, 2, 3]
        directions.truncate(5);
    }

    if col == 0 {
        directions.retain(|&x| x != 7);
        directions.retain(|&x| x != 4);
        directions.retain(|&x| x != 1);
    } else if col == matrix[0].len() - 1 {
        directions.retain(|&x| x != 9);
        directions.retain(|&x| x != 6);
        directions.retain(|&x| x != 3);
    }

    let dir_row_col: [(i32, &str, &str); 8] = [(7, "dec", "dec"),
        (8, "dec", "no"),
        (9, "dec", "inc"),
        (4, "no", "dec"),
        (6, "no", "inc"),
        (1, "inc", "dec"),
        (2, "inc", "no"),
        (3, "inc", "inc"),
    ];

    let mut found_count = 0;

    for dir in directions {
        let mut search_row = row;
        let mut search_col = col;
        for (d, r, c) in dir_row_col {
            if d == dir {
                match r {
                    "inc" =>{search_row += 1},
                    "dec" =>{search_row -= 1},
                    _ =>{},
                }
                match c {
                    "inc" =>{search_col += 1},
                    "dec" =>{search_col -= 1},
                    _ =>{},
                }
            }
        }

        if look_for == 'M' {
            // Can look in any direction from X to M
            search_direction = dir;
        }
        if matrix[search_row][search_col] == look_for && dir == search_direction {
            match look_for {
                //'X' => return look_for_xmas('M', search_row, search_col, matrix),
                'M' => found_count += count_xmas('A', dir, search_row, search_col, matrix),
                'A' => return count_xmas('S', dir, search_row, search_col, matrix),
                'S' => return 1,
                _ => return 0
            }
            
        }
    }

    return found_count;
}

fn str_to_matrix(text: &str) -> Vec<Vec<char>> {
    let mut matrix: Vec<Vec<char>> = vec![];
    for (i, line) in text.lines().enumerate() {
        matrix.push(vec![]);
        for c in line.chars() {
            matrix[i].push(c);
        }
    }

    return matrix;
}
