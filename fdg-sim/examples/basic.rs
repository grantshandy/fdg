use std::{thread, time::Duration};

use fdg_sim::{Dimensions, ForceGraph, ForceGraphHelper, Simulation};

fn main() {
    pretty_env_logger::init();

    // &str is the internal value for the node, this can be literally anything with Clone + Send (I think)
    let mut graph: ForceGraph<&str> = ForceGraph::default();

    // add high schools with associated data and connect skyline to olympus
    let _skyline = graph.add_force_node("Skyline", "Skyline Data");
    let _olympus = graph.add_force_node("Olympus", "Olympus Data");

    // ForceGraph Simulation
    let mut sim = Simulation::from_graph(graph, Dimensions::Two);

    loop {
        // step through the simulation
        println!("Stepping!");
        sim.step();

        // sleep 1 sec
        thread::sleep(Duration::from_secs(1));
    }
}
