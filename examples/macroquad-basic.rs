use fdg::sim::{ForceGraph, ForceGraphHelper};

#[macroquad::main("Force Graph Demo")]
async fn main() {
    let mut graph: ForceGraph<()> = ForceGraph::default();

    graph.add_force_node("Skyline", ());
    graph.add_force_node("East", ());

    fdg::macroquad::run_window(graph).await;
}
