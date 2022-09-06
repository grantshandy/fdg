use std::fs;

use fdg_sim::{ForceGraph, ForceGraphHelper};

fn main() {
    // initialize a graph
    let mut graph: ForceGraph<(), ()> = ForceGraph::default();

    // create a circle
    let nodes = 10;

    graph.add_force_node("0", ());
    for x in 1..nodes {
        graph.add_force_node(x.to_string(), ());
        graph.add_edge(x.into(), (x - 1).into(), ());
    }
    graph.add_edge(0.into(), (nodes - 1).into(), ());

    // generate svg text for your graph
    let svg = fdg_img::gen_image(&graph, None).unwrap();

    // save the svg on disk
    fs::write("ring.svg", svg.as_bytes()).unwrap();
}
