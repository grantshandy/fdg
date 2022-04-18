use fdg::{
    sim::{ForceGraph, ForceGraphHelper}, Dimensions, SimulationParameters, Simulation,
};

#[macroquad::main("Force Graph Square Demo")]
async fn main() {
    pretty_env_logger::init();

    let mut graph: ForceGraph<()> = ForceGraph::default();

    let q = graph.add_force_node("1", ());
    let w = graph.add_force_node("2", ());
    let e = graph.add_force_node("3", ());
    let r = graph.add_force_node("4", ());

    graph.add_edge(q, w, ());
    graph.add_edge(w, e, ());
    graph.add_edge(e, r, ());
    graph.add_edge(r, q, ());

    let mut sim = Simulation::from_graph(graph, Dimensions::Two, SimulationParameters::default());

    fdg::macroquad::run_window(&mut sim).await;
}