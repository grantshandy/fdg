use std::{thread, time::Duration};

use fdg::{Dimensions, ForceGraph, ForceGraphHelper, Simulation, SimulationParameters};

fn main() {
    pretty_env_logger::init();

    // &str is the internal value for the node, this can be literally anything with Clone + Send (I think)
    let mut graph: ForceGraph<&str> = ForceGraph::default();

    // add high schools with associated data and connect skyline to olympus
    let _skyline = graph.add_force_node("Skyline", "Skyline Data");
    let _olympus = graph.add_force_node("Olympus", "Olympus Data");

    // ForceGraph Simulation
    let mut sim = Simulation::from_graph(graph, Dimensions::Two, SimulationParameters::default());
    let time_difference = 1;

    loop {
        // step through the simulation
        println!("Stepping!");
        sim.step(time_difference as f32);

        // sleep 1 sec
        thread::sleep(Duration::from_secs(time_difference));
    }
}
