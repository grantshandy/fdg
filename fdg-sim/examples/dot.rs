use std::fs;

use fdg_sim::{dot, json};

const SOCIAL_NETWORK: &'static str = include_str!("../../datasets/social_network.json");

fn main() {
    let graph = json::graph_from_json(SOCIAL_NETWORK).unwrap();

    fs::write(
        "social_network.dot",
        dot::graph_to_dot(&graph).unwrap().as_bytes(),
    )
    .unwrap();
}
