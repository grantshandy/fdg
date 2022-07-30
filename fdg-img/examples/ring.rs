use std::fs;

use fdg_img::Settings;
use fdg_sim::{force, ForceGraph, ForceGraphHelper};

fn main() {
    let mut graph: ForceGraph<(), ()> = ForceGraph::default();

    let nodes = 10;

    graph.add_force_node("", ());
    for x in 1..nodes {
        graph.add_force_node("", ());
        graph.add_edge(x.into(), (x - 1).into(), ());
    }
    graph.add_edge(0.into(), (nodes - 1).into(), ());

    let center = graph.add_force_node("", ());
    for x in 0..nodes {
        graph.add_edge(x.into(), center, ());
    }

    let svg = fdg_img::gen_image(
        &graph,
        &force::fruchterman_reingold(45.0, 0.975),
        Some(Settings {
            iterations: 50000,
            ..Default::default()
        }),
    )
    .unwrap();

    fs::write("ring.svg", svg.as_bytes()).unwrap();
}
