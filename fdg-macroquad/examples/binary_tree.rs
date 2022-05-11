use fdg_sim::{
    petgraph::graph::NodeIndex, ForceGraph, ForceGraphHelper, Simulation, SimulationParameters,
};

#[macroquad::main("Force Graph Binary Tree Demo")]
async fn main() {
    pretty_env_logger::init();

    let mut graph: ForceGraph<()> = ForceGraph::default();
    let parent = graph.add_force_node("", ());

    tree(&mut graph, parent, 9);

    fdg_macroquad::run_window(&mut Simulation::from_graph(
        &graph,
        SimulationParameters::default(),
    ))
    .await;
}

fn tree(graph: &mut ForceGraph<()>, parent: NodeIndex, depth: u8) {
    let mut depth = depth;
    let mut graph = graph;

    if depth > 0 {
        depth -= 1;
    }

    if depth <= 0 {
        return;
    }

    let a = graph.add_force_node("", ());
    let b = graph.add_force_node("", ());

    graph.add_edge(parent, a, ());
    graph.add_edge(parent, b, ());

    tree(&mut graph, a, depth);
    tree(&mut graph, b, depth);
}
