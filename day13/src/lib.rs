pub fn day_13_1(text: &str) -> u32 {
    let mut sum = 0;
    let blocks = text.split("\n\n");

    for block in blocks {
        let mut vertical_mirrors: Vec<Vec<usize>> = vec![];
        let mut rows: Vec<Vec<char>> = vec![];

        for line in block.lines() {
            let line = line.chars().collect::<Vec<char>>();
            let mirrors = find_vertical_mirrors(&line);
            vertical_mirrors.push(mirrors);
            rows.push(line);
        }
        // Get vertical mirror shared by all rows, if one exists
        let mut shared_vert_mirror: Vec<usize> = vec![];

        if vertical_mirrors[0].len() > 0 {
            for i in 0..vertical_mirrors[0].len() {
                let mirror = vertical_mirrors[0][i];
                let mut is_shared = true;

                for j in 1..vertical_mirrors.len() {
                    if vertical_mirrors[j].contains(&mirror) {
                        continue;
                    } else {
                        is_shared = false;
                        break;
                    }
                }
                if is_shared {
                    shared_vert_mirror.push(mirror);
                    //break;
                }
            }
        }
        if shared_vert_mirror.len() > 0 {
            if shared_vert_mirror.len() > 1 {
                panic!("more than one vertical mirror");
            }
            // Average of palindrome start and end, plus one b/c mirror is between indices
            let add = shared_vert_mirror[0] as u32;
            sum += add;
            continue;
        }
        // If there is no vertical mirror, there is a horizontal mirror
        let mut horiz_mirrors: Vec<usize> = Vec::with_capacity(rows.len());
        for i in 0..rows.len() - 1 {
            let mut mirror_ind = 0;

            if rows[i] == rows[i + 1] {
                mirror_ind = i + 1;
                let mut hi = 0;
                let mut lo = 0;
                if i != 0 {
                    lo = i - 1;
                    hi = i + 2;
                }
                while hi < rows.len() {
                    if rows[lo] == rows[hi] {
                        if lo == 0 || hi == rows.len() - 1 {
                            break;
                        }
                        lo -= 1;
                        hi += 1;
                    } else {
                        mirror_ind = 0;
                        break;
                    }
                }
            }
            if mirror_ind > 0 {
                horiz_mirrors.push(mirror_ind);
            }
        }
        if horiz_mirrors.len() > 1 {
            panic!("more than one horizontal mirror");
        }
        if horiz_mirrors.len() > 0 {
            sum += 100 * horiz_mirrors[0] as u32;
        }
    }

    return sum;
}

fn find_vertical_mirrors(line: &Vec<char>) -> Vec<usize> {
    let mut palindromes = vec![];

    for left in 0..line.len() - 1 {
        for right in left + 1..line.len() {
            // Right is inclusive, as in [left, right].
            if left != 0 && right != line.len() - 1 {
                continue;
            } else if (right - left) % 2 == 0 {
                // Mirror index is always between indices, so odd-length sequences
                // like ".#." (left = 0, right = 2) are not valid.
                continue;
            }
            if is_palindrome(&line[left..=right]) {
                palindromes.push((left + right) / 2 + 1);
            }
        }
    }

    return palindromes;
}

fn is_palindrome(chrs: &[char]) -> bool {
    for i in 0..chrs.len() / 2 {
        if chrs[i] != chrs[chrs.len() - 1 - i] {
            return false;
        }
    }
    return true;
}
