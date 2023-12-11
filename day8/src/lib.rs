use std::collections::HashMap;

pub fn day_8_1(text: &str) -> u32 {
    // Return number of steps required to reach node ZZZ from node AAA
    // The number should be a multiple of length of the first line of text
    let mut lines = text.lines();
    let directions = lines.next().unwrap();
    lines.next();

    let mut graph = HashMap::new();
    loop {
        let line = lines.next();
        if line == None {
            break;
        }

        let (node, edges) = line.unwrap().split_once(" = (").unwrap();
        let (edge1, mut edge2) = edges.split_once(", ").unwrap();
        edge2 = edge2.trim_end_matches(')');

        graph.insert(node, [edge1, edge2]);
    }

    let mut current = "AAA";
    let mut count_steps = 0;
    loop {
        for direction in directions.chars() {
            if direction == 'R' {
                current = graph[current][1];
            } else if direction == 'L' {
                current = graph[current][0];
            }
            count_steps += 1;
        }
        if current == "ZZZ" {
            break;
        }
    }

    return count_steps;
}
