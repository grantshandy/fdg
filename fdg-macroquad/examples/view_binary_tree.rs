use fdg_sim::{petgraph::graph::NodeIndex, ForceGraph, ForceGraphHelper};

#[macroquad::main("Force Graph Binary Tree Demo")]
async fn main() {
    let mut graph: ForceGraph<(), ()> = ForceGraph::default();
    let parent = graph.add_force_node(format!("{}", graph.node_count() + 1), ());

    tree(&mut graph, parent, 9);

    fdg_macroquad::run_window(&graph).await;
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

    let a = graph.add_force_node(format!("{}", graph.node_count() + 1), ());
    let b = graph.add_force_node(format!("{}", graph.node_count() + 1), ());

    graph.add_edge(parent, a, ());
    graph.add_edge(parent, b, ());

    tree(&mut graph, a, depth);
    tree(&mut graph, b, depth);
}
