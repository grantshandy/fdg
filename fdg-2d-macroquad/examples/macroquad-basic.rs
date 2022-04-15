use fdg_sim::ForceGraph;
use macroquad::prelude::*;

#[macroquad::main("Force Graph Demo")]
async fn main() {
    let mut graph: ForceGraph<&str> = ForceGraph::default();

    fdg_2d_macroquad::run_window(graph).await;
}