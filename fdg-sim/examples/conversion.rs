use std::fs;

use fdg_sim::{gml, json};

const IMPORT: &'static str = include_str!("../../datasets/pyramid.json");

// converts a jsongraph to a graph in gml.

fn main() {
    let graph = json::graph_from_json(IMPORT).unwrap();

    let filename = "json2gml.gml";

    println!("writing jsongraph as gml at {filename}");

    fs::write(filename, gml::graph_to_gml(&graph)).unwrap();
}
