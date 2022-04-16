use fdg::sim::{ForceGraph, ForceGraphHelper};

#[macroquad::main("Force Graph Demo")]
async fn main() {
    pretty_env_logger::init();

    let mut graph: ForceGraph<()> = ForceGraph::default();
    for i in 0..100 {
        graph.add_force_node("1", ());

    }

    fdg::macroquad::run_window(graph).await;
}
