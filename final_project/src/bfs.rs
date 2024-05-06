use std::collections::{HashMap, HashSet, VecDeque};
use crate::graph::*;
use rand::Rng;

// BFS to find the shortest path from `start_node` to `target_node`
pub fn bfs_shortest(graph: &HashMap<String, Vec<String>>, start_node: &str, target_node: &str) -> Result<usize, BFSError> {
    // track visited nodes and their distances from the start node
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    let mut distance: HashMap<String, usize> = HashMap::new();

    // initialize BFS from the start node
    visited.insert(start_node.to_string());
    queue.push_back(start_node.to_string());
    distance.insert(start_node.to_string(), 0);

    // process each node in the queue
    while let Some(node) = queue.pop_front() {
        if node == target_node {
            return Ok(distance[&node]);
        }

        // explore each unvisited neighbor
        if let Some(neighbors) = graph.get(&node) {
            for neighbor in neighbors {
                if !visited.contains(neighbor) {
                    visited.insert(neighbor.clone());
                    queue.push_back(neighbor.clone());
                    distance.insert(neighbor.clone(), distance[&node] + 1);
                }
            }
        }
    }
    
    // return an error if the target node is not reachable
    Err(BFSError::NodeNotFound)
}


    // driver function to simulate and analyze graph data from a file
pub fn run_file(filename: &str, nodes: u32, test_iterations: u32, track_nodes: bool, print_adj_list: bool) -> (u32, f32, f32, u32) {
    
    println!("----------------------------------------------------");
    println!("FILENAME: {filename}");
    println!("----------------------------------------------------");

    // initialize graph and tracking variables
    let mut graph = read_graph_from_file(filename).expect("Error reading graph");
    let mut paths_total: u32 = 0;
    let mut paths_found: u32 = 0;
    let mut longest_path: u32 = 0;

    // optional detailed iteration output
    if track_nodes { println!("Iterations\n"); }
    for i in 1..=test_iterations {
        let mut rng = rand::thread_rng();

        // randomly select start and target nodes
        let start_node = rng.gen_range(0..=nodes).to_string();
        let target_node = rng.gen_range(0..=nodes).to_string();

        // attempt to find shortest path
        match bfs_shortest(&graph.get_map(), &start_node, &target_node) {
            Ok(distance) => {
                if track_nodes { println!("Shortest path distance from {} to {} is {}", start_node, target_node, distance); }

                paths_total += distance as u32;
                paths_found += 1;

                if distance as u32 > longest_path { longest_path = distance as u32; }
            }
            Err(_err) => {
                if track_nodes { println!("No path found from node {} to {} - iteration {}", start_node, target_node, i); }
            }
        }
    }
    if track_nodes {println!("----------------------------------------------------");}

    // calculate statistics
    let avg_len: f32 = paths_total as f32 / paths_found as f32;
    let connected: f32 = paths_found as f32 / test_iterations as f32 * 100.;

    println!("Total paths found: {paths_found}");
    println!("Percent node-pairs connected to one another: {connected}%");
    println!("Average path length: {avg_len}");
    println!("Longest path length: {longest_path}");
    println!("----------------------------------------------------");

    if print_adj_list { graph.print_adjacency_list(); }

    // return collected metrics
    (paths_found, connected, avg_len, longest_path)
}