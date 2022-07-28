use fdg_macroquad::JsonValue;
use fdg_sim::{
    petgraph::graph::NodeIndex, ForceGraph, ForceGraphHelper, Simulation, SimulationParameters,
};

#[macroquad::main("Force Graph Lattice Demo")]
async fn main() {
    pretty_env_logger::init();

    let mut graph: ForceGraph<JsonValue, JsonValue> = ForceGraph::default();
    let mut indices: Vec<NodeIndex> = Vec::new();

    let size = 50;

    for x in 0..size {
        for y in 0..size {
            indices.push(graph.add_force_node(format!("{x},{y}"), JsonValue::default()));
        }
    }

    for y in 0..size {
        for x in 0..size {
            if x != 0 {
                graph.add_edge(
                    indices[(size * y) + x],
                    indices[((size * y) + x) - 1],
                    JsonValue::default(),
                );
            }

            if y != 0 {
                graph.add_edge(
                    indices[(size * y) + x],
                    indices[(size * (y - 1)) + x],
                    JsonValue::default(),
                );
            }
        }
    }

    fdg_macroquad::run_window(&mut Simulation::from_graph(
        &graph,
        SimulationParameters::default(),
    ))
    .await;
}
