use petgraph::graph::DiGraph;
use petgraph::visit::IntoNodeIdentifiers;

pub fn analyze_graph(graph: &DiGraph<i32, usize>) {
    let total_nodes = graph.node_count();
    let total_edges = graph.edge_count();
    println!("Total years: {}", total_nodes);
    println!("Total transitions: {}", total_edges);

    let total_additions: usize = graph.edge_references().map(|edge| *edge.weight()).sum();
    println!("Total additions: {}", total_additions);

    let avg_additions_per_year = total_additions as f64 / total_nodes as f64;
    println!("Average additions per year: {:.2}", avg_additions_per_year);
}

pub fn year_with_greatest_change(graph: &DiGraph<i32, usize>) -> (i32, i32, usize) {
    let mut max_change = 0;
    let mut year_of_change = 0;
    let mut prev_year = 0;

    for node in graph.node_identifiers() {
        let year = graph[node];
        let mut total_additions = 0;

        for edge in graph.edges(node) {
            total_additions += edge.weight();
        }

        if total_additions > max_change {
            max_change = total_additions;
            year_of_change = year;
            prev_year = year - 1;
        }
    }

    (prev_year, year_of_change, max_change)
}
pub fn print_additions_per_year(graph: &DiGraph<i32, usize>) {
    println!("Additions per year:");
    for node in graph.node_identifiers() {
        let year = graph[node];
        let mut total_additions = 0;
        for edge in graph.edges(node) {
            total_additions += edge.weight();
        }
        println!("Year {}: {}", year, total_additions);
    }
}
