# fdg (Force Directed Graph)
A Force Directed Graph Framework for Rust. This manages your forces and event loop for a visualization of a graph. I've also created compatible visualizers for it. This simulation sits on top of [`petgraph`](https://crates.io/crates/petgraph).

| Name            | Version                                                                                                        | Docs                                                                                       | License                                                                                                                             |
|-----------------|----------------------------------------------------------------------------------------------------------------|--------------------------------------------------------------------------------------------|-------------------------------------------------------------------------------------------------------------------------------------|
| `fdg-sim`       | [![Latest version](https://img.shields.io/crates/v/fdg-sim.svg)](https://crates.io/crates/fdg-sim)             | [![Documentation](https://docs.rs/fdg-sim/badge.svg)](https://docs.rs/fdg-sim)             | [![MIT](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/grantshandy/fdg/blob/main/fdg-sim/LICENSE)       |
| `fdg-macroquad` | [![Latest version](https://img.shields.io/crates/v/fdg-macroquad.svg)](https://crates.io/crates/fdg-macroquad) | [![Documentation](https://docs.rs/fdg-macroquad/badge.svg)](https://docs.rs/fdg-macroquad) | [![GPL-3.0](https://img.shields.io/badge/license-GPL-blue.svg)](https://github.com/grantshandy/fdg/blob/main/fdg-macroquad/LICENSE) |
| `fdg-img`       | [![Latest version](https://img.shields.io/crates/v/fdg-img.svg)](https://crates.io/crates/fdg-img)             | [![Documentation](https://docs.rs/fdg-img/badge.svg)](https://docs.rs/fdg-img)             | [![GPL-3.0](https://img.shields.io/badge/license-GPL-blue.svg)](https://github.com/grantshandy/fdg/blob/main/fdg-img/LICENSE)       |
| `fdg-wasm`      | [![NPM Package](https://img.shields.io/npm/v/fdg-wasm)](https://www.npmjs.com/package/fdg-wasm)                | [![View Readme](https://docs.rs/fdg-sim/badge.svg)](https://github.com/grantshandy/fdg/tree/main/fdg-wasm#forcegraphsimulation-documentation) | [![MIT](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/grantshandy/fdg/blob/main/fdg-sim/LICENSE)       |

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
- [`fdg-macroquad`](https://crates.io/crates/fdg-macroquad) A demo visualizer that uses [`macroquad`](https://crates.io/crates/macroquad) for real-time rendering ([View Demo Online](https://grantshandy.github.io/fdg)).
- [`fdg-img`](https://crates.io/crates/fdg-img) An SVG visualizer for the simulation.
