use fdg_sim::Simulation;

fn main() {
    pretty_env_logger::init();

    let mut sim = Simulation::<()>::default();

    for _ in 0..5 {
        sim.add_node("", ());
    }

    for _ in 0..5 {
        sim.update(0.01);
    }
}
