mod graph;

mod bfs;
use crate::bfs::run_file;


fn main() {
    // running simulations on different types of graph data
    // each `run_file` processes a file containing graph data
    run_file("data/directed.txt", 8, 10000, false, true); // directed, displays adjacency list
    run_file("data/directed_connected.txt", 8, 10000, false, true); // directed, displays adjacency list
    run_file("data/undirected.txt", 8, 10000,false, true); // undirected, displays adjacency list

    // data taken from Stanford's website
    run_file("data/email-Eu-core.txt", 1005, 1000, false, false); // directed
    run_file("data/epinions.txt", 75879, 100, false, false); // directed
    run_file("data/slashdot0902.txt", 82168, 100, false, false); // undirected
}

// tests to ensure the correct functionality of the graph processing
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_file() {
        let filename = "data/directed.txt"; 
        let nodes = 9;
        let test_iterations = 500;
        let track_nodes = false;
        let print_adj_list = false;

        run_file(filename, nodes, test_iterations, track_nodes, print_adj_list);
    }

    #[test]
    fn check_run_file_values() {
        let filename = "data/directed_connected.txt"; 
        let nodes = 8;
        let test_iterations = 500;
        let track_nodes = false;
        let print_adj_list = false;

        let results = run_file(filename, nodes, test_iterations, track_nodes, print_adj_list);
        let (paths_found, connected, _avg_len, longest_path) = results;

        assert_eq!(longest_path, 6);
        assert_eq!(paths_found, 500);
        assert_eq!(connected, 100.);
    }
}