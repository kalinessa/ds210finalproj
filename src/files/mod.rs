use crate::{UndirectedGraph, Vertex, NodeLabel};
use serde_json::from_str;
use std::fs::File;
use std::io::{BufReader, Read};
use std::collections::HashMap;
use csv::Reader;

pub fn read_edges_file(filename: &str) -> UndirectedGraph {
    let file = File::open(filename).expect("Failed to open file");
    let mut buf_reader = BufReader::new(file);

    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).expect("Failed to read file");

    let egonet_adjacency_map: HashMap<String, Vec<Vec<usize>>> = from_str(&contents).expect("failed to parse data");
    let n = egonet_adjacency_map.len();
    let mut edges = Vec::new();
    for (node_id_str, adj_list) in egonet_adjacency_map.iter() {
        let node_id = node_id_str.parse::<usize>().expect("Failed to parse nodes' IDs");
        for edge in adj_list {
            let adj_node = edge[0];
            edges.push((Vertex(node_id), Vertex(adj_node)));
        }
    }
    UndirectedGraph::create_graph(n, &edges)
}


pub fn read_labels_file(filename: &str) -> HashMap<usize, usize> {
    let file = File::open(filename).expect("Failed to open file");
    let buf_reader = BufReader::new(file);
    let mut csv_reader = csv::Reader::from_reader(buf_reader);

    let mut node_labels = HashMap::new();
    for result in csv_reader.deserialize() {
        let record: NodeLabel = result.expect("Error parsing CSV record");
        node_labels.insert(record.id, record.target);
    }
    node_labels
}