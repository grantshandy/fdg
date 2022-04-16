use fdg::{
    sim::{ForceGraph, ForceGraphHelper},
    Dimensions, Simulation, SimulationParameters,
};

#[macroquad::main("Force Graph Max Nodes Demo")]
async fn main() {
    pretty_env_logger::init();

    let mut graph: ForceGraph<()> = ForceGraph::default();

    for i in 0..300 {
        graph.add_force_node(i.to_string(), ());
    }

    let parameters = SimulationParameters {
        gravity: 30.0,
        node_start_range: -2.0..2.0,
        ..Default::default()
    };

    let mut sim = Simulation::from_graph(graph, Dimensions::Two, parameters);

    fdg::macroquad::run_window(&mut sim).await;
}
