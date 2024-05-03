//Does not include the Twitch Ego Nets dataset. 
//It can be downloaded here: https://snap.stanford.edu/data/twitch_ego_nets.html

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read};

use serde::Deserialize;

struct Graph {
    adjacency_list: Vec<Vec<usize>>,
}
// defining vertices as usizes
#[derive(Debug, Clone, Copy)]
struct Vertex(usize);

impl Graph {
//creates adjacency list
    fn new(n:usize) -> Self {
        Graph {
            adjacency_list: vec![vec![]; n],
        }
    }

    fn create_graph(n: usize, edges: &Vec<(Vertex, Vertex)>) -> Graph {
        let mut graph = Graph::new(n);
        for &(u, v) in edges {
            graph.adjacency_list[u.0].push(v.0);
            graph.adjacency_list[v.0].push(u.0);
        }
        graph
    }
    // Not sure
    // fn get_degree(&self, vertex: Vertex) -> usize {
    //     self.adjacency_list[*&vertex.0].len()
    // }

}

#[derive(Debug, Deserialize)]
struct NodeLabel {
    id: usize,
    target: usize,
}

fn read_edges_file(filename: &str) -> Graph {
    let file = File::open(filename).expect("Failed to open file");
    let mut buf_reader = BufReader::new(file);

    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).expect("Failed to read file");

    let parsed_data: HashMap<String, Vec<Vec<usize>>> =
    serde_json::from_str(&contents).expect("failed to parse data");
    let n = parsed_data.len();
    let mut edges = Vec::new();
    for (node_id_str, adj_list) in parsed_data.iter() {
        let node_id = node_id_str.parse::<usize>().expect("Failed to parse nodes' IDs");
        for edge in adj_list {
            let adj_node = edge[0];
            edges.push((Vertex(node_id), Vertex(adj_node)));
        }
    }
    return Graph::create_graph(n, &edges)

}


fn read_labels_file(filename: &str) -> HashMap<usize, usize> {
    let file = File::open(filename).expect("Failed to open file");
    let buf_reader = BufReader::new(file);
    let mut csv_reader = csv::Reader::from_reader(buf_reader);

    let mut node_labels = HashMap::new();
    for result in csv_reader.deserialize() {
        let record: NodeLabel = result.expect("Error parsing CSV record");
        node_labels.insert(record.id, record.target);
    }
    return node_labels
}

// not done
fn calculate_average_vertex_degree(edges_file: &Graph,node_labels: &HashMap<usize, usize>) -> (f32, f32){
    return (3.0,2.0)
}

fn main() {
    let edges_file = read_edges_file("/Users/nnnnn/twitch_egos/twitch_edges.json");
    let targets_file = read_labels_file("/Users/nnnnn/twitch_egos/twitch_target.csv");
    //println!("{:?}", edges_file);
    // println!("{:?}", targets_file);

    let (avg_degree_multiple_games, avg_degree_single_game) = calculate_average_vertex_degree(&edges_file, &targets_file);

    println!("Average degree for multiple games: {}", avg_degree_multiple_games);
    println!("Average degree for single game: {}", avg_degree_single_game);
}