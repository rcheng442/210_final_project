use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::bfs::bfs_shortest;
use crate::bfs::run_file;

#[derive(Debug)]

#[derive(PartialEq)]
pub enum BFSError {
    NodeNotFound,
}
// graph data structure using adjacency list representation
pub struct Graph(HashMap<String, Vec<String>>);

impl Graph {
    pub fn new() -> Self {
        Graph(HashMap::new())
    }

    pub fn add_edge(&mut self, from_node: String, to_node: String) {
        self.0.entry(from_node).or_insert_with(Vec::new).push(to_node);
    }

    pub fn get_map(&mut self) -> &mut HashMap<String, Vec<String>> {
        &mut self.0
    }

    pub fn print_adjacency_list(&self) {
        for (node, neighbors) in &self.0 {
            println!("{}: {:?}", node, neighbors);
        }
    }
}

// function to read a graph from a file
pub fn read_graph_from_file(filename: &str) -> Result<Graph, std::io::Error> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut graph: Graph = Graph::new();

    // parse each line as an edge
    for line in reader.lines() {
        let line = line?; // unwrap if not empty/error
        let nodes: Vec<&str> = line.split_whitespace().collect();

        // ensure each line has two nodes
        if nodes.len() != 2 {
            println!("Invalid line in the input file: {line}");
            continue; // skip when invalid, let user know
        }

        let from_node = nodes[0].to_string();
        let to_node = nodes[1].to_string();
        graph.add_edge(from_node, to_node);
    }
    Ok(graph)
}

// tests to verify the functionality of the Graph structure and BFS algorithm
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_graph_creation() {
        let graph = Graph::new();
        assert_eq!(graph.0.len(), 0);
    }

    #[test]
    fn test_add_edge() {
        let mut graph = Graph::new();
        // replicates the graph in "data/directed_connected.txt"
        graph.add_edge("0".to_string(), "4".to_string());
        graph.add_edge("1".to_string(), "2".to_string());
        graph.add_edge("1".to_string(), "8".to_string());
        graph.add_edge("2".to_string(), "3".to_string());
        graph.add_edge("3".to_string(), "1".to_string());
        graph.add_edge("3".to_string(), "8".to_string());
        graph.add_edge("4".to_string(), "2".to_string());
        graph.add_edge("5".to_string(), "4".to_string());
        graph.add_edge("6".to_string(), "5".to_string());
        graph.add_edge("7".to_string(), "5".to_string());
        graph.add_edge("7".to_string(), "6".to_string());
        graph.add_edge("8".to_string(), "7".to_string());

        assert_eq!(graph.0.len(), 9);
        assert_eq!(graph.0.get("0"), Some(&vec!["4".to_string()]));
        assert_eq!(graph.0.get("7"), Some(&vec!["5".to_string(), "6".to_string()]));
        assert_eq!(graph.0.get("2"), Some(&vec!["3".to_string()]));
    }

    #[test]
    fn test_bfs_shortest_path() {
        let mut graph = Graph::new();
        // Draw out this graph again for report
        graph.add_edge("0".to_string(), "4".to_string());
        graph.add_edge("1".to_string(), "2".to_string());
        graph.add_edge("1".to_string(), "8".to_string());
        graph.add_edge("2".to_string(), "3".to_string());
        graph.add_edge("3".to_string(), "1".to_string());
        graph.add_edge("3".to_string(), "8".to_string());
        graph.add_edge("4".to_string(), "2".to_string());
        graph.add_edge("5".to_string(), "4".to_string());
        graph.add_edge("6".to_string(), "5".to_string());
        graph.add_edge("7".to_string(), "5".to_string());
        graph.add_edge("7".to_string(), "6".to_string());
        graph.add_edge("8".to_string(), "7".to_string());

        assert_eq!(bfs_shortest(&graph.0, "2", "6"), Ok(4));
        assert_eq!(bfs_shortest(&graph.0, "2", "2"), Ok(0));
        assert_eq!(bfs_shortest(&graph.0, "0", "2"), Ok(2));
        assert_eq!(bfs_shortest(&graph.0, "1", "4"), Ok(4));
    }

    #[test]
    fn test_read_graph_from_file() {
        let result = read_graph_from_file("data/directed_connected.txt");
        assert!(result.is_ok());

        let graph = result.unwrap();
        assert_eq!(bfs_shortest(&graph.0, "2", "6"), Ok(4));
        assert_eq!(bfs_shortest(&graph.0, "2", "2"), Ok(0));
        assert_eq!(bfs_shortest(&graph.0, "0", "2"), Ok(2));
        assert_eq!(bfs_shortest(&graph.0, "1", "4"), Ok(3));
    }
}