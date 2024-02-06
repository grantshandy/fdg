use fdg::{fruchterman_reingold::FruchtermanReingold, Force, ForceGraph, simple::Center};

fn main() {
    let mut rng = rand::thread_rng();
    let dataset = petgraph_gen::barabasi_albert_graph(&mut rng, 10, 3, None);

    let mut graph: ForceGraph<f32, 2, (), ()> = fdg::init_force_graph_uniform(dataset, 10.0);

    FruchtermanReingold::default().apply_many(&mut graph, 100);
    Center::default().apply(&mut graph);

    println!("nodes:");
    for (_, pos) in graph.node_weights() {
        println!("{pos:?}");
    }

    println!("edges:");
    for edge_idx in graph.edge_indices() {
        let (source_idx, target_idx) = graph.edge_endpoints(edge_idx).unwrap();

        println!("{edge_idx:?}: {:?} to {:?}", &graph[source_idx].1, &graph[target_idx].1);
    }
}
