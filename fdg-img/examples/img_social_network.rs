use std::fs;

use fdg_sim::{force, json};

fn main() {
    let graph = json::graph_from_json(include_str!("../../datasets/social_network.json")).unwrap();

    let svg = fdg_img::gen_image(&graph, &force::handy(45.0, 0.975, true, true), None).unwrap();

    fs::write("social_network.svg", svg.as_bytes()).unwrap();
}
