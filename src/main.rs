//Does not include the Twitch Ego Nets dataset. 
//It can be downloaded here: https://snap.stanford.edu/data/twitch_ego_nets.html

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read};
use serde_json::from_str;
use serde::Deserialize;

mod tests;
mod files;
mod visuals;

pub struct UndirectedGraph {
    adjacency_list: Vec<Vec<usize>>,
}

// defining vertices as usizes
#[derive(Debug, Clone, Copy)]
struct Vertex(usize);

impl UndirectedGraph {
//creates adjacency list
    fn new(n:usize) -> Self {
        UndirectedGraph {
            adjacency_list: vec![vec![]; n],
        }
    }

    fn create_graph(n: usize, edges: &Vec<(Vertex, Vertex)>) -> UndirectedGraph {
        let mut graph = UndirectedGraph::new(n);
        for &(u, v) in edges {
            graph.adjacency_list[u.0].push(v.0);
            graph.adjacency_list[v.0].push(u.0);
        }
        graph
    }

    fn get_degree(&self, vertex: Vertex) -> usize {         //written with help of chat gpt
        let neighbors = &self.adjacency_list[*&vertex.0];
        neighbors.len()
    }

}

#[derive(Debug, Deserialize)]
struct NodeLabel {
    id: usize,
    target: usize,
}

pub fn calculate_average_vertex_degree(edges_file: &UndirectedGraph,node_labels: &HashMap<usize, usize>) -> (f32, f32){
    let mut total_degree_multiple_games = 0;
    let mut total_degree_single_game = 0;
    let mut count_multiple_games = 0;
    let mut count_single_game = 0;
    for (&node_id, &label) in node_labels.iter() {
        let degree = edges_file.get_degree(Vertex(node_id));
        if label == 0 {
        // if node plays multiple games, add its degree to total and count the node
            total_degree_multiple_games += degree;
            count_multiple_games += 1;
        } else {
            // if node only plays one game, add its degree to total and count the node
            total_degree_single_game += degree;
            count_single_game += 1;
        }
    }
        let avg_degree_multiple_games = total_degree_multiple_games as f32 / count_multiple_games as f32;
        let avg_degree_single_game = total_degree_single_game as f32 / count_single_game as f32;
        return (avg_degree_multiple_games, avg_degree_single_game);

}


fn main() {
    let edges_file = files::read_edges_file("/Users/nnnnn/twitch_egos/twitch_edges.json");
    let targets_file = files::read_labels_file("/Users/nnnnn/twitch_egos/twitch_target.csv");
    //println!("{:?}", edges_file);
    // println!("{:?}", targets_file);

    //calculates the degree distribution of all nodes in the graph, and sorts degrees before drawing into histogram (written with help of chat gpt)
    let mut degree_counts = HashMap::new();
    for (&node_id, &label) in targets_file.iter() {
        let degree = edges_file.get_degree(Vertex(node_id));
        let entry = degree_counts.entry(degree).or_insert(0);
        *entry += 1;
    }
    let mut sorted_degree_counts: Vec<(usize, usize)> = degree_counts.into_iter().collect();
    sorted_degree_counts.sort_by_key(|&(degree, _)| degree);

    //histogram visualisation of average degrees across ego-nets
    println!("\nHistogram of degrees:");
    visuals::draw_histogram(&sorted_degree_counts);

    //average degree
    let (avg_degree_multiple_games, avg_degree_single_game) = calculate_average_vertex_degree(&edges_file, &targets_file);

    println!("Average degree for multiple games: {}", avg_degree_multiple_games);
    println!("Average degree for single game: {}", avg_degree_single_game);
}