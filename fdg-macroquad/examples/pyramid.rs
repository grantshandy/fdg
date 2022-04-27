use fdg_sim::{ForceGraph, ForceGraphHelper, Simulation, SimulationParameters, Dimensions};

#[macroquad::main("Force Graph Ring Demo")]
async fn main() {
    pretty_env_logger::init();

    let mut graph: ForceGraph<()> = ForceGraph::default();

    let one = graph.add_force_node("", ());
    let two = graph.add_force_node("", ());
    let three = graph.add_force_node("", ());
    let four = graph.add_force_node("", ());

    graph.add_edge(one, two, ());
    graph.add_edge(two, three, ());
    graph.add_edge(three, one, ());
    graph.add_edge(one, four, ());
    graph.add_edge(two, four, ());
    graph.add_edge(three, four, ());

    let params = SimulationParameters {
        dimensions: Dimensions::Three,
        ..Default::default()
    };
    let mut sim = Simulation::from_graph(graph, params);

    fdg_macroquad::run_window(&mut sim).await;
}
