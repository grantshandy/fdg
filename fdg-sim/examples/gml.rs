use std::fs;

use fdg_sim::{gml, json};

const SOCIAL_NETWORK: &'static str = include_str!("../../datasets/social_network.json");

fn main() {
    let graph = json::graph_from_json(SOCIAL_NETWORK).unwrap();

    fs::write("social_network.gml", gml::graph_to_gml(&graph).as_bytes()).unwrap();
}
