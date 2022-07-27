use fdg_sim::{Simulation, SimulationParameters};

#[macroquad::main("Force Graph JSON Demo")]
async fn main() {
    pretty_env_logger::init();

    let graph = fdg_sim::graph_from_json(include_str!("datasets/les_miserables.json")).unwrap();
    let mut sim = Simulation::from_graph(&graph, SimulationParameters::default());

    fdg_macroquad::run_window(&mut sim).await;
}