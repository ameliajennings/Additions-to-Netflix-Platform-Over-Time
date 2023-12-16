use petgraph::graph::{NodeIndex, DiGraph};
use std::collections::HashMap;

pub struct YearGraph {
    pub graph: DiGraph<i32, usize>,
    year_indices: HashMap<i32, NodeIndex<u32>>,
}

impl YearGraph {
    pub fn new() -> Self {
        Self {
            graph: DiGraph::new(),
            year_indices: HashMap::new(),
        }
    }

    pub fn add_year(&mut self, year: i32, count: usize) {
        let year_node = *self.year_indices.entry(year).or_insert_with(|| self.graph.add_node(year));
        if let Some(&last_year_node) = self.year_indices.get(&(year - 1)) {
            self.graph.update_edge(last_year_node, year_node, count);
        }
    }

    pub fn add_sorted_years(&mut self, year_counts: &HashMap<i32, usize>) {
        let mut years: Vec<_> = year_counts.keys().cloned().collect();
        years.sort_unstable();
        for year in years {
            if let Some(&count) = year_counts.get(&year) {
                self.add_year(year, count);
            }
        }
    }

    pub fn connect_all_years(&mut self) {
        let mut previous_year_node: Option<NodeIndex<u32>> = None;
        let mut years: Vec<_> = self.year_indices.keys().cloned().collect();
        years.sort_unstable();
        for year in years {
            let year_node = self.year_indices[&year];
            if let Some(prev_node) = previous_year_node {
                if !self.graph.contains_edge(prev_node, year_node) {
                    self.graph.add_edge(prev_node, year_node, 0);
                }
            }
            previous_year_node = Some(year_node);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use petgraph::visit::EdgeRef;

    #[test]
    fn test_graph_construction() {
        let mut year_graph = YearGraph::new();
        year_graph.add_year(2019, 100);
        year_graph.add_year(2020, 150);
        year_graph.connect_all_years();

        assert!(year_graph.year_indices.contains_key(&2019));
        assert!(year_graph.year_indices.contains_key(&2020));

        let node_2019 = year_graph.year_indices[&2019];
        let node_2020 = year_graph.year_indices[&2020];
        
        // are 2019 and 2020 connected?
        let edge_index = year_graph.graph.edges_connecting(node_2019, node_2020).next().map(|e| e.id());

        assert!(edge_index.is_some(), "Edge between 2019 and 2020 should exist");

        let edge_weight = year_graph.graph.edge_weight(edge_index.unwrap()).unwrap();
        assert_eq!(*edge_weight, 150, "Edge weight between 2019 and 2020 should be 150");
    }
}
