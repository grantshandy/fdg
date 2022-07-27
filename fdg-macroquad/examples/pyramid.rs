use fdg_macroquad::JsonValue;
use fdg_sim::{ForceGraph, ForceGraphHelper, Simulation, SimulationParameters};

#[macroquad::main("Force Graph Pyramid Demo")]
async fn main() {
    pretty_env_logger::init();

    let mut graph: ForceGraph<JsonValue, JsonValue> = ForceGraph::default();

    let one = graph.add_force_node("one", JsonValue::default());
    let two = graph.add_force_node("two", JsonValue::default());
    let three = graph.add_force_node("three", JsonValue::default());
    let four = graph.add_force_node("four", JsonValue::default());
    let center = graph.add_force_node("center", JsonValue::default());

    graph.add_edge(one, two, JsonValue::default());
    graph.add_edge(two, three, JsonValue::default());
    graph.add_edge(three, four, JsonValue::default());
    graph.add_edge(four, one, JsonValue::default());
    graph.add_edge(center, one, JsonValue::default());
    graph.add_edge(center, two, JsonValue::default());
    graph.add_edge(center, three, JsonValue::default());
    graph.add_edge(center, four, JsonValue::default());

    let mut sim = Simulation::from_graph(&graph, SimulationParameters::default());

    fdg_macroquad::run_window(&mut sim).await;
}
