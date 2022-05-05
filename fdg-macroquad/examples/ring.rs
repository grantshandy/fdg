use fdg_sim::{CpuSimulation, ForceGraph, ForceGraphHelper, Simulation, SimulationParameters};

#[macroquad::main("Force Graph Ring Demo")]
async fn main() {
    pretty_env_logger::init();

    let mut graph: ForceGraph<()> = ForceGraph::default();

    let nodes = 15;

    graph.add_force_node("", ());
    for x in 1..nodes {
        graph.add_force_node("", ());
        graph.add_edge(x.into(), (x - 1).into(), ());
    }
    graph.add_edge(0.into(), (nodes - 1).into(), ());

    // let center = graph.add_force_node("", ());
    // for x in 0..nodes {
    //     graph.add_edge(x.into(), center, ());
    // }

    let mut sim = CpuSimulation::from_graph(&graph, SimulationParameters::default());

    fdg_macroquad::run_window(&mut sim).await;
}
