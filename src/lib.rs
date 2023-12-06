// Advent of Code 2023
pub fn day_1_1(text: &str) -> i32 {
    let mut total = 0;
    let mut first_digit = 'x';
    let mut last_digit = 'x';

    for line in text.lines() {
        for c in line.chars() {
            if c.is_ascii_digit() {
                if first_digit == 'x' {
                    first_digit = c;
                }
                last_digit = c;
                continue;
            }
        }
        let s_num = format!("{}{}", first_digit, last_digit);
        let i_num;
        match s_num.parse::<i32>() {
            Ok(n) => {
                i_num = n;
            }
            Err(e) => {
                println!("error: {s_num} can't convert to int, {e}");
                return 0;
            }
        }
        total += i_num;
        first_digit = 'x';
        last_digit = 'x';
    }
    return total;
}

pub fn day_1_2(text: &str) -> i32 {
    let mut total = 0;
    //let spelled_nums = ["zero", "one", "two", "three", "four", "five", "six", "seven",
                //"eight", "nine"];
    let mut letters = String::with_capacity(5);

    for line in text.lines() {
        letters.clear();
        let mut first_digit = 'x';
        let mut last_digit = 'x';
        let line = &line.replace("zero", "ze0o");
        let line = &line.replace("one", "o1e");
        let line = &line.replace("two", "t2o");
        let line = &line.replace("three", "th3ee");
        let line = &line.replace("four", "fo4r");
        let line = &line.replace("five", "fi5e");
        let line = &line.replace("six", "s6x");
        let line = &line.replace("seven", "se7en");
        let line = &line.replace("eight", "ei8ht");
        let line = &line.replace("nine", "ni9e");
        for c in line.chars() {
            if c.is_ascii_digit() {
                if first_digit == 'x' {
                    first_digit = c;
                }
                last_digit = c;
                letters.clear();
                continue;
            }
            letters.push(c);
            if letters.len() < 3 {
                continue;
            }
            // TODO: find last num, including overlaps
            /*
            let mut ind_where_found: [i32; 10] = [-1, -1, -1, -1, -1, -1, -1, -1, -1, -1];
            for (i, num) in spelled_nums.iter().enumerate() {
                if letters.contains(num) {
                    let byte_ind = letters.find(num).unwrap();
                    ind_where_found[i] = byte_ind as i32;
                    let converted_to_char = char::from_digit(i as u32, 10);
                    if let Some(converted_to_char) = converted_to_char {
                        if first_digit == 'x' {
                            first_digit = converted_to_char;
                            last_digit = converted_to_char;
                        }
                    } else {
                        println!("error: {letters} contains {num}");
                        continue;
                    }
                }
            }
            // answer > 51757, !=52017, !=54427
            let mut max: i32 = -1;
            let mut last: i32 = -1;
            // Determine which spelled number was found at the highest str index
            for (found_num, ind_where_found) in ind_where_found.iter().enumerate() {
                if ind_where_found > &max {
                    last = found_num as i32;
                    max = *ind_where_found;
                }
            }
            //println!("max={max}, last={last}, letters={letters}, line={line}");
            if max > -1 {
                last_digit = char::from_digit(last as u32, 10).unwrap();
                //letters.clear();
            }
            */
        }

        let s_num = format!("{}{}", first_digit, last_digit);
        if s_num == "xx" {
            continue;
        }
        let i_num;
        match s_num.parse::<i32>() {
            Ok(n) => {
                i_num = n;
            }
            Err(e) => {
                println!("error: can't convert {s_num} to int, {e}");
                return 0;
            }
        }
        total += i_num;
    }
    return total;
}
