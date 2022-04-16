use fdg::{
    sim::{ForceGraph, ForceGraphHelper},
    Dimensions, Simulation, SimulationParameters,
};

#[macroquad::main("Force Graph Square Demo")]
async fn main() {
    pretty_env_logger::init();

    let mut graph: ForceGraph<()> = ForceGraph::default();

    let one = graph.add_force_node("One", ());
    let two = graph.add_force_node("Two", ());
    let _three = graph.add_force_node("Three", ());

    graph.add_edge(one, two, ());

    let mut sim = Simulation::from_graph(graph, Dimensions::Two, SimulationParameters::default());

    fdg::macroquad::run_window(&mut sim).await;
}
