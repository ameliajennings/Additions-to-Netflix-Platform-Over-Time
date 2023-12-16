use petgraph::graph::DiGraph;
use petgraph::visit::EdgeRef;

pub fn visualize_graph(graph: &DiGraph<i32, usize>) -> String {
    let mut dot_string = "digraph {\n".to_owned();

    dot_string.push_str("  node [shape=circle, style=filled, fillcolor=gray95];\n");
    dot_string.push_str("  edge [color=gray50];\n");
    dot_string.push_str("  rankdir=LR;\n");

    for node in graph.node_indices() {
        let additions = graph.edges(node).map(|edge| *edge.weight()).sum::<usize>();
        let color = if additions > 1000 { "red" } else { "black" };
        let shape = if additions == 0 { "diamond" } else { "circle" };
        let label = format!("  {} [label=\"{}\", color={}, shape={}];\n", node.index(), graph[node], color, shape);
        dot_string.push_str(&label);
    }

    for edge in graph.edge_references() {
        let (source, target) = (edge.source().index(), edge.target().index());
        let weight = *edge.weight();
        let label = format!("  {} -> {} [label=\"{}\"];\n", source, target, weight);
        dot_string.push_str(&label);
    }

    dot_string.push_str("}\n");
    dot_string
}
