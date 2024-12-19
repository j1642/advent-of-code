use std::collections::HashMap;
pub fn day05_1(text: &str) -> i32 {
    // Edge case not covered: cyclic graph, 
    let (rules_text, updates_text) = text.split_once("\n\n").unwrap();

    let mut rules: HashMap<&str,Vec<&str>> = HashMap::new();
    for line in rules_text.lines() {
        let (l, r) = line.split_once('|').unwrap();
        //let left = l.parse::<i32>().unwrap();
        //let right = r.parse::<i32>().unwrap();

        if let Some(v) = rules.get_mut(l) {
            v.push(r);
        } else {
            rules.insert(l, vec![r]);
        }
    }

    let mut sum = 0;

    for line in updates_text.lines() {
        let mut is_correct_order = true;
        let pages = line.split(',').collect::<Vec<&str>>();
        for i in 0..pages.len() {
            // check if rule exists for pages[i]
            if let Some(rule) = rules.get(pages[i]) {
                //check that all previous nums obey pages[i] rules
                for j in 0..i {
                    if rule.iter().any(|x| x == &pages[j]) {
                        is_correct_order = false;
                        println!("false: {line}");
                        break
                    }
                }
            }
            if !is_correct_order {
                break;
            }
        }
        if is_correct_order {
            sum += pages[pages.len() / 2].parse::<i32>().unwrap();
            println!("true: {line}");
        }
        //println!("page: {:?}", pages);
    }

    /*
    for k in rules.keys() {
        println!("{k}: {:?}", rules.get(k).unwrap());
    }
    */
    return sum;
}
