use fdg::{sim::{ForceGraph, ForceGraphHelper}, Simulation, Dimensions, SimulationParameters};

#[macroquad::main("Force Graph Demo")]
async fn main() {
    pretty_env_logger::init();

    let mut graph: ForceGraph<()> = ForceGraph::default();
    for _ in 0..100 {
        graph.add_force_node("1", ());

    }

    let parameters = SimulationParameters {
        force_charge: 300.0,
        node_start_range: -50.0..50.0,
    };

    let mut sim = Simulation::from_graph(graph, Dimensions::Two, parameters);

    fdg::macroquad::run_window(&mut sim).await;
}
