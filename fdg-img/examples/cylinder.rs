use std::fs;

use fdg_sim::{force, petgraph::graph::NodeIndex, ForceGraph, ForceGraphHelper};

fn main() {
    let mut graph: ForceGraph<(), ()> = ForceGraph::default();
    let mut indices: Vec<NodeIndex> = Vec::new();

    let size = 25;

    for x in 0..size {
        for y in 0..size {
            indices.push(graph.add_force_node(format!("x: {x}, y: {y}"), ()));
        }
    }

    for y in 0..size {
        for x in 0..size {
            if x != 0 {
                graph.add_edge(indices[(size * y) + x], indices[((size * y) + x) - 1], ());
            }

            if y != 0 {
                graph.add_edge(indices[(size * y) + x], indices[(size * (y - 1)) + x], ());
            }
        }

        // cylinder
        graph.add_edge(indices[(size * y) + (size - 1)], indices[(size * y)], ());
    }

    let svg = fdg_img::gen_image(&graph, &force::handy(45.0, 0.975, true, true), None).unwrap();

    fs::write("cylinder.svg", svg.as_bytes()).unwrap();
}
