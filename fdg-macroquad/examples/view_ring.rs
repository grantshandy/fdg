use fdg_sim::{ForceGraph, ForceGraphHelper};

#[macroquad::main("Force Graph Ring Demo")]
async fn main() {
    let mut graph: ForceGraph<(), ()> = ForceGraph::default();

    let nodes = 100;

    graph.add_force_node("0", ());
    for x in 1..nodes {
        graph.add_force_node(x.to_string(), ());
        graph.add_edge(x.into(), (x - 1).into(), ());
    }
    graph.add_edge(0.into(), (nodes - 1).into(), ());

    // let center = graph.add_force_node("", ());
    // for x in 0..nodes {
    //     graph.add_edge(x.into(), center, ());
    // }

    fdg_macroquad::run_window(&graph).await;
}
