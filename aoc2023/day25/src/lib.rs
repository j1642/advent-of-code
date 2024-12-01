use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug)]
struct Node<'a> {
    idx: usize,
    connections: Vec<&'a str>,
}

pub fn day25_1(text: &str) -> usize {
    // Karger's algorithm, more or less
    let mut adj_matrix = build_adjacency_matrix(text);

    // TODO: redo in progress
    let mut supernode_contents = vec![vec![]; adj_matrix.len()];
    let mut collapsed_node_count = 0;
    let mut edges = vec![];

    while collapsed_node_count < adj_matrix.len() - 2 {
        // Iterate over all edges 1/2n^2 style.
        edges.clear();
        for i in 0..adj_matrix.len() - 1 {
            for j in (i + 1)..adj_matrix[0].len() {
                // store edges in vec n times, where n is adj_matrix[i][j]
                for _ in 0..adj_matrix[i][j] {
                    edges.push((i, j));
                }
            }
        }
        // Randomly select one edge (i, j) from the vec (takes advantage of algorithm's probability)
        let rand_edge_idx = get_rand_int(edges.len() as u32);
        let edge_idxs = edges[rand_edge_idx];
        let expand_idx = edge_idxs.0;
        let collapse_idx = edge_idxs.1;
        // Cut and paste the collapsing node's outgoing edges to the expanding node
        for i in 0..adj_matrix[collapse_idx].len() {
            adj_matrix[expand_idx][i] += adj_matrix[collapse_idx][i];
            adj_matrix[collapse_idx][i] = 0;
        }
        // Redirect the collapsing node's incoming edges to the expanding_node
        for i in 0..adj_matrix.len() {
            if adj_matrix[i][collapse_idx] > 0 {
                adj_matrix[i][expand_idx] += adj_matrix[i][collapse_idx];
                adj_matrix[i][collapse_idx] = 0;
            }
        }
        // Remove edge from self, to self
        adj_matrix[expand_idx][expand_idx] = 0;
        supernode_contents[expand_idx].push(collapse_idx);

        println!("collapsed {} into {}", collapse_idx, expand_idx);
        collapsed_node_count += 1;
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
            if !nodes.contains_key(conn) && !new_nodes.iter().any(|x: &(&str, Node)| x.0 == *conn) {
                new_nodes.push((
                    *conn,
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
