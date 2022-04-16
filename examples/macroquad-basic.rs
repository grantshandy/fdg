use fdg::sim::{ForceGraph, ForceGraphHelper};

#[macroquad::main("Force Graph Demo")]
async fn main() {
    pretty_env_logger::init();

    let mut graph: ForceGraph<()> = ForceGraph::default();

    graph.add_force_node("", ());
    graph.add_force_node("", ());
    graph.add_force_node("", ());
    graph.add_force_node("", ());
    graph.add_force_node("", ());
    graph.add_force_node("", ());


    fdg::macroquad::run_window(graph).await;
}
