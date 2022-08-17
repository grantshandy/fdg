use fdg_sim::json;
use petgraph::visit::{EdgeRef, IntoEdgeReferences};

const SOCIAL_NETWORK: &'static str = include_str!("../../datasets/social_network.json");

fn main() {
    let graph = json::graph_from_json(SOCIAL_NETWORK).unwrap();

    println!("---- nodes ----");
    for node in graph.node_weights() {
        println!("name: {}, data: {}", node.name, node.data);
    }

    println!("---- edges ----");
    for edge in graph.edge_references() {
        println!(
            "source: {}, target: {}, data: {}",
            &graph[edge.source()].name,
            &graph[edge.target()].name,
            edge.weight()
        );
    }

    println!("---- output ----");
    println!(
        "{}",
        serde_json::to_string_pretty(&json::graph_to_json(&graph).unwrap()).unwrap()
    );
}
