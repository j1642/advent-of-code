pub fn day_15_1(text: &str) -> u32 {
    let mut total_hash = 0;
    let strings = text.split(',');

    for s in strings {
        let mut hash = 0;
        for c in s.chars() {
            if c == '\n' {
                continue;
            }
            hash += c as u32;
            hash *= 17;
            hash = hash % 256;
        }
        total_hash += hash;
    }
    return total_hash;
}

fn find_hash(chrs: &Vec<char>) -> usize {
    let mut hash = 0;
    for c in chrs {
        if c == &'\n' {
            continue;
        }
        hash += *c as usize;
        hash *= 17;
        hash = hash % 256;
    }
    return hash;
}

pub fn day_15_2(text: &str) -> usize {
    let mut boxes: Vec<Vec<(String, usize)>> = vec![vec![]; 256];
    let strings = text.split(',').map(|s| s.trim());
    for s in strings {
        let mut chrs = s.chars().collect::<Vec<char>>();

        let last_char = chrs.pop().unwrap();
        if last_char.is_digit(10) {
            // remove '='
            chrs.pop();
        }
        let hash = find_hash(&chrs);

        if last_char == '-' {
            let label = String::from_iter(chrs);
            for i in 0..boxes[hash].len() {
                if boxes[hash][i].0 == label {
                    boxes[hash].remove(i);
                    break;
                }
            }
        } else if last_char.is_digit(10) {
            let label = String::from_iter(chrs);
            let mut is_an_update = false;
            for i in 0..boxes[hash].len() {
                if boxes[hash][i].0 == label {
                    boxes[hash][i].1 = last_char.to_digit(10).unwrap() as usize;
                    is_an_update = true;
                    break;
                }
            }
            if !is_an_update {
                boxes[hash].push((label, last_char.to_digit(10).unwrap() as usize));
            }
        }
    }
    let mut focal_power = 0;
    for i in 0..boxes.len() {
        for j in 0..boxes[i].len() {
            let addend = (i + 1) * (j + 1) * boxes[i][j].1;
            focal_power += addend;
        }
    }
    return focal_power;
}
