use fdg_sim::{petgraph::graph::NodeIndex, ForceGraph, ForceGraphHelper};

#[macroquad::main("Force Graph Lattice Demo")]
async fn main() {
    let mut graph: ForceGraph<(), ()> = ForceGraph::default();
    let mut indices: Vec<NodeIndex> = Vec::new();

    let size = 50;

    for x in 0..size {
        for y in 0..size {
            indices.push(graph.add_force_node(format!("{x},{y}"), ()));
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
    }

    fdg_macroquad::run_window(&graph).await;
}
