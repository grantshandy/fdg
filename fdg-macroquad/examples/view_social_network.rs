use fdg_sim::json;

#[macroquad::main("Force Graph Social Network Demo")]
async fn main() {
    let graph = json::graph_from_json(include_str!("../../datasets/social_network.json")).unwrap();

    fdg_macroquad::run_window(&graph).await;
}
