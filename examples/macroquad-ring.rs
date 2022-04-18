use fdg::{
    sim::{ForceGraph, ForceGraphHelper},
    Dimensions, Simulation, SimulationParameters,
};
use petgraph::graph::NodeIndex;

#[macroquad::main("Force Graph Square Demo")]
async fn main() {
    pretty_env_logger::init();

    let mut graph: ForceGraph<()> = ForceGraph::default();
    let mut edge_indices: Vec<NodeIndex> = Vec::new();

    let center = graph.add_force_node("Center", ());
    let edges: u8 = 10;

    for x in 0..edges {
        let i = graph.add_force_node(x.to_string(), ());
        edge_indices.push(i);

        graph.add_edge(i, center, ());
    }

    let mut sim = Simulation::from_graph(graph, Dimensions::Two, SimulationParameters::default());

    fdg::macroquad::run_window(&mut sim).await;
}
