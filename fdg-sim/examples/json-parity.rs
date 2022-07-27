use fdg_sim::{ForceGraph, ForceGraphHelper};
use petgraph::visit::{EdgeRef, IntoEdgeReferences};

fn main() {
    let mut graph: ForceGraph<(), ()> = ForceGraph::default();

    let a = graph.add_force_node("a", ());
    let b = graph.add_force_node("b", ());
    let c = graph.add_force_node("c", ());

    graph.add_edge(a, b, ());
    graph.add_edge(b, c, ());
    graph.add_edge(c, a, ());

    let json = fdg_sim::json_from_graph(&graph).unwrap();

    println!("serialized:");
    println!("{json}");

    let graph = fdg_sim::graph_from_json(json).unwrap();

    println!("deserialized:");

    for node in graph.node_weights() {
        println!("{:?}", node.name);
    }

    println!("=========");

    for edge in graph.edge_references() {
        let source = &graph[edge.source()];
        let target = &graph[edge.target()];

        println!(
            "source: {}\ntarget: {}\ndata: {}\n",
            source.name,
            target.name,
            edge.weight()
        );
    }
}
