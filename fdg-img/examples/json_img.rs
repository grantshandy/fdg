use std::fs;

use fdg_sim::json;

fn main() {
    let graph = json::graph_from_json(include_str!("../../datasets/les_miserables.json")).unwrap();

    let svg = fdg_img::gen_image(&graph, None).unwrap();

    fs::write("json.svg", svg.as_bytes()).unwrap();
}
