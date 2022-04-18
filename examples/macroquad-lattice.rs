use fdg::{
    sim::{ForceGraph, ForceGraphHelper},
    Dimensions, Simulation, SimulationParameters,
};
use petgraph::graph::NodeIndex;

#[macroquad::main("Force Graph Square Demo")]
async fn main() {
    pretty_env_logger::init();

    let mut graph: ForceGraph<()> = ForceGraph::default();

    let mut indices: Vec<NodeIndex> = Vec::new();

    for x in 0..50 {
        let i = graph.add_force_node(x.to_string(), ());
        indices.push(i);

        if x > 0 {
            graph.add_edge(indices[x as usize], indices[(x - 1) as usize], ());
        }
    }

    let mut sim = Simulation::from_graph(graph, Dimensions::Two, SimulationParameters::default());

    fdg::macroquad::run_window(&mut sim).await;
}