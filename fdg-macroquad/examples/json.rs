#[macroquad::main("Force Graph JSON Demo")]
async fn main() {
    pretty_env_logger::init();

    let graph = fdg_sim::graph_from_json(include_str!("datasets/les_miserables.json")).unwrap();

    fdg_macroquad::run_window(&graph).await;
}
