pub fn pt1(text: &str) -> u64 {
    let mut points: Vec<(f64, f64)> = vec![];
    let mut x = 0.0;
    let mut y = 0.0;

    let mut perimeter = 0.0;
    //points.push((0.0, 0.0));

    for line in text.lines() {
        let line = line.split(' ');
        let mut direction = "A";
        let mut number = 0.0;
        for (i, item) in line.enumerate() {
            if i == 0 {
                direction = item;
            } else if i == 1 {
                number = item.parse::<f64>().unwrap();
                perimeter += number
            }
        }

        if direction == "U" {
            // Switchind plus/minus doesn't change answer
            y -= number;
        } else if direction == "D" {
            y += number;
        } else if direction == "R" {
            x += number;
        } else if direction == "L" {
            x -= number;
        } else {
            panic!();
        }
        points.push((x, y));
    }
    //points.pop();
    // Shoelace formula
    let mut interior_area: f64 = 0.0;
    for i in 0..points.len() - 1 {
        // Trapezoid formula
        //interior_area += (points[i].1 + points[i + 1].1) * (points[i].0 - points[i + 1].0);
        // Triangle formula
        interior_area += (points[i].0 * points[i+1].1) - (points[i+1].0 * points[i].1);
    }
    println!("{}", interior_area);
    println!("{}", interior_area / 2.0);
    println!("{}", perimeter);
    println!("{}", perimeter / 2.0);

    // Pick's theorem
    //return (interior_area.abs() / 2.0 + (perimeter + 1.0) / 2.0 + 1.0) as u64;
    return interior_area.abs() as u64 / 2 + (perimeter as u64 + 1) / 2 + 1;
}

fn change_height(v: &mut Vec<[usize; 2]>, low_height: usize, high_height: usize, cur_width: usize) {
    for h in low_height..=high_height {
        // First value found
        if v[h][0] == usize::MAX {
            v[h][0] = cur_width;
        // Second value found
        } else if v[h][1] == usize::MAX {
            if cur_width < v[h][0] {
                v[h][1] = v[h][0];
            }
            v[h][0] = cur_width;
        // New left value
        } else if cur_width < v[h][0] {
            if v[h][0] > v[h][1] {
                v[h][1] = v[h][0];
            }
            v[h][0] = cur_width;
        // New right value
        } else if cur_width > v[h][1] {
            v[h][1] = cur_width;
        // Ignore middle value
        } else if v[h][0] < cur_width && cur_width < v[h][1] {
            continue;
        // Ignore matching values
        } else if cur_width == v[h][0] || cur_width == v[h][1] {
            continue;
        } else {
            println!("row: {:?}", v[h]);
            println!("cur width: {cur_width}");
            panic!();
        }
        println!();
        println!("cur width: {cur_width}");
        println!("v[{}]: {:?}", h, v[h]);
    }
}

pub fn day_18_1_new(text: &str) -> usize {
    // Find the excavation volume given the digging instructions
    let (dimensions, start_coord) = find_excavation_dimensions(text);
    let (width, height) = dimensions;

    let mut v: Vec<[usize; 2]> = vec![[usize::MAX; 2]; height];
    let mut cur_width = start_coord.0;
    let mut cur_height = start_coord.1;
    v[cur_height][0] = cur_width;

    for line in text.lines() {
        let line = line.split(' ');
        let mut direction = "A";
        let mut number = 0;
        for (i, item) in line.enumerate() {
            if i == 0 {
                direction = item;
            } else if i == 1 {
                number = item.parse::<usize>().unwrap();
            }
        }

        println!("xxxxxxxxxxxxxxxxxxxxxxxx");
        if direction == "R" {
            cur_width += number;
        } else if direction == "L" {
            cur_width -= number;
        } else if direction == "U" {
            let new_height = cur_height - number;
            change_height(&mut v, new_height, cur_height, cur_width);
            cur_height = new_height;
        } else if direction == "D" {
            let new_height = cur_height + number;
            change_height(&mut v, cur_height, new_height, cur_width);
            cur_height = new_height;
        } else {
            panic!("invalid direction: {}", direction);
        }

        if direction == "D" || direction == "U" {
            continue;
        }
        // First value
        if v[cur_height][0] == usize::MAX {
            v[cur_height][0] = cur_width;
        // Second, unique value
        } else if v[cur_height][1] == usize::MAX {//&& v[cur_height][0] != cur_width {
            if v[cur_height][0] > cur_width {
                v[cur_height][1] = v[cur_height][0];
            }
            v[cur_height][0] = cur_width;
        // New left
        } else if cur_width < v[cur_height][0] {
            if v[cur_height][0] > v[cur_height][1] {
                v[cur_height][1] = v[cur_height][0];
            }
            v[cur_height][0] = cur_width;
        // New right
        } else if cur_width > v[cur_height][1] {
            v[cur_height][1] = cur_width;
        // Ignore middle value
        } else if v[cur_height][0] < cur_width && cur_width < v[cur_height][1] {
            continue;
        } else {
            println!("panic");
            println!("cur width: {cur_width}");
            println!("v[{}]: {:?}", cur_height, v[cur_height]);
            panic!();
        }
        println!();
        println!("cur width: {cur_width}");
        println!("v[{}]: {:?}", cur_height, v[cur_height]);
    }

    let mut volume = 0;
    for i in 0..v.len() {
        // + 1 account for index 0
        println!("i: {i}, v[{i}]: {:?}", v[i]);
        if v[i][1] > v[i][0] {
            volume += v[i][1] - v[i][0] + 1;
        } else {
            volume += v[i][0] - v[i][1] + 1;
        }
    }
    return volume;
}

pub fn day_18_2(text: &str) -> u64 {
    // Find the excavation volume using the hexadecimal dig instructions
    let (dimensions, start_coord) = find_excavation_dimensions_hex(text);
    let (_width, height) = dimensions;
    //println!("dimensions: {:?}", dimensions);
    //println!("start_coord: {:?}", start_coord);

    let matrix = build_vec_2(text, height, start_coord);

    let mut volume = 0;
    for i in 0..matrix.len() {
        if matrix[i][1] > matrix[i][0] {
            volume += matrix[i][1] - matrix[i][0];
        } else {
            volume += matrix[i][0] - matrix[i][1];
        }
    }
    return volume as u64;
}

fn build_vec_2(text: &str, height: usize, start: (usize, usize)) -> Vec<[usize; 2]> {
    // Find start and end cols in each row, add number of coordinates
    let mut v = vec![[0; 2]; height];

    let mut cur_width = start.0;
    let mut cur_height = start.1;
    v[cur_height][0] = cur_width;

    for line in text.lines() {
        let line = line.split(' ');

        let matches: &[_] = &['#', '(', ')'];
        let hex = line.last().unwrap().trim_matches(matches);
        let last_char = hex.chars().last().unwrap();
        let direction = match last_char {
            '0' => "R",
            '1' => "D",
            '2' => "L",
            '3' => "U",
            _ => panic!("invalid final digit: {}", last_char),
        };
        // Integer divide by 16 to remove last digit
        let hex = usize::from_str_radix(hex, 16).unwrap() / 16;

        if direction == "R" {
            cur_width += hex;
        } else if direction == "L" {
            cur_width -= hex;
        } else if direction == "U" {
            //cur_height -= hex;
            let new_height = cur_height - hex;
            for h in new_height..cur_height {
                if v[h][0] == 0 {
                    v[h][0] = cur_width;
                } else if cur_width < v[h][0] {
                    if v[h][0] > v[h][1] {
                        v[h][1] = v[h][0];
                    }
                    v[h][0] = cur_width;
                } else if cur_width > v[cur_width][1] {
                    v[h][1] = cur_width;
                } else if cur_width == v[h][0] || cur_width == v[h][1] {
                    continue;
                } else {
                    println!("row: {:?}", v[h]);
                    println!("cur width: {cur_width}");
                    panic!();
                }
            }
            cur_height = new_height;
        } else if direction == "D" {
            //cur_height += hex;
            let new_height = cur_height + hex;
            for h in cur_height..new_height {
                if v[h][0] == 0 {
                    v[h][0] = cur_width;
                } else if cur_width < v[h][0] {
                    if v[h][0] > v[h][1] {
                        v[h][1] = v[h][0];
                    }
                    v[h][0] = cur_width;
                } else if cur_width > v[cur_width][1] {
                    v[h][1] = cur_width;
                } else if cur_width == v[h][0] || cur_width == v[h][1] {
                    continue;
                } else {
                    println!("row: {:?}", v[h]);
                    println!("cur width: {cur_width}");
                    panic!();
                }
            }
            cur_height = new_height;
        } else {
            panic!("invalid direction: {}", direction);
        }

        if direction == "D" || direction == "U" {
            continue;
        }
        if v[cur_height][0] == 0 {
            v[cur_height][0] = cur_width;
        } else if cur_width < v[cur_height][0] {
            if v[cur_height][0] > v[cur_height][1] {
                v[cur_height][1] = v[cur_height][0];
            }
            v[cur_height][0] = cur_width;
        } else if cur_width > v[cur_width][1] {
            v[cur_height][1] = cur_width;
        } else {
            panic!();
        }
    }

    return v;
}

fn find_excavation_dimensions_hex(text: &str) -> ((usize, usize), (usize, usize)) {
    // Return (width, height) and (start width, start height)
    let mut max_width = 0;
    let mut min_width = 0;
    let mut max_height = 0;
    let mut min_height = 0;

    let mut cur_width = 0;
    let mut cur_height = 0;

    for line in text.lines() {
        let line = line.split(' ');

        let matches: &[_] = &['#', '(', ')'];
        let hex = line.last().unwrap().trim_matches(matches);
        let last_char = hex.chars().last().unwrap();
        let direction = match last_char {
            '0' => "R",
            '1' => "D",
            '2' => "L",
            '3' => "U",
            _ => panic!("invalid final digit: {}", last_char),
        };
        // Integer divide by 16 to remove last digit
        let hex = usize::from_str_radix(hex, 16).unwrap() / 16;

        if direction == "R" {
            cur_width += hex;
        } else if direction == "L" {
            cur_width -= hex;
        } else if direction == "U" {
            cur_height -= hex;
        } else if direction == "D" {
            cur_height += hex;
        } else {
            panic!("invalid direction: {}", direction);
        }

        if cur_width > max_width {
            max_width = cur_width;
        }
        if cur_width < min_width {
            min_width = cur_width;
        }
        if cur_height > max_height {
            max_height = cur_height;
        }
        if cur_height < min_height {
            min_height = cur_height;
        }
    }
    // +1 includes the location where width=0 and height=0
    return (
        (
            (max_width - min_width) as usize + 1,
            (max_height - min_height) as usize + 1,
        ),
        ((0 - min_width) as usize, (0 - min_height) as usize),
    );
}

fn flood_fill_edge_0s(matrix: &mut Vec<Vec<u8>>) {
    // Replace any 0 outside the excavation with a 2. The
    // excavation edge is marked with 1s
    for col in 0..matrix[0].len() {
        if matrix[0][col] == 0 {
            flood_fill_recurse(matrix, 0, col);
        }
        if matrix[matrix.len() - 1][col] == 0 {
            flood_fill_recurse(matrix, matrix.len() - 1, col);
        }
    }
    for row in 0..matrix.len() {
        if matrix[row][0] == 0 {
            flood_fill_recurse(matrix, row, 0);
        }
        if matrix[row][matrix[0].len() - 1] == 0 {
            flood_fill_recurse(matrix, row, matrix[0].len() - 1);
        }
    }
}

fn flood_fill_recurse(matrix: &mut Vec<Vec<u8>>, row: usize, col: usize) {
    // Orthogonal recursion replaces any 0s outisde the excavation with 2s
    if matrix[row][col] != 0 {
        return;
    }

    matrix[row][col] = 2;

    // North
    if row > 0 {
        flood_fill_recurse(matrix, row - 1, col);
    }
    // West
    if col > 0 {
        flood_fill_recurse(matrix, row, col - 1);
    }
    // South
    if row < matrix.len() - 1 {
        flood_fill_recurse(matrix, row + 1, col);
    }
    // East
    if col < matrix[0].len() - 1 {
        flood_fill_recurse(matrix, row, col + 1);
    }
}

/*
fn build_matrix(
    text: &str,
    width: usize,
    height: usize,
    start_coord: (usize, usize),
) -> Vec<Vec<u8>> {
    let mut matrix: Vec<Vec<u8>> = vec![vec![0; width]; height];
    let mut cur_width = start_coord.0;
    let mut cur_height = start_coord.1;

    for line in text.lines() {
        let line = line.split(' ');
        let mut direction = "A";
        for (i, item) in line.enumerate() {
            if i == 0 {
                direction = item;
            } else if i == 1 {
                if direction == "R" {
                    let new_width = cur_width + item.parse::<usize>().unwrap();
                    for width in cur_width..=new_width {
                        matrix[cur_height][width] = 1;
                    }
                    cur_width = new_width;
                } else if direction == "L" {
                    let mut new_width = cur_width - item.parse::<usize>().unwrap();
                    (cur_width, new_width) = (new_width, cur_width);
                    for width in cur_width..=new_width {
                        matrix[cur_height][width] = 1;
                    }
                } else if direction == "U" {
                    let mut new_height = cur_height - item.parse::<usize>().unwrap();
                    (cur_height, new_height) = (new_height, cur_height);
                    for height in cur_height..=new_height {
                        matrix[height][cur_width] = 1;
                    }
                } else if direction == "D" {
                    let new_height = cur_height + item.parse::<usize>().unwrap();
                    for height in cur_height..=new_height {
                        matrix[height][cur_width] = 1;
                    }
                    cur_height = new_height;
                } else {
                    panic!("invalid at i=1: {}", item);
                }
            }
        }
    }
    return matrix;
}
*/

fn find_excavation_dimensions(text: &str) -> ((usize, usize), (usize, usize)) {
    // Return (width, height) and (start width, start height)
    let mut max_width = 0;
    let mut min_width = 0;
    let mut max_height = 0;
    let mut min_height = 0;

    let mut cur_width = 0;
    let mut cur_height = 0;

    for line in text.lines() {
        let line = line.split(' ');
        let mut direction = "A";
        for (i, item) in line.enumerate() {
            if i == 0 {
                direction = item;
            } else if i == 1 {
                if direction == "R" {
                    cur_width += item.parse::<i32>().unwrap();
                } else if direction == "L" {
                    cur_width -= item.parse::<i32>().unwrap();
                } else if direction == "U" {
                    cur_height -= item.parse::<i32>().unwrap();
                } else if direction == "D" {
                    cur_height += item.parse::<i32>().unwrap();
                } else {
                    panic!("invalid at i=1: {}", item);
                }
            }
        }
        if cur_width > max_width {
            max_width = cur_width;
        }
        if cur_width < min_width {
            min_width = cur_width;
        }
        if cur_height > max_height {
            max_height = cur_height;
        }
        if cur_height < min_height {
            min_height = cur_height;
        }
    }
    // +1 includes the location where width=0 and height=0
    return (
        (
            (max_width - min_width) as usize + 1,
            (max_height - min_height) as usize + 1,
        ),
        ((0 - min_width) as usize, (0 - min_height) as usize),
    );
}
