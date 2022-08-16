use std::fs;

use fdg_sim::json;
use petgraph::dot::{Config, Dot};

const SOCIAL_NETWORK: &'static str = include_str!("../../datasets/social_network.json");

fn main() {
    let graph = json::graph_from_json(SOCIAL_NETWORK).unwrap();

    fs::write(
        "social_network.dot",
        format!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel])).as_bytes(),
    )
    .unwrap();
}
