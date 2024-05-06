mod tests {

    use crate::{UndirectedGraph, calculate_average_vertex_degree};
    use std::collections::HashMap;

    #[test]
    fn test_calculate_average_vertex_degree() {
        
        // sample test nodes
        let edges_file = UndirectedGraph {
            adjacency_list: vec![vec![1, 2], vec![0, 2], vec![0, 2], vec![2, 4], vec![3, 0],],};


        let mut targets_file = HashMap::new();
        targets_file.insert(0, 0);
        targets_file.insert(1, 0);
        targets_file.insert(2, 1);
        targets_file.insert(3, 1);
        targets_file.insert(4, 0);


        let (avg_degree_multiple_games, avg_degree_single_game) = calculate_average_vertex_degree(&edges_file, &targets_file);

        // both categories should both equal 2
        assert_eq!(avg_degree_multiple_games, 2.0);
        assert_eq!(avg_degree_single_game, 2.0);
    }
}