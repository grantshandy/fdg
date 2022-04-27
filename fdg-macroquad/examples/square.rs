use fdg_sim::{ForceGraph, ForceGraphHelper, Simulation, SimulationParameters, SimulationForces};

#[macroquad::main("Force Graph Square Demo")]
async fn main() {
    pretty_env_logger::init();

    let mut graph: ForceGraph<()> = ForceGraph::default();

    let q = graph.add_force_node("1", ());
    let w = graph.add_force_node("2", ());
    let e = graph.add_force_node("3", ());
    let r = graph.add_force_node("4", ());
    let t = graph.add_force_node("5", ());
    let y = graph.add_force_node("6", ());
    let u = graph.add_force_node("7", ());
    let c = graph.add_force_node("8", ());

    graph.add_edge(q, c, ());
    graph.add_edge(w, c, ());
    graph.add_edge(e, c, ());
    graph.add_edge(r, c, ());
    graph.add_edge(q, w, ());
    graph.add_edge(w, e, ());
    graph.add_edge(e, r, ());
    graph.add_edge(r, q, ());
    graph.add_edge(t, q, ());
    graph.add_edge(t, w, ());
    graph.add_edge(t, y, ());
    graph.add_edge(y, u, ());

    let params = SimulationParameters {
        forces: SimulationForces::fruchterman_reingold(350.0),
        ..Default::default()
    };

    fdg_macroquad::run_window(&mut Simulation::from_graph(graph, params)).await;
}
