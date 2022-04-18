use fdg::{
    sim::{ForceGraph, ForceGraphHelper},
    Dimensions, Simulation, SimulationParameters,
    petgraph::graph::NodeIndex
};
use rand::Rng;

#[macroquad::main("Force Graph Square Demo")]
async fn main() {
    pretty_env_logger::init();

    let mut graph: ForceGraph<()> = ForceGraph::default();
    let mut node_indices: Vec<NodeIndex> = Vec::new();

    let num_nodes = 500;
    let num_edges = 900;

    for n in 0..num_nodes {
        node_indices.push(graph.add_force_node(n.to_string(), (),))
    }

    // let mut rng  = rand::thread_rng();

    // for _ in 0..num_edges {
    //     let a = node_indices[rng.gen_range(0..num_nodes)];
    //     let b = node_indices[rng.gen_range(0..num_nodes)];

    //     graph.add_edge(a, b, ());
    // }

    let mut sim = Simulation::from_graph(graph, Dimensions::Two, SimulationParameters::default());

    fdg::macroquad::run_window(&mut sim).await;
}