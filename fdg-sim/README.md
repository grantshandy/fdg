# fdg (Force Directed Graph)
A Force Directed Graph Framework for Rust. This manages your forces and event loop for a visualization of a graph. I've also created compatible visualizers for the simulation.

[![Latest version](https://img.shields.io/crates/v/fdg_sim.svg)](https://crates.io/crates/fdg_sim)
[![Documentation](https://docs.rs/fdg-sim/badge.svg)](https://docs.rs/fdg-sim)
[![GPL-3.0](https://img.shields.io/badge/license-GPL-blue.svg)](https://github.com/skylinecc/fdg/blob/main/LICENSE)

![example screenshot](https://raw.githubusercontent.com/grantshandy/fdg/main/fdg-macroquad/screenshots/screenshot-3D.png)

[View Demo Online](https://grantshandy.github.io/fdg)

## Basic Example
```rust
use fdg_sim::{ForceGraph, ForceGraphHelper, Simulation, SimulationParameters};

fn main() {
    // initialize a graph
    let mut graph: ForceGraph<(), ()> = ForceGraph::default();

    let one = graph.add_force_node("one", ());
    let two = graph.add_force_node("two", ());
    let _three = graph.add_force_node("three", ());
    graph.add_edge(one, two, ());

    // create a simulation from the graph
    let mut simulation = Simulation::from_graph(&graph, SimulationParameters::default());

    // your event/render loop
    for frame in 0..50 {
        // update the nodes positions based on force algorithm
        simulation.update(0.035);

        // render (print) your nodes new locations.
        println!("---- frame {frame} ----");
        for node in simulation.get_graph().node_weights() {
            println!("\"{}\" - {:?}", node.name, node.location);
        }
        println!("-----------------------")
    }
}
```

## Related Crates
- `/fdg-macroquad` A visualizer that uses `macroquad` for real-time rendering ([View Demo Online](https://grantshandy.github.io/fdg)).
- `/fdg-img` An SVG visualizer for the simulation.
