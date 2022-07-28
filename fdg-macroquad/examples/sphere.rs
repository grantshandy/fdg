use fdg_macroquad::JsonValue;
use fdg_sim::{
    petgraph::graph::NodeIndex, ForceGraph, ForceGraphHelper, Simulation, SimulationParameters,
};

#[macroquad::main("Force Graph Sphere Demo")]
async fn main() {
    pretty_env_logger::init();

    let mut graph: ForceGraph<JsonValue, JsonValue> = ForceGraph::default();
    let mut indices: Vec<NodeIndex> = Vec::new();

    let height = 10;
    let width = 50;

    for x in 0..width {
        for y in 0..height {
            indices.push(graph.add_force_node(format!("x: {x}, y: {y}"), JsonValue::default()));
        }
    }

    let top = graph.add_force_node("top", JsonValue::default());
    let bottom = graph.add_force_node("bottom", JsonValue::default());

    for y in 0..height {
        for x in 0..width {
            if x != 0 {
                graph.add_edge(
                    indices[(width * y) + x],
                    indices[((width * y) + x) - 1],
                    JsonValue::default(),
                );
            }

            if y != 0 {
                graph.add_edge(
                    indices[(width * y) + x],
                    indices[(width * (y - 1)) + x],
                    JsonValue::default(),
                );
            }
        }

        // cylinder
        graph.add_edge(
            indices[(width * y) + (width - 1)],
            indices[(width * y)],
            JsonValue::default(),
        );
    }

    for x in 0..width {
        graph.add_edge(
            indices[(width * (height - 1)) + x],
            top,
            JsonValue::default(),
        );
        graph.add_edge(indices[x], bottom, JsonValue::default());
    }

    fdg_macroquad::run_window(&mut Simulation::from_graph(
        &graph,
        SimulationParameters::default(),
    ))
    .await;
}
