use fdg_sim::{ForceGraph, ForceGraphHelper, Simulation, SimulationParameters};

#[macroquad::main("Force Graph Max Nodes Demo")]
async fn main() {
    pretty_env_logger::init();

    let mut graph: ForceGraph<()> = ForceGraph::default();

    for i in 0..700 {
        graph.add_force_node(format!("{i}"), ());
    }

    let mut sim = Simulation::from_graph(graph, SimulationParameters::default());

    fdg_macroquad::run_window(&mut sim).await;
}
