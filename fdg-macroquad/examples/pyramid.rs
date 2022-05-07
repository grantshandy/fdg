use fdg_sim::{CpuSimulation, ForceGraph, ForceGraphHelper, Simulation, SimulationParameters};

#[macroquad::main("Force Graph Pyramid Demo")]
async fn main() {
    pretty_env_logger::init();

    let mut graph: ForceGraph<()> = ForceGraph::default();

    let one = graph.add_force_node("one", ());
    let two = graph.add_force_node("two", ());
    let three = graph.add_force_node("three", ());
    let four = graph.add_force_node("four", ());
    let center = graph.add_force_node("center", ());

    graph.add_edge(one, two, ());
    graph.add_edge(two, three, ());
    graph.add_edge(three, four, ());
    graph.add_edge(four, one, ());
    graph.add_edge(center, one, ());
    graph.add_edge(center, two, ());
    graph.add_edge(center, three, ());
    graph.add_edge(center, four, ());

    let mut sim = CpuSimulation::from_graph(&graph, SimulationParameters::default());

    fdg_macroquad::run_window(&mut sim).await;
}
