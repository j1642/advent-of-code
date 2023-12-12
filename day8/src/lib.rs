use std::collections::HashMap;
use std::str::Lines;

pub fn day_8_1(text: &str) -> u64 {
    // Return number of steps required to reach node ZZZ from node AAA
    // The number should be a multiple of length of the first line of text
    let mut lines = text.lines();
    let directions = lines.next().unwrap();
    lines.next();
    let graph = build_graph(&mut lines);

    let mut current = "AAA";
    let mut count_steps = 0;
    loop {
        for direction in directions.chars() {
            if direction == 'R' {
                current = graph[current][1];
            } else if direction == 'L' {
                current = graph[current][0];
            } else {
                panic!("invalid direction: {direction}");
            }
            count_steps += 1;
        }
        if current == "ZZZ" {
            break;
        }
    }

    return count_steps;
}

pub fn day_8_2(text: &str) -> u64 {
    // Return number of steps required to reach node ZZZ from node AAA
    // The number should be a multiple of length of the first line of text
    let mut lines = text.lines();
    let directions = lines.next().unwrap();
    lines.next();

    let mut current_nodes: Vec<&str> = vec![];
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
        if node.chars().last().unwrap() == 'A' {
            current_nodes.push(node);
        }
    }

    let mut distances_to_end: Vec<u64> = vec![0; current_nodes.len()];

    for i in 0..current_nodes.len() {
        let mut count_steps = 0;
        let mut current = current_nodes[i];
        'middle: loop {
            for direction in directions.chars() {
                if direction == 'R' {
                    current = graph[current][1];
                } else if direction == 'L' {
                    current = graph[current][0];
                } else {
                    panic!("invalid direction '{direction}'");
                }
                count_steps += 1;
                if current.chars().last().unwrap() == 'Z' {
                    distances_to_end[i] = count_steps;
                    break 'middle;
                }
            }
        }
    }

    return least_common_multiple(distances_to_end);
}

fn least_common_multiple(mut nums: Vec<u64>) -> u64 {
    let mut numerator;
    let mut denom;

    while nums.len() > 1 {
        denom = greatest_common_divisor(nums[0], nums[1]);
        numerator = nums.pop().unwrap() * nums.pop().unwrap();
        let least_common_multiple = numerator / denom;
        nums.push(least_common_multiple);
    }

    return nums[0];
}

fn greatest_common_divisor(mut a: u64, mut b: u64) -> u64 {
    // Euclidean algorithm
    while b != 0 {
        (a, b) = (b, a % b);
    }
    return a;
}

fn build_graph<'a>(lines: &mut Lines<'a>) -> HashMap<&'a str, [&'a str; 2]> {
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

    return graph;
}
