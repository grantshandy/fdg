use std::fs;

use fdg_sim::{petgraph::graph::NodeIndex, ForceGraph, ForceGraphHelper};

fn main() {
    let mut graph: ForceGraph<(), ()> = ForceGraph::default();
    let mut indices: Vec<NodeIndex> = Vec::new();

    let height = 10;
    let width = 50;

    for x in 0..width {
        for y in 0..height {
            indices.push(graph.add_force_node(format!("x: {x}, y: {y}"), ()));
        }
    }

    let top = graph.add_force_node("top", ());
    let bottom = graph.add_force_node("bottom", ());

    for y in 0..height {
        for x in 0..width {
            if x != 0 {
                graph.add_edge(indices[(width * y) + x], indices[((width * y) + x) - 1], ());
            }

            if y != 0 {
                graph.add_edge(indices[(width * y) + x], indices[(width * (y - 1)) + x], ());
            }
        }

        // cylinder
        graph.add_edge(indices[(width * y) + (width - 1)], indices[(width * y)], ());
    }

    for x in 0..width {
        graph.add_edge(indices[(width * (height - 1)) + x], top, ());
        graph.add_edge(indices[x], bottom, ());
    }

    let svg = fdg_img::gen_image(&graph, None).unwrap();

    fs::write("sphere.svg", svg.as_bytes()).unwrap();
}
