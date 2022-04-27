use fdg_sim::{ForceGraph, ForceGraphHelper, Simulation, SimulationParameters, Dimensions, petgraph::graph::NodeIndex};

#[macroquad::main("Force Graph Lattice Demo")]
async fn main() {
    pretty_env_logger::init();

    let mut graph: ForceGraph<()> = ForceGraph::default();
    let mut indices: Vec<NodeIndex> = Vec::new();

    let size = 25;

    for _ in 0..size {
        for _ in 0..size {
            indices.push(graph.add_force_node("",()));
        }
    }

    for y in 0..size {
        for x in 0..size {
            if x != 0 {
                graph.add_edge(indices[(size * y) + x], indices[((size * y) + x) - 1], ());
            }

            if y != 0 {
                graph.add_edge(indices[(size * y) + x], indices[(size * (y - 1)) + x], ());
            }
        }
    }

    let params = SimulationParameters {
        dimensions: Dimensions::Three,
        ..Default::default()
    };

    fdg_macroquad::run_window(&mut Simulation::from_graph(graph, params)).await;
}
