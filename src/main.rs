use std::collections::{HashMap};
use std::fs::File;
use std::io::{BufReader, Read};

use serde::Deserialize;

struct Graph {
    adjacency_list: HashMap<usize, Vec<usize>>,
}

impl Graph {
//create graph as hashmap
    fn new() -> Self {
        Graph {
            adjacency_list: HashMap::new(),
        }
    }

}

#[derive(Debug, Deserialize)]
struct NodeLabel {
    id: usize,
    target: usize,
}

fn read_edges_file(filename: &str) -> String {
    let file = File::open(filename).expect("Failed to open file");
    let mut buf_reader = BufReader::new(file);

    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).expect("Failed to read file");
    return contents
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

fn main() {
    let edges_file = read_edges_file("/Users/nnnnn/ds210finalproj/twitch_egos/twitch_edges.json");
    let targets_file = read_labels_file("/Users/nnnnn/ds210finalproj/twitch_egos/twitch_target.csv");
    // println!("{:?}", edges_file);
    // println!("{:?}", targets_file);
}






