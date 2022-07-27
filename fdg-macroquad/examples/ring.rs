use fdg_macroquad::JsonValue;
use fdg_sim::{ForceGraph, ForceGraphHelper, Simulation, SimulationParameters};

#[macroquad::main("Force Graph Ring Demo")]
async fn main() {
    pretty_env_logger::init();

    let mut graph: ForceGraph<JsonValue, JsonValue> = ForceGraph::default();

    let nodes = 100;

    graph.add_force_node("", JsonValue::default());
    for x in 1..nodes {
        graph.add_force_node("", JsonValue::default());
        graph.add_edge(x.into(), (x - 1).into(), JsonValue::default());
    }
    graph.add_edge(0.into(), (nodes - 1).into(), JsonValue::default());

    // let center = graph.add_force_node("", ());
    // for x in 0..nodes {
    //     graph.add_edge(x.into(), center, ());
    // }

    let mut sim = Simulation::from_graph(&graph, SimulationParameters::default());

    fdg_macroquad::run_window(&mut sim).await;
}
