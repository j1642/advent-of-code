pub fn day_13_1(text: &str) -> u32 {
    // Return sum of vertical reflection indices plus sum of 100 * horizontal
    // mirror indices.
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
                }
            }
        }
        if shared_vert_mirror.len() > 0 {
            if shared_vert_mirror.len() > 1 {
                panic!("more than one vertical mirror");
            }
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
        sum += 100 * horiz_mirrors[0] as u32;
    }

    return sum;
}

pub fn day_13_2(text: &str) -> u32 {
    // Assuming there is one incorrect character in each input case, find the
    // new lines of reflection and return their sum.
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
        let mut shared_vert_mirror: Vec<usize> = vec![];

        // Centralize all vertical mirrors to avoid iterating over an incorrect
        // row's mirrors
        let mut all_mirrors: Vec<usize> = vec![];
        for i in 0..vertical_mirrors.len() {
            for j in 0..vertical_mirrors[i].len() {
                // TODO: HashSet for O(1) lookup and insertion (from docs)
                if !all_mirrors.contains(&vertical_mirrors[i][j]) {
                    all_mirrors.push(vertical_mirrors[i][j]);
                }
            }
        }
        let mut count_shared_mirrors: Vec<usize> = vec![0; all_mirrors.len()];

        // Make a histogram of mirrors
        if all_mirrors.len() > 0 {
            for i in 0..all_mirrors.len() {
                let mirror = all_mirrors[i];
                for j in 0..vertical_mirrors.len() {
                    if vertical_mirrors[j].contains(&mirror) {
                        count_shared_mirrors[i] += 1;
                    }
                }
            }
        }
        // If a mirror is shared by all rows except one, that is the mirror we want
        for i in 0..count_shared_mirrors.len() {
            if count_shared_mirrors[i] == rows.len() - 1 {
                shared_vert_mirror.push(all_mirrors[i]);
            }
        }
        if shared_vert_mirror.len() > 0 {
            let add = shared_vert_mirror[0] as u32;
            sum += add;
            continue;
        }

        // If there is no vertical mirror, there is a horizontal mirror
        let mut horiz_mirrors: Vec<usize> = Vec::with_capacity(rows.len());
        let mut is_diffs_used = false;

        for i in 0..rows.len() - 1 {
            let mut mirror_ind = 0;

            let diffs = count_differences(&rows[i], &rows[i + 1]);
            if diffs == 0 || (diffs == 1 && !is_diffs_used) {
                if diffs == 1 {
                    is_diffs_used = true;
                }
                mirror_ind = i + 1;
                let mut hi = 0;
                let mut lo = 0;
                if i != 0 {
                    lo = i - 1;
                    hi = i + 2;
                }
                while hi < rows.len() {
                    let diffs = count_differences(&rows[lo], &rows[hi]);
                    if rows[lo] == rows[hi] || (diffs == 1 && !is_diffs_used) {
                        if diffs == 1 {
                            is_diffs_used = true;
                        }
                        if lo == 0 || hi == rows.len() - 1 {
                            break;
                        }
                        lo -= 1;
                        hi += 1;
                    } else {
                        mirror_ind = 0;
                        // Missing the next line broke a few results
                        is_diffs_used = false;
                        break;
                    }
                }
            }
            if mirror_ind > 0 {
                if is_diffs_used {
                    horiz_mirrors.push(mirror_ind);
                    break;
                }
            }
        }
        if horiz_mirrors.len() > 1 {
            panic!("more than one horizontal mirror");
        }
        let prev_sum = sum;
        sum += 100 * horiz_mirrors[0] as u32;
        assert!(prev_sum != sum);
    }

    return sum;
}

fn count_differences(row1: &Vec<char>, row2: &Vec<char>) -> usize {
    // Return number of different chars in two equal-length vectors
    let mut count = 0;
    if row1.len() != row2.len() {
        panic!(
            "inequal lengths: row1: {}, row2: {}",
            row1.len(),
            row2.len()
        );
    }
    for i in 0..row1.len() {
        if row1[i] != row2[i] {
            count += 1;
        }
    }
    return count;
}

fn find_vertical_mirrors(line: &Vec<char>) -> Vec<usize> {
    // Find the set of mirrors within a single line of text. The returned values
    // mark the index to the left of the mirror. The mirror is between indices.
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
                // Center of the palindrome, plus one b/c mirror is between indices
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
