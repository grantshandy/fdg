use fdg_sim::{CpuSimulation, Simulation, SimulationParameters, ForceGraph, ForceGraphHelper};

#[macroquad::main("Simple Demo")]
async fn main() {
    let mut graph: ForceGraph<()> = ForceGraph::default();

    graph.add_force_node("nOdE!", ());

    let params = SimulationParameters::default();
    let mut sim = CpuSimulation::from_graph(&graph, params);

    fdg_macroquad::run_window(&mut sim).await;
}