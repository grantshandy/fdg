use fdg_sim::CpuSimulation;

#[macroquad::main("Force Graph Empty Demo")]
async fn main() {
    pretty_env_logger::init();

    fdg_macroquad::run_window(&mut CpuSimulation::<()>::default()).await;

}
