use fdg_sim::{Simulation, SimulationParameters, force::Translate};

#[macroquad::main("Force Graph Translate Demo")]
async fn main() {
    pretty_env_logger::init();

    let graph = fdg_sim::graph_from_json(include_str!("datasets/les_miserables.json")).unwrap();
    let mut sim = Simulation::from_graph(&graph, SimulationParameters::from_force(Translate::default()));

    fdg_macroquad::run_window(&mut sim).await;
}
