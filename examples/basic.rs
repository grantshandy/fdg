use fdg_sim::{Force, ForceGraph, FruchtermanReingold};
use petgraph::Graph;
use rand::distributions::Uniform;

fn main() {
    let mut graph = Graph::<&str, &str>::new();
    let pg = graph.add_node("petgraph");
    let fb = graph.add_node("fixedbitset");
    let qc = graph.add_node("quickcheck");
    let rand = graph.add_node("rand");
    let libc = graph.add_node("libc");
    graph.extend_with_edges(&[(pg, fb), (pg, qc), (qc, rand), (rand, libc), (qc, libc)]);

    let mut force_graph: ForceGraph<f32, 2, &str, &str> =
        fdg_sim::init_force_graph(graph, Uniform::new(-10.0, 10.0));
    let mut force = FruchtermanReingold::default();

    for _ in 1..100000 {
        force.apply(&mut force_graph);
    }
}
