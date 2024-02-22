use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug)]
struct Node<'a> {
    idx: usize,
    connections: Vec<&'a str>,
}

pub fn day25_1(text: &str) -> usize {
    // Karger's algorithm
    let mut adj_matrix = build_adjacency_matrix(text);

    // Record which nodes are absorbed by `supernode_contents[i]`
    let mut supernode_contents = vec![vec![]; adj_matrix.len()];

    let mut collapsed_node_count = 0;
    while collapsed_node_count < adj_matrix.len() - 2 {
        let rand_idx = get_rand_int(adj_matrix.len() as u32);

        let mut edges = vec![];
        for i in 0..adj_matrix[rand_idx].len() {
            if adj_matrix[rand_idx][i] != 0 {
                edges.push(i);
            }
        }
        // Do not use a collapsed node
        if edges.len() == 0 {
            continue;
        }
        let rand_edges_idx = get_rand_int(edges.len() as u32);
        // Cant't remove() collapsed nodes b/c adj. matrix would not be square; would need to fix
        // Collapse node `rand_connected_idx` into node `rand_idx`
        let rand_connected_idx = edges[rand_edges_idx];
        for i in 0..adj_matrix[0].len() {
            if adj_matrix[rand_connected_idx][i] == 1
            && adj_matrix[rand_idx][i] == 0 {
                adj_matrix[rand_idx][i] = 1;
            }
            // Remove all edges from the collapsed node
            adj_matrix[rand_connected_idx][i] = 0;
            // Remove all edges to the collapsed node
            adj_matrix[i][rand_connected_idx] = 0;
            // Remove loops from and to the same node
            adj_matrix[rand_idx][rand_idx] = 0;
        }
        println!("collapsed {rand_connected_idx} into {rand_idx}");
        collapsed_node_count += 1;
        supernode_contents[rand_idx].push(rand_connected_idx);
    }

    // Centralize collapsed node contents into their parent's index
    for i in 0..supernode_contents.len() {
        for j in 0..supernode_contents[i].len() {
            let val = supernode_contents[i][j];
            while supernode_contents[val].len() > 0 {
                let popped = supernode_contents[val].pop().unwrap();
                supernode_contents[i].push(popped);
            }
        }
    }
    println!("x");
    for x in &supernode_contents {
        println!("{:?}", x);
    }

    let mut empty_idx = 0;
    let mut supernode_lengths = [0; 2];
    for contents in supernode_contents {
        if contents.len() > 0 {
            supernode_lengths[empty_idx] = contents.len() + 1;
            empty_idx += 1;
        }
    }

    return supernode_lengths.iter().product();
}

fn build_adjacency_matrix(text: &str) -> Vec<Vec<u8>> {
    let mut nodes: HashMap<&str, Node> = HashMap::new();
    let mut idx = 0;
    for line in text.lines() {
        let (name, connections) = line.split_once(": ").unwrap();
        let connections = connections.split(' ').collect::<Vec<&str>>();
        nodes.insert(
            name,
            Node {
                idx: idx,
                connections: connections,
            },
        );
        idx += 1;
    }

    // Check for any nodes matrixed as connections that do not
    // have their own line in the input
    let mut new_nodes = vec![];
    for node in nodes.values() {
        for conn in node.connections.iter() {
            if !nodes.contains_key(conn)
                && !new_nodes.iter().any(|x: &(&str, Node)| x.0 == *conn) {
                new_nodes.push(
                    (*conn,
                    Node {
                        idx: idx,
                        connections: vec![],
                    },
                ));
                idx += 1;
            }
        }
    }
    for (name, node) in new_nodes {
        nodes.insert(name, node);
    }

    let mut adj_matrix = vec![vec![0; nodes.len()]; nodes.len()];
    for node in nodes.values() {
        for conn in node.connections.iter() {
            if nodes.contains_key(conn) {
                // All edges are bi-directional
                adj_matrix[node.idx][nodes[conn].idx] = 1;
                adj_matrix[nodes[conn].idx][node.idx] = 1;
            } else {
                panic!();
            }
        }
    }

    return adj_matrix;
}

fn get_rand_int(excluded_max: u32) -> usize {
    let nanosec = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .subsec_nanos();

    return (nanosec % excluded_max) as usize;
}
