use std::fs;

use fdg_img::Settings;
use fdg_sim::{force, petgraph::graph::NodeIndex, ForceGraph, ForceGraphHelper};

fn main() {
    let mut graph: ForceGraph<(), f32> = ForceGraph::default();
    let parent = graph.add_force_node("", ());

    tree(&mut graph, parent, 6);

    let svg = fdg_img::gen_image(
        &graph,
        &force::weighted_fruchterman_reingold(45.0, 0.975),
        Some(Settings {
            iterations: 10000,
            ..Default::default()
        }),
    )
    .unwrap();

    fs::write("weighted_binary_tree.svg", svg.as_bytes()).unwrap();
}

fn tree(graph: &mut ForceGraph<(), f32>, parent: NodeIndex, depth: u8) {
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

    graph.add_edge(parent, a, 1.0);
    graph.add_edge(parent, b, 2.0);

    tree(&mut graph, a, depth);
    tree(&mut graph, b, depth);
}
