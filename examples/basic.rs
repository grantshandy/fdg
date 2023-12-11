use fdg_sim::{Force, ForceGraph, distributions::Uniform};

fn main() {
    let mut rng = rand::thread_rng();

    let mut force_graph: ForceGraph<f32, 2, (), ()> = fdg_sim::init_force_graph(
        petgraph_gen::barabasi_albert_graph(&mut rng, 100, 3, None),
        Uniform::new(-10.0, 10.0),
    );

    let mut force = fdg_sim::FruchtermanReingoldParallel::default();

    for _ in 1..100 {
        force.apply(&mut force_graph);
    }
}
