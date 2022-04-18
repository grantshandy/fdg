use fdg::{
    sim::{ForceGraph, ForceGraphHelper}, Simulation, Dimensions, SimulationParameters,
};

#[macroquad::main("Force Graph Max Nodes Demo")]
async fn main() {
    pretty_env_logger::init();

    let mut graph: ForceGraph<()> = ForceGraph::default();

    for i in 0..700 {
        graph.add_force_node(format!("{i}"), ());
    }

    let mut sim = Simulation::from_graph(graph, Dimensions::Two, SimulationParameters::default());

    fdg::macroquad::run_window(&mut sim).await;
}
