use fdg::sim::ForceGraph;

#[macroquad::main("Force Graph Demo")]
async fn main() {
    let mut graph: ForceGraph<&str> = ForceGraph::default();

    fdg::macroquad::run_window(graph).await;
}
