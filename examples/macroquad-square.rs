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
    // let three = graph.add_force_node("Three", ());
    // let four = graph.add_force_node("Four", ());

    graph.add_edge(one, two, ());
    // graph.add_edge(two, three, ());
    // graph.add_edge(three, four, ());
    // graph.add_edge(four, one, ());

    let parameters = SimulationParameters {
        gravity: 30.0,
        node_start_range: -2.0..2.0,
        ..Default::default()
    };

    let mut sim = Simulation::from_graph(graph, Dimensions::Two, parameters);

    fdg::macroquad::run_window(&mut sim).await;
}
