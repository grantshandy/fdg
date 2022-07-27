use fdg_macroquad::JsonValue;
use fdg_sim::Simulation;

#[macroquad::main("Force Graph Empty Demo")]
async fn main() {
    pretty_env_logger::init();

    fdg_macroquad::run_window(&mut Simulation::<JsonValue, JsonValue>::default()).await;
}
