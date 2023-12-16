mod parser;
mod graph_construction;
mod analysis;
mod visualization;
mod date_added;
use crate::date_added::remove_extra_columns;

use std::error::Error;
use std::fs::File;
use std::io::Write;

fn main() -> Result<(), Box<dyn Error>> {
    let input_file = "netflix_data.csv";
    let cleaned_file = "cleaned_netflix_data.csv";

    remove_extra_columns(input_file, cleaned_file)?;

    let data = parser::parse_netflix_data(cleaned_file)?;

    let mut graph = graph_construction::YearGraph::new();
    graph.add_sorted_years(&data);

    graph.connect_all_years();

    analysis::analyze_graph(&graph.graph);
    analysis::print_additions_per_year(&graph.graph);

    let (prev_year, year_of_change, max_change) = analysis::year_with_greatest_change(&graph.graph);
    println!("Greatest change in additions was between {} and {}: {} additions", prev_year, year_of_change, max_change);

    let dot_string = visualization::visualize_graph(&graph.graph);

    // Write the DOT data to a file named "netflix_graph.dot"
    let mut file = File::create("netflix_graph.dot")?;
    file.write_all(dot_string.as_bytes())?;

    Ok(())
}
