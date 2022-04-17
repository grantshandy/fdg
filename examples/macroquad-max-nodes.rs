use fdg::{
    sim::{ForceGraph, ForceGraphHelper},
    Dimensions, Simulation, SimulationParameters,
};

use rand::Rng; 

#[macroquad::main("Force Graph Max Nodes Demo")]
async fn main() {
    pretty_env_logger::init();

    let mut graph: ForceGraph<()> = ForceGraph::default();

    for i in 0..75 {
        let node = graph.add_force_node(i.to_string(), ());
        let node = &mut graph[node];
        let mut rng = rand::thread_rng();
        
        node.mass = rng.gen_range(0.0..10.0);

    }

    let mut sim = Simulation::from_graph(graph, Dimensions::Two, SimulationParameters::default());

    fdg::macroquad::run_window(&mut sim).await;
}
