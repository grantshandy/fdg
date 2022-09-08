use std::fs;

use fdg_sim::{ForceGraph, ForceGraphHelper};

fn main() {
    let mut graph: ForceGraph<(), ()> = ForceGraph::default();

    let one = graph.add_force_node("one", ());
    let two = graph.add_force_node("two", ());
    let three = graph.add_force_node("three", ());
    let four = graph.add_force_node("four", ());
    let center = graph.add_force_node("center", ());

    graph.add_edge(one, two, ());
    graph.add_edge(two, three, ());
    graph.add_edge(three, four, ());
    graph.add_edge(four, one, ());
    graph.add_edge(center, one, ());
    graph.add_edge(center, two, ());
    graph.add_edge(center, three, ());
    graph.add_edge(center, four, ());

    let svg = fdg_img::gen_image(graph, None).unwrap();

    fs::write("pyramid.svg", svg.as_bytes()).unwrap();
}
