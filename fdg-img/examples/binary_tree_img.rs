use std::fs;

use fdg_img::Settings;
use fdg_sim::{petgraph::graph::NodeIndex, ForceGraph, ForceGraphHelper};

fn main() {
    let mut graph: ForceGraph<(), ()> = ForceGraph::default();
    let parent = graph.add_force_node("", ());

    tree(&mut graph, parent, 6);

    let svg = fdg_img::gen_image(
        &graph,
        Some(Settings {
            iterations: 10000,
            ..Default::default()
        }),
    )
    .unwrap();

    fs::write("binary_tree.svg", svg.as_bytes()).unwrap();
}

fn tree(graph: &mut ForceGraph<(), ()>, parent: NodeIndex, depth: u8) {
    let mut depth = depth;
    let mut graph = graph;

    if depth > 0 {
        depth -= 1;
    }

    if depth <= 0 {
        return;
    }

    let a = graph.add_force_node("", ());
    let b = graph.add_force_node("", ());

    graph.add_edge(parent, a, ());
    graph.add_edge(parent, b, ());

    tree(&mut graph, a, depth);
    tree(&mut graph, b, depth);
}
