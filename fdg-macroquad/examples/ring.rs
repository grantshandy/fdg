use fdg_sim::{
    ForceGraph, ForceGraphHelper, Simulation, SimulationParameters, petgraph::graph::NodeIndex
};

#[macroquad::main("Force Graph Ring Demo")]
async fn main() {
    pretty_env_logger::init();

    let mut graph: ForceGraph<()> = ForceGraph::default();
    let mut edge_indices: Vec<NodeIndex> = Vec::new();

    let center = graph.add_force_node("Center", ());
    let nodes: usize = 10;

    for x in 0..nodes {
        let i = graph.add_force_node(x.to_string(), ());
        edge_indices.push(i);

        graph.add_edge(i, center, ());
    }

    // for x in 1..nodes {
    //     graph.add_edge(edge_indices[x], edge_indices[x - 1], ());
    // }

    graph.add_edge(0.into(), 1.into(), ());

    let mut sim = Simulation::from_graph(graph, SimulationParameters::default());

    fdg_macroquad::run_window(&mut sim).await;
}
