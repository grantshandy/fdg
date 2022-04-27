use fdg_sim::{ForceGraph, ForceGraphHelper, Simulation, SimulationParameters, Dimensions};

#[macroquad::main("Force Graph Shape Demo")]
async fn main() {
    pretty_env_logger::init();

    let mut graph: ForceGraph<()> = ForceGraph::default();

    let q = graph.add_force_node("", ());
    let w = graph.add_force_node("", ());
    let e = graph.add_force_node("", ());
    let r = graph.add_force_node("", ());
    let t = graph.add_force_node("", ());
    let y = graph.add_force_node("", ());
    let u = graph.add_force_node("", ());
    let i = graph.add_force_node("", ());
    let c = graph.add_force_node("", ());

    graph.add_edge(q, c, ());
    graph.add_edge(w, c, ());
    graph.add_edge(e, c, ());
    graph.add_edge(r, c, ());
    graph.add_edge(q, w, ());
    graph.add_edge(w, e, ());
    graph.add_edge(t, c, ());
    graph.add_edge(e, r, ());
    graph.add_edge(r, q, ());
    graph.add_edge(t, q, ());
    graph.add_edge(t, w, ());
    graph.add_edge(t, y, ());
    graph.add_edge(y, u, ());
    graph.add_edge(u, i, ());
    graph.add_edge(i, c, ());

    let params = SimulationParameters {
        dimensions: Dimensions::Three,
        ..Default::default()
    };

    fdg_macroquad::run_window(&mut Simulation::from_graph(graph, params)).await;
}
